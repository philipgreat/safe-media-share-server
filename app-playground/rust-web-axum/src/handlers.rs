
//! HTTP handlers: invite URL, IP ACL, Range/resume media streaming.

use std::net::SocketAddr;
use std::path::PathBuf;

use axum::body::Body;
use axum::extract::{ConnectInfo, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use serde_json::{json, Value};
use teaql_runtime::UserContext;
use teaql_web_integration_axum::{ContextProvider, TeaContext, WebResponse};
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::entity_ref::{
    decode_asset_ref, decode_asset_ref_for_session, encode_asset_ref, encode_asset_ref_for_session,
    CODEC_REQUIRED, INVALID_REF,
};
use crate::proxy::resolve_client_ip;
use crate::range::parse_range_header;
use crate::service::{self, InviteOutcome};
use crate::session::{
    attach_session_cookie, session_fp_from_cookie, session_fp_from_headers,
};
use crate::settings::join_external;

#[derive(Clone)]
pub struct AppState {
    pub media_root: PathBuf,
    /// Shared SQLite connection pool; each request gets a fully wired UserContext.
    pub pool: media_guard_service_core::DataServicePool,
}

impl ContextProvider for AppState {
    fn build_context(&self) -> UserContext {
        crate::build_request_context(&self.pool)
    }
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        // Frontend (React SPA)
        .route("/", get(spa_index))
        .route("/view/{asset_ref}", get(spa_index))
        .route("/admin", get(spa_index))
        .route("/admin/", get(spa_index))
        .route("/admin/{*rest}", get(spa_index))
        .route("/assets/{*name}", get(spa_asset))
        .route("/vite.svg", get(spa_named))
        .route("/static/{*name}", get(spa_named))
        // JSON API for the UI
        .route("/api/media", get(api_list_media))
        .route("/api/media/{asset_ref}", get(api_media_detail))
        // Admin console (requires X-Admin-Key / MEDIA_GUARD_ADMIN_KEY)
        .route("/admin/api/ping", get(admin_api_ping))
        .route("/admin/api/media", get(admin_api_media))
        .route("/admin/api/whitelist", get(admin_api_whitelist))
        .route("/admin/api/spent-invites", get(admin_api_spent))
        .route("/admin/api/invites", get(admin_api_create_invite).post(admin_api_create_invite))
        .route("/admin/api/settings", get(admin_api_get_settings).post(admin_api_set_settings))
        .route("/admin/api/media", post(admin_api_upload_media))
        // Specific URL for invite redemption — not exposed elsewhere.
        .route("/invite/{token}", get(redeem_invite_url))
        // Durable cross-session share entry: mints a session-bound media ref.
        .route("/open/{handle_token}", get(open_share_handle))
        .route("/media/{asset_ref}", get(serve_media))
        .route("/media/{asset_ref}/download", get(download_media))
        .route("/media/{asset_id}/grant/{client_ip}", get(admin_grant))
        .route("/admin/invites", get(admin_create_invite))
        .route("/admin/assets", get(admin_create_asset))
        .with_state(state)
}

async fn health() -> WebResponse<Value> {
    WebResponse::of_single(json!({
        "status": "ok",
        "service": "media-guard-service-core-web-axum"
    }))
}

fn web_dist_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web/dist")
}

fn mime_for(path: &std::path::Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .as_deref()
    {
        Some("html") => "text/html; charset=utf-8",
        Some("js") | Some("mjs") => "application/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        Some("map") => "application/json",
        _ => "application/octet-stream",
    }
}

fn serve_static_file(rel: &str) -> Response {
    // Prevent path traversal.
    if rel.contains("..") {
        return (StatusCode::BAD_REQUEST, "bad path").into_response();
    }
    let root = web_dist_root();
    let path = root.join(rel);
    if path.is_file() {
        match std::fs::read(&path) {
            Ok(bytes) => {
                return Response::builder()
                    .header(axum::http::header::CONTENT_TYPE, mime_for(&path))
                    .body(Body::from(bytes))
                    .unwrap()
            }
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "read failed").into_response()
            }
        }
    }
    // SPA fallback
    let index = root.join("index.html");
    match std::fs::read(&index) {
        Ok(bytes) => html_response_bytes(bytes),
        Err(_) => (
            StatusCode::NOT_FOUND,
            "frontend not built: run `cd web && npm run build`",
        )
            .into_response(),
    }
}

fn html_response_bytes(bytes: Vec<u8>) -> Response {
    Response::builder()
        .header(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(bytes))
        .unwrap()
}

/// SPA shell for React routes (`/`, `/view/:id`, `/admin/*`).
async fn spa_index() -> Response {
    serve_static_file("index.html")
}

async fn spa_asset(Path(name): Path<String>) -> Response {
    serve_static_file(&format!("assets/{name}"))
}

async fn spa_named(Path(name): Path<String>) -> Response {
    serve_static_file(&name)
}

fn admin_key_configured() -> Option<String> {
    std::env::var("MEDIA_GUARD_ADMIN_KEY")
        .ok()
        .filter(|s| !s.trim().is_empty())
}

fn check_admin_key(headers: &HeaderMap) -> Result<(), Response> {
    let Some(expected) = admin_key_configured() else {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            WebResponse::<Value>::fail(
                "MEDIA_GUARD_ADMIN_KEY is not set; super invite/admin key is disabled",
            ),
        )
            .into_response());
    };
    let provided = headers
        .get("x-admin-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if provided.trim() == expected.trim() {
        return Ok(());
    }
    Err((
        StatusCode::UNAUTHORIZED,
        WebResponse::<Value>::fail("invalid admin key"),
    )
        .into_response())
}

async fn admin_api_ping(headers: HeaderMap) -> Response {
    if let Err(resp) = check_admin_key(&headers) {
        return resp;
    }
    WebResponse::of_single(json!({ "ok": true, "role": "admin" })).into_response()
}

async fn admin_api_media(
    State(_state): State<AppState>,
    TeaContext(context): TeaContext,
    headers: HeaderMap,
) -> Response {
    if let Err(resp) = check_admin_key(&headers) {
        return resp;
    }
    let base = service::external_base_url(&context).await.unwrap_or_default();
    match service::list_active_assets(&context).await {
        Ok(items) => {
            let fresh = service::list_fresh_invites(&context).await.unwrap_or_default();
            let mut by_asset: std::collections::HashMap<u64, String> = std::collections::HashMap::new();
            for (_id, token, asset_id) in fresh {
                by_asset.entry(asset_id).or_insert(token);
            }
            let handles = service::list_active_share_handles(&context).await.unwrap_or_default();
            let mut share_by_asset: std::collections::HashMap<u64, String> =
                std::collections::HashMap::new();
            for (asset_id, token) in handles {
                share_by_asset.entry(asset_id).or_insert(token);
            }
            let data: Vec<Value> = items
                .iter()
                .map(|v| {
                    let token = by_asset.get(&v.id).cloned().unwrap_or_default();
                    let invite_url = if token.is_empty() {
                        String::new()
                    } else {
                        join_external(&base, &format!("/invite/{token}"))
                    };
                    // Admin may keep the internal id for diagnostics, but share
                    // links must embed the encrypted guest ref.
                    let asset_ref = encode_asset_ref(&context, v.id).unwrap_or_default();
                    let share_token = share_by_asset.get(&v.id).cloned().unwrap_or_default();
                    let share_url = if share_token.is_empty() {
                        String::new()
                    } else {
                        join_external(&base, &format!("/open/{share_token}"))
                    };
                    json!({
                        "id": v.id,
                        "ref": asset_ref,
                        "displayName": v.display_name,
                        "filePath": v.file_path,
                        "contentType": service::detect_content_type(&v.file_path, &v.content_type),
                        "fileSize": v.file_size,
                        "inviteToken": token,
                        "inviteUrl": invite_url,
                        "shareUrl": share_url,
                        "viewUrl": join_external(&base, &format!("/view/{asset_ref}")),
                        "downloadUrl": join_external(&base, &format!("/media/{asset_ref}/download")),
                        "mediaUrl": join_external(&base, &format!("/media/{asset_ref}")),
                    })
                })
                .collect();
            WebResponse::of_list(data).into_response()
        }
        Err(err) => WebResponse::<Value>::fail(err.to_string()).into_response(),
    }
}

async fn admin_api_get_settings(
    TeaContext(context): TeaContext,
    headers: HeaderMap,
) -> Response {
    if let Err(resp) = check_admin_key(&headers) {
        return resp;
    }
    match service::list_settings(&context).await {
        Ok(rows) => {
            let data: Vec<Value> = rows
                .iter()
                .map(|(key, value, name, note)| {
                    json!({
                        "settingKey": key,
                        "settingValue": value,
                        "settingName": name,
                        "settingNote": note,
                        // compat with existing UI
                        "externalBaseUrl": if key == service::SETTING_EXTERNAL_BASE_URL {
                            value.clone()
                        } else {
                            String::new()
                        },
                    })
                })
                .collect();
            WebResponse::of_list(data).into_response()
        }
        Err(err) => WebResponse::<Value>::fail(err.to_string()).into_response(),
    }
}

async fn admin_api_set_settings(
    TeaContext(context): TeaContext,
    headers: HeaderMap,
    axum::Json(body): axum::Json<Value>,
) -> Response {
    if let Err(resp) = check_admin_key(&headers) {
        return resp;
    }
    let value = body
        .get("externalBaseUrl")
        .or_else(|| body.get("settingValue"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let name = body
        .get("settingName")
        .and_then(|v| v.as_str())
        .unwrap_or("外部访问 URL 前缀");
    let note = body
        .get("settingNote")
        .and_then(|v| v.as_str())
        .unwrap_or("复制邀请链接与打开链接时使用的对外地址前缀");

    // Resolve library id (first row).
    let libs = media_guard_service_core::Q::media_libraries()
        .order_by_id_asc()
        .limit(1)
        .comment("what: load media library root")
        .purpose("why: store application setting under domain root")
        .execute_for_list(&context)
        .await;
    let library_id = match libs {
        Ok(rows) => rows
            .iter()
            .next()
            .and_then(|l| media_guard_service_core::E::media_library(l).get_id().eval())
            .unwrap_or(0),
        Err(err) => return WebResponse::<Value>::fail(err.to_string()).into_response(),
    };
    if library_id == 0 {
        return WebResponse::<Value>::fail("no media library root").into_response();
    }

    match service::upsert_setting(
        &context,
        library_id,
        service::SETTING_EXTERNAL_BASE_URL,
        value,
        name,
        note,
    )
    .await
    {
        Ok(()) => {
            let saved = service::external_base_url(&context).await.unwrap_or_default();
            WebResponse::of_single(json!({
                "externalBaseUrl": saved,
                "settingKey": service::SETTING_EXTERNAL_BASE_URL,
                "settingValue": saved,
                "message": "settings saved to database",
            }))
            .into_response()
        }
        Err(err) => WebResponse::<Value>::fail(err.to_string()).into_response(),
    }
}

async fn admin_api_whitelist(
    TeaContext(context): TeaContext,
    headers: HeaderMap,
) -> Response {
    if let Err(resp) = check_admin_key(&headers) {
        return resp;
    }
    match service::list_current_whitelist(&context).await {
        Ok(rows) => {
            let data: Vec<Value> = rows
                .iter()
                .map(|r| {
                    json!({
                        "grantId": r.grant_id,
                        "clientIp": r.client_ip,
                        "expireAt": r.expire_at,
                        "grantNote": r.grant_note,
                        "mediaAssetId": r.media_asset_id,
                        "mediaName": r.media_name,
                        "mediaFile": r.media_file,
                    })
                })
                .collect();
            WebResponse::of_list(data).into_response()
        }
        Err(err) => WebResponse::<Value>::fail(err.to_string()).into_response(),
    }
}

async fn admin_api_spent(
    TeaContext(context): TeaContext,
    headers: HeaderMap,
) -> Response {
    if let Err(resp) = check_admin_key(&headers) {
        return resp;
    }
    match service::list_spent_invites(&context).await {
        Ok(rows) => {
            let data: Vec<Value> = rows
                .iter()
                .map(|r| {
                    json!({
                        "inviteId": r.invite_id,
                        "inviteToken": r.invite_token,
                        "firstClientIp": r.first_client_ip,
                        "firstClickAt": r.first_click_at,
                        "secondClientIp": r.second_client_ip,
                        "secondClickAt": r.second_click_at,
                        "mediaAssetId": r.media_asset_id,
                        "mediaName": r.media_name,
                        "mediaFile": r.media_file,
                        "grantNote": r.grant_note,
                    })
                })
                .collect();
            WebResponse::of_list(data).into_response()
        }
        Err(err) => WebResponse::<Value>::fail(err.to_string()).into_response(),
    }
}

/// GET/POST /admin/api/invites?asset_id=&note= — create invite (admin).
async fn admin_api_create_invite(
    State(_state): State<AppState>,
    TeaContext(context): TeaContext,
    headers: HeaderMap,
    axum::extract::Query(params): axum::extract::Query<
        std::collections::HashMap<String, String>,
    >,
) -> Response {
    if let Err(resp) = check_admin_key(&headers) {
        return resp;
    }
    let Some(asset_id) = params.get("asset_id").and_then(|s| s.parse::<u64>().ok()) else {
        return WebResponse::<Value>::fail("asset_id query param required").into_response();
    };
    let note = params
        .get("note")
        .cloned()
        .unwrap_or_else(|| "invite created via admin console".to_owned());
    let asset_lookup = service::load_active_asset(&context, asset_id).await;
    let asset_opt = match asset_lookup {
        Ok(v) => v,
        Err(err) => return WebResponse::<Value>::fail(err.to_string()).into_response(),
    };
    let Some(asset) = asset_opt else {
        return WebResponse::<Value>::fail("asset not found").into_response();
    };
    let view = service::asset_view(&asset);
    match service::create_invite(&context, view.library_id, asset_id, &note).await {
        Ok(invite) => {
            let id = media_guard_service_core::E::invite_code(&invite)
                .get_id()
                .eval()
                .unwrap_or(0);
            let token = media_guard_service_core::E::invite_code(&invite)
                .get_invite_token()
                .eval()
                .unwrap_or_default();
            let base = service::external_base_url(&context).await.unwrap_or_default();
            let url = join_external(&base, &format!("/invite/{token}"));
            WebResponse::of_single(json!({
                "inviteId": id,
                "inviteToken": token,
                "url": url,
                "path": format!("/invite/{token}"),
                "mediaAssetId": asset_id,
                "note": note,
            }))
            .into_response()
        }
        Err(err) => WebResponse::<Value>::fail(err.to_string()).into_response(),
    }
}

/// Guest-facing media JSON: encrypted `ref` only, never the raw numeric id.
///
/// Refs are minted against the caller's session fingerprint (empty fp keeps the
/// bare `media-access` purpose for cookie-less legacy clients).
fn guest_asset_json(
    context: &UserContext,
    view: &service::AssetView,
    authorized: bool,
    session_fp: &str,
) -> Value {
    let asset_ref = encode_asset_ref_for_session(context, view.id, session_fp).unwrap_or_default();
    json!({
        "ref": asset_ref,
        "displayName": view.display_name,
        "filePath": view.file_path,
        "contentType": service::detect_content_type(&view.file_path, &view.content_type),
        "fileSize": view.file_size,
        "authorized": authorized,
    })
}

/// GET /api/media — browse list (metadata is visible; playback still needs whitelist).
async fn api_list_media(
    TeaContext(context): TeaContext,
    headers: HeaderMap,
) -> Response {
    let session_fp = session_fp_from_cookie(&headers);
    match service::list_active_assets(&context).await {
        Ok(items) => {
            let data: Vec<Value> = items
                .iter()
                .map(|v| guest_asset_json(&context, v, false, &session_fp))
                .collect();
            WebResponse::of_list(data).into_response()
        }
        Err(err) => WebResponse::<Value>::fail(err.to_string()).into_response(),
    }
}

/// GET /api/media/{asset_ref}
async fn api_media_detail(
    TeaContext(context): TeaContext,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    Path(asset_ref): Path<String>,
    headers: HeaderMap,
) -> Response {
    let session_fp = session_fp_from_cookie(&headers);
    let asset_id = match decode_media_ref(&context, &asset_ref, &session_fp).await {
        Ok(id) => id,
        Err(err) => return invalid_media_ref_response(err),
    };
    let client_ip = match resolve_ip(&context, peer, &headers).await {
        Ok(ip) => ip,
        Err(err) => return WebResponse::<Value>::fail(err).into_response(),
    };
    let Ok(Some(asset)) = service::load_active_asset(&context, asset_id).await else {
        return (
            StatusCode::NOT_FOUND,
            WebResponse::<Value>::fail("asset not found"),
        )
            .into_response();
    };
    let view = service::asset_view(&asset);
    let authorized = service::is_ip_allowed(&context, asset_id, &client_ip.to_string())
        .await
        .unwrap_or(false);
    WebResponse::of_single(guest_asset_json(&context, &view, authorized, &session_fp)).into_response()
}

/// 400 for bad guest refs. Plain-text body so error envelopes never echo ids.
fn invalid_media_ref_response(err: String) -> Response {
    let message = if err == CODEC_REQUIRED {
        err
    } else {
        INVALID_REF.to_owned()
    };
    (StatusCode::BAD_REQUEST, message).into_response()
}

/// Decode a media ref for this request ([S5]).
///
/// Fingerprint-bound purpose first (`media-access:{fp}`); bare `media-access`
/// remains readable so pre-session refs keep working when no matching cookie
/// is presented. A non-empty fingerprint must map to a live DB session — an
/// unknown or revoked cookie rejects the ref entirely.
async fn decode_media_ref(
    context: &UserContext,
    token: &str,
    session_fp: &str,
) -> Result<u64, String> {
    if !session_fp.is_empty() {
        let active = service::is_session_active(context, session_fp)
            .await
            .map_err(|_| INVALID_REF.to_owned())?;
        if !active {
            return Err(INVALID_REF.to_owned());
        }
        if let Ok(id) = decode_asset_ref_for_session(context, token, session_fp) {
            return Ok(id);
        }
    }
    decode_asset_ref(context, token)
}

/// Resolve the real client IP: trusted-proxy CIDR + forwarded headers.
pub(crate) async fn resolve_ip(
    context: &UserContext,
    peer: SocketAddr,
    headers: &HeaderMap,
) -> Result<std::net::IpAddr, String> {
    let rules = service::load_proxy_trust_configs(context)
        .await
        .map_err(|e| e.to_string())?;
    Ok(resolve_client_ip(peer, headers, &rules))
}

/// GET /invite/{token}
///
/// Dedicated one-shot invite URL.
/// 1st click joins the IP whitelist; 2nd click expires that IP from the
/// whitelist; later clicks are rejected (code cannot be reused).
#[axum::debug_handler]
async fn redeem_invite_url(
    State(_state): State<AppState>,
    TeaContext(context): TeaContext,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    Path(token): Path<String>,
    headers: HeaderMap,
) -> Response {
    let client_ip = match resolve_ip(&context, peer, &headers).await {
        Ok(ip) => ip,
        Err(err) => {
            return WebResponse::<Value>::fail(err.to_string()).into_response();
        }
    };
    let wants_html = headers
        .get(axum::http::header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.contains("text/html"))
        .unwrap_or(false);
    // Invite mint prefers session-bound refs + cookie ([S5]); the ref is for
    // immediate UX only, never a durable bookmark.
    let (session_fp, new_cookie) = session_fp_from_headers(&headers);

    match service::redeem_invite(&context, &token, &client_ip.to_string()).await {
        Ok(InviteOutcome::Joined {
            library_id,
            media_asset_id,
            client_ip,
            expire_at,
            ..
        }) => {
            if let Err(err) = service::ensure_user_session(&context, library_id, &session_fp).await {
                return WebResponse::<Value>::fail(err.to_string()).into_response();
            }
            let media_ref = match encode_asset_ref_for_session(&context, media_asset_id, &session_fp)
            {
                Ok(r) => r,
                Err(err) => return WebResponse::<Value>::fail(err).into_response(),
            };
            if wants_html {
                // Browser: jump straight into the media view.
                let mut builder = Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(
                        axum::http::header::LOCATION,
                        format!("/view/{media_ref}"),
                    );
                if let Some(cookie) = &new_cookie {
                    builder = builder.header(axum::http::header::SET_COOKIE, cookie.as_str());
                }
                return builder.body(Body::empty()).unwrap();
            }
            attach_session_cookie(
                WebResponse::of_single(json!({
                    "action": "join_whitelist",
                    "mediaRef": media_ref,
                    "clientIp": client_ip,
                    "expireAt": expire_at,
                    "message": "First click: IP added to whitelist"
                }))
                .into_response(),
                new_cookie,
            )
        }
        Ok(InviteOutcome::ExpiredFromWhitelist {
            library_id,
            media_asset_id,
            client_ip,
            revoked_grants,
            ..
        }) => {
            if let Err(err) = service::ensure_user_session(&context, library_id, &session_fp).await {
                return WebResponse::<Value>::fail(err.to_string()).into_response();
            }
            let media_ref = match encode_asset_ref_for_session(&context, media_asset_id, &session_fp)
            {
                Ok(r) => r,
                Err(err) => return WebResponse::<Value>::fail(err).into_response(),
            };
            if wants_html {
                let mut builder = Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(
                        axum::http::header::LOCATION,
                        format!("/?expired=1&asset={media_ref}"),
                    );
                if let Some(cookie) = &new_cookie {
                    builder = builder.header(axum::http::header::SET_COOKIE, cookie.as_str());
                }
                return builder.body(Body::empty()).unwrap();
            }
            attach_session_cookie(
                WebResponse::of_single(json!({
                    "action": "expire_whitelist",
                    "mediaRef": media_ref,
                    "clientIp": client_ip,
                    "revokedGrants": revoked_grants,
                    "message": "Second click: IP expired from whitelist"
                }))
                .into_response(),
                new_cookie,
            )
        }
        Ok(InviteOutcome::AlreadySpent { .. }) => {
            if wants_html {
                return Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(axum::http::header::LOCATION, "/?spent=1")
                    .body(Body::empty())
                    .unwrap();
            }
            (
                StatusCode::GONE,
                WebResponse::<Value>::fail(
                    "invite already spent; expired codes cannot be used again".to_owned(),
                ),
            )
                .into_response()
        }
        Err(err) => {
            if wants_html {
                return Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(axum::http::header::LOCATION, "/?invalid=1")
                    .body(Body::empty())
                    .unwrap();
            }
            (
                StatusCode::NOT_FOUND,
                WebResponse::<Value>::fail(format!("invite error: {err}")),
            )
                .into_response()
        }
    }
}

/// GET /open/{handle_token} — durable share entry ([S4]).
///
/// Resolves the handle, enforces the IP whitelist, then mints a session-bound
/// media ref and redirects (HTML) or returns `{mediaRef}` (JSON). The redirect
/// target never contains the `SHD-` handle or a raw asset id.
#[axum::debug_handler]
async fn open_share_handle(
    State(_state): State<AppState>,
    TeaContext(context): TeaContext,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    Path(handle_token): Path<String>,
    headers: HeaderMap,
) -> Response {
    let wants_html = headers
        .get(axum::http::header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.contains("text/html"))
        .unwrap_or(false);

    let loaded = service::load_share_handle_by_token(&context, &handle_token).await;
    let handle = match loaded {
        Ok(Some(handle)) => handle,
        Ok(None) => {
            return open_error_response(wants_html, StatusCode::NOT_FOUND, "share link not found");
        }
        Err(err) => {
            return WebResponse::<Value>::fail(err.to_string()).into_response();
        }
    };

    let active = media_guard_service_core::E::share_handle(&handle)
        .get_active()
        .eval()
        .unwrap_or(false);
    if !active {
        return open_error_response(
            wants_html,
            StatusCode::GONE,
            "share link is no longer active",
        );
    }
    let asset_id = media_guard_service_core::E::share_handle(&handle)
        .get_media_asset_id()
        .eval()
        .unwrap_or(0);

    let client_ip = match resolve_ip(&context, peer, &headers).await {
        Ok(ip) => ip,
        Err(err) => return WebResponse::<Value>::fail(err).into_response(),
    };
    let allowed = service::is_ip_allowed(&context, asset_id, &client_ip.to_string())
        .await
        .unwrap_or(false);
    if !allowed {
        // No asset id / handle in the bounce — guest needs an invite ([S7]).
        if wants_html {
            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(axum::http::header::LOCATION, "/?need_invite=1")
                .body(Body::empty())
                .unwrap();
        }
        return (
            StatusCode::FORBIDDEN,
            WebResponse::<Value>::fail(
                "this IP is not authorized for the shared media; use an invite link",
            ),
        )
            .into_response();
    }

    let (session_fp, new_cookie) = session_fp_from_headers(&headers);
    let library_id = media_guard_service_core::E::share_handle(&handle)
        .get_media_library_id()
        .eval()
        .unwrap_or(0);
    if let Err(err) = service::ensure_user_session(&context, library_id, &session_fp).await {
        return WebResponse::<Value>::fail(err.to_string()).into_response();
    }
    let media_ref = match encode_asset_ref_for_session(&context, asset_id, &session_fp) {
        Ok(r) => r,
        Err(err) => return WebResponse::<Value>::fail(err).into_response(),
    };

    if wants_html {
        let mut builder = Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(axum::http::header::LOCATION, format!("/view/{media_ref}"));
        if let Some(cookie) = &new_cookie {
            builder = builder.header(axum::http::header::SET_COOKIE, cookie.as_str());
        }
        return builder.body(Body::empty()).unwrap();
    }
    attach_session_cookie(
        WebResponse::of_single(json!({ "mediaRef": media_ref })).into_response(),
        new_cookie,
    )
}

/// Opaque 404/410 for share handles — never echo ids or tokens ([S7]).
fn open_error_response(wants_html: bool, status: StatusCode, message: &str) -> Response {
    if wants_html {
        (status, message.to_owned()).into_response()
    } else {
        (status, WebResponse::<Value>::fail(message.to_owned())).into_response()
    }
}

/// GET /admin/invites?asset_id=1&note=demo
async fn admin_create_invite(
    State(_state): State<AppState>,
    TeaContext(context): TeaContext,
    headers: HeaderMap,
    axum::extract::Query(params): axum::extract::Query<
        std::collections::HashMap<String, String>,
    >,
) -> Response {
    if let Err(resp) = check_admin_key(&headers) {
        return resp;
    }
    let asset_id: u64 = match params.get("asset_id").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => {
            return WebResponse::<Value>::fail("asset_id query param required").into_response()
        }
    };
    let note = params
        .get("note")
        .cloned()
        .unwrap_or_else(|| "invite created via admin endpoint".to_owned());
    let asset_lookup = service::load_active_asset(&context, asset_id).await;
    let asset_opt = match asset_lookup {
        Ok(v) => v,
        Err(err) => return WebResponse::<Value>::fail(err.to_string()).into_response(),
    };
    let Some(asset) = asset_opt else {
        return WebResponse::<Value>::fail("asset not found").into_response();
    };
    let view = service::asset_view(&asset);
    match service::create_invite(&context, view.library_id, asset_id, &note).await {
        Ok(invite) => {
            let id = media_guard_service_core::E::invite_code(&invite)
                .get_id()
                .eval()
                .unwrap_or(0);
            let token = media_guard_service_core::E::invite_code(&invite)
                .get_invite_token()
                .eval()
                .unwrap_or_default();
            WebResponse::of_single(json!({
                "inviteId": id,
                "inviteToken": token,
                "url": format!("/invite/{token}"),
                "mediaAssetId": asset_id,
                "note": note,
            }))
            .into_response()
        }
        Err(err) => WebResponse::<Value>::fail(err.to_string()).into_response(),
    }
}

/// GET /media/{asset_ref} — IP-whitelist gate + Range/resume streaming.
async fn serve_media(
    State(state): State<AppState>,
    TeaContext(context): TeaContext,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    Path(asset_ref): Path<String>,
    headers: HeaderMap,
) -> Response {
    let session_fp = session_fp_from_cookie(&headers);
    let asset_id = match decode_media_ref(&context, &asset_ref, &session_fp).await {
        Ok(id) => id,
        Err(err) => return invalid_media_ref_response(err),
    };
    let client_ip = match resolve_ip(&context, peer, &headers).await {
        Ok(ip) => ip,
        Err(err) => return WebResponse::<Value>::fail(err).into_response(),
    };

    let allowed = match service::is_ip_allowed(&context, asset_id, &client_ip.to_string()).await {
        Ok(v) => v,
        Err(err) => return WebResponse::<Value>::fail(err.to_string()).into_response(),
    };
    if !allowed {
        return (
            StatusCode::FORBIDDEN,
            WebResponse::<Value>::fail(format!(
                "IP {client_ip} is not authorized for this media"
            )),
        )
            .into_response();
    }

    let asset_lookup = service::load_active_asset(&context, asset_id).await;
    let asset_opt = match asset_lookup {
        Ok(v) => v,
        Err(err) => return WebResponse::<Value>::fail(err.to_string()).into_response(),
    };
    let Some(asset) = asset_opt else {
        return (
            StatusCode::NOT_FOUND,
            WebResponse::<Value>::fail("asset not found"),
        )
            .into_response();
    };

    let view = service::asset_view(&asset);
    let fs_path = state.media_root.join(&view.file_path);
    let file = match tokio::fs::File::open(&fs_path).await {
        Ok(f) => f,
        Err(_) => {
            return (StatusCode::NOT_FOUND, WebResponse::<Value>::fail("media file missing")).into_response()
        }
    };
    let meta = match file.metadata().await {
        Ok(m) => m,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                WebResponse::<Value>::fail("stat failed"),
            )
                .into_response()
        }
    };
    let total_len = meta.len();
    let content_type = service::detect_content_type(&view.file_path, &view.content_type);
    let filename = service::download_filename(&view.display_name, &view.file_path);

    let range = match parse_range_header(
        headers.get(axum::http::header::RANGE).and_then(|v| v.to_str().ok()),
        total_len,
    ) {
        Ok(r) => r,
        Err(()) => {
            return Response::builder()
                .status(StatusCode::RANGE_NOT_SATISFIABLE)
                .header(axum::http::header::CONTENT_RANGE, format!("bytes */{total_len}"))
                .body(Body::empty())
                .unwrap()
        }
    };

    if let Some(range) = range {
        let mut file = file;
        if file.seek(std::io::SeekFrom::Start(range.start)).await.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                WebResponse::<Value>::fail("seek failed"),
            )
                .into_response();
        }
        let take = range.len() as usize;
        let mut buf = vec![0u8; take];
        if file.read_exact(&mut buf).await.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                WebResponse::<Value>::fail("read failed"),
            )
                .into_response();
        }
        return Response::builder()
            .status(StatusCode::PARTIAL_CONTENT)
            .header(axum::http::header::ACCEPT_RANGES, "bytes")
            .header(axum::http::header::CONTENT_TYPE, content_type)
            .header(axum::http::header::CONTENT_RANGE, range.content_range_header(total_len))
            .header(axum::http::header::CONTENT_LENGTH, take)
            .header("X-Client-IP", client_ip.to_string())
            .body(Body::from(buf))
            .unwrap();
    }

    let mut file = file;
    let mut buf = Vec::with_capacity(total_len as usize);
    if file.read_to_end(&mut buf).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            WebResponse::<Value>::fail("read failed"),
        )
            .into_response();
    }
    let disposition = if service::prefers_download(&view.file_path, &view.content_type) {
        format!("attachment; filename=\"{filename}\"")
    } else {
        format!("inline; filename=\"{filename}\"")
    };
    Response::builder()
        .status(StatusCode::OK)
        .header(axum::http::header::ACCEPT_RANGES, "bytes")
        .header(axum::http::header::CONTENT_TYPE, content_type)
        .header(axum::http::header::CONTENT_LENGTH, total_len)
        .header(axum::http::header::CONTENT_DISPOSITION, disposition)
        .header("X-Client-IP", client_ip.to_string())
        .body(Body::from(buf))
        .unwrap()
}

/// GET /media/{asset_ref}/download — always force attachment (docs, zip, any file).
async fn download_media(
    State(state): State<AppState>,
    TeaContext(context): TeaContext,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    Path(asset_ref): Path<String>,
    headers: HeaderMap,
) -> Response {
    let session_fp = session_fp_from_cookie(&headers);
    let asset_id = match decode_media_ref(&context, &asset_ref, &session_fp).await {
        Ok(id) => id,
        Err(err) => return invalid_media_ref_response(err),
    };
    let client_ip = match resolve_ip(&context, peer, &headers).await {
        Ok(ip) => ip,
        Err(err) => return WebResponse::<Value>::fail(err).into_response(),
    };
    let allowed = service::is_ip_allowed(&context, asset_id, &client_ip.to_string())
        .await
        .unwrap_or(false);
    if !allowed {
        return (
            StatusCode::FORBIDDEN,
            WebResponse::<Value>::fail(format!(
                "IP {client_ip} is not authorized for this media"
            )),
        )
            .into_response();
    }
    let Ok(Some(asset)) = service::load_active_asset(&context, asset_id).await else {
        return (
            StatusCode::NOT_FOUND,
            WebResponse::<Value>::fail("asset not found"),
        )
            .into_response();
    };
    let view = service::asset_view(&asset);
    let fs_path = state.media_root.join(&view.file_path);
    let Ok(file) = tokio::fs::File::open(&fs_path).await else {
        return (StatusCode::NOT_FOUND, WebResponse::<Value>::fail("media file missing")).into_response();
    };
    let Ok(meta) = file.metadata().await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            WebResponse::<Value>::fail("stat failed"),
        )
            .into_response();
    };
    let total_len = meta.len();
    let filename = service::download_filename(&view.display_name, &view.file_path);
    let mut buf = Vec::with_capacity(total_len as usize);
    let mut file = file;
    if file.read_to_end(&mut buf).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            WebResponse::<Value>::fail("read failed"),
        )
            .into_response();
    }
    Response::builder()
        .status(StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, "application/octet-stream")
        .header(axum::http::header::CONTENT_LENGTH, total_len)
        .header(
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        )
        .body(Body::from(buf))
        .unwrap()
}

/// GET /media/{asset_id}/grant/{client_ip} — admin smoke helper.
async fn admin_grant(
    TeaContext(context): TeaContext,
    headers: HeaderMap,
    Path((asset_id, client_ip)): Path<(u64, String)>,
) -> Response {
    if let Err(resp) = check_admin_key(&headers) {
        return resp;
    }
    let asset_lookup = service::load_active_asset(&context, asset_id).await;
    let asset_opt = match asset_lookup {
        Ok(v) => v,
        Err(err) => return WebResponse::<Value>::fail(err.to_string()).into_response(),
    };
    let Some(asset) = asset_opt else {
        return (
            StatusCode::NOT_FOUND,
            WebResponse::<Value>::fail("asset not found"),
        )
            .into_response();
    };
    let view = service::asset_view(&asset);
    match service::grant_ip_access(
        &context,
        view.library_id,
        asset_id,
        &client_ip,
        None,
        "Granted via admin smoke endpoint",
    )
    .await
    {
        Ok(grant) => {
            let expire = media_guard_service_core::E::ip_access_grant(&grant)
                .get_expire_at()
                .eval()
                .unwrap_or_default();
            let id = media_guard_service_core::E::ip_access_grant(&grant)
                .get_id()
                .eval()
                .unwrap_or(0);
            WebResponse::of_single(json!({
                "grantId": id,
                "clientIp": client_ip,
                "expireAt": expire
            }))
            .into_response()
        }
        Err(err) => WebResponse::<Value>::fail(err.to_string()).into_response(),
    }
}

/// POST /admin/api/media — upload a file and register it as a media asset.
async fn admin_api_upload_media(
    State(state): State<AppState>,
    TeaContext(context): TeaContext,
    headers: HeaderMap,
    mut multipart: axum::extract::Multipart,
) -> Response {
    if let Err(resp) = check_admin_key(&headers) {
        return resp;
    }

    let mut name = String::new();
    let mut file_path = String::new();
    let mut bytes: Option<Vec<u8>> = None;
    let mut original_name = String::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("").to_owned();
        match field_name.as_str() {
            "name" => {
                name = field.text().await.unwrap_or_default().trim().to_owned();
            }
            "file_path" => {
                file_path = field.text().await.unwrap_or_default().trim().to_owned();
            }
            "file" | "media" => {
                if let Some(f) = field.file_name() {
                    original_name = f.to_owned();
                }
                bytes = field.bytes().await.ok().map(|b| b.to_vec());
            }
            _ => {}
        }
    }

    let Some(bytes) = bytes else {
        return WebResponse::<Value>::fail("multipart field 'file' is required").into_response();
    };

    // Sanitize destination path under media root.
    let dest_rel = if !file_path.is_empty() {
        file_path
            .trim_start_matches('/')
            .replace("..", "")
            .replace('\\', "/")
    } else {
        let base = if original_name.is_empty() {
            "upload.bin".to_owned()
        } else {
            std::path::Path::new(&original_name)
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "upload.bin".to_owned())
        };
        format!("uploads/{base}")
    };

    if dest_rel.is_empty() || dest_rel.contains("..") {
        return WebResponse::<Value>::fail("invalid file path").into_response();
    }

    let dest = state.media_root.join(&dest_rel);
    if let Some(parent) = dest.parent() {
        if let Err(err) = std::fs::create_dir_all(parent) {
            return WebResponse::<Value>::fail(format!("mkdir: {err}")).into_response();
        }
    }
    if let Err(err) = std::fs::write(&dest, &bytes) {
        return WebResponse::<Value>::fail(format!("write: {err}")).into_response();
    }

    let display_name = if name.is_empty() {
        std::path::Path::new(&dest_rel)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| dest_rel.clone())
    } else {
        name
    };
    let content_type = service::detect_content_type(&dest_rel, "");

    let libs = media_guard_service_core::Q::media_libraries()
        .order_by_id_asc()
        .limit(1)
        .comment("what: load media library root")
        .purpose("why: register uploaded media asset")
        .execute_for_list(&context)
        .await;
    let library_id = match libs {
        Ok(rows) => rows
            .iter()
            .next()
            .and_then(|l| media_guard_service_core::E::media_library(l).get_id().eval())
            .unwrap_or(0),
        Err(err) => return WebResponse::<Value>::fail(err.to_string()).into_response(),
    };
    if library_id == 0 {
        return WebResponse::<Value>::fail("no media library; restart server to seed").into_response();
    }

    match service::create_media_asset(
        &context,
        library_id,
        &display_name,
        &dest_rel,
        &content_type,
        bytes.len() as i64,
        0,
    )
    .await
    {
        Ok(asset) => {
            let id = media_guard_service_core::E::media_asset(&asset)
                .get_id()
                .eval()
                .unwrap_or(0);
            let base = service::external_base_url(&context).await.unwrap_or_default();
            let asset_ref = encode_asset_ref(&context, id).unwrap_or_default();
            WebResponse::of_single(json!({
                "mediaAssetId": id,
                "displayName": display_name,
                "filePath": dest_rel,
                "contentType": content_type,
                "fileSize": bytes.len(),
                "viewUrl": join_external(&base, &format!("/view/{asset_ref}")),
            }))
            .into_response()
        }
        Err(err) => WebResponse::<Value>::fail(err.to_string()).into_response(),
    }
}

/// GET /admin/assets?file_path=videos/demo.mp4&name=Demo
///
/// Register a media file under the library root (smoke/admin helper).
async fn admin_create_asset(
    State(state): State<AppState>,
    TeaContext(context): TeaContext,
    headers: HeaderMap,
    axum::extract::Query(params): axum::extract::Query<
        std::collections::HashMap<String, String>,
    >,
) -> Response {
    if let Err(resp) = check_admin_key(&headers) {
        return resp;
    }
    let Some(file_path) = params.get("file_path").cloned() else {
        return WebResponse::<Value>::fail("file_path query param required").into_response();
    };
    let name = params
        .get("name")
        .cloned()
        .unwrap_or_else(|| file_path.clone());
    let content_type = params
        .get("content_type")
        .cloned()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| service::detect_content_type(&file_path, ""));

    let fs_path = state.media_root.join(&file_path);
    let (file_size, duration_seconds) = match tokio::fs::metadata(&fs_path).await {
        Ok(m) => (m.len() as i64, 0i64),
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                WebResponse::<Value>::fail(format!(
                    "file not found under media root: {file_path}"
                )),
            )
                .into_response()
        }
    };

    let libs = media_guard_service_core::Q::media_libraries()
        .order_by_id_asc()
        .limit(1)
        .comment("what: load media library root")
        .purpose("why: register admin media asset")
        .execute_for_list(&context)
        .await;
    let library_id = match libs {
        Ok(rows) => rows
            .iter()
            .next()
            .and_then(|l| media_guard_service_core::E::media_library(l).get_id().eval())
            .unwrap_or(0),
        Err(err) => return WebResponse::<Value>::fail(err.to_string()).into_response(),
    };
    if library_id == 0 {
        return WebResponse::<Value>::fail("no media library; restart server to seed").into_response();
    }

    match service::create_media_asset(
        &context,
        library_id,
        &name,
        &file_path,
        &content_type,
        file_size,
        duration_seconds,
    )
    .await
    {
        Ok(asset) => {
            let id = media_guard_service_core::E::media_asset(&asset)
                .get_id()
                .eval()
                .unwrap_or(0);
            let asset_ref = encode_asset_ref(&context, id).unwrap_or_default();
            WebResponse::of_single(json!({
                "mediaAssetId": id,
                "name": name,
                "filePath": file_path,
                "fileSize": file_size,
                "url": format!("/media/{asset_ref}"),
            }))
            .into_response()
        }
        Err(err) => WebResponse::<Value>::fail(err.to_string()).into_response(),
    }
}
