
//! `GET /open/{handle_token}`: durable share handles mint session-bound media
//! refs ([S4]–[S8]).

use std::net::SocketAddr;
use std::path::PathBuf;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use media_guard_service_core::E;
use serde_json::Value;
use tower::ServiceExt;

use media_guard_service_core_web_axum::entity_ref::decode_asset_ref_for_session;
use media_guard_service_core_web_axum::handlers::{app, AppState};
use media_guard_service_core_web_axum::service;
use media_guard_service_core_web_axum::session::SESSION_COOKIE;

const FP_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const FP_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn temp_db_url(dir: &tempfile::TempDir) -> String {
    format!("sqlite://{}/media-guard.db", dir.path().display())
}

struct Fixture {
    _dir: tempfile::TempDir,
    context: teaql_runtime::UserContext,
    asset_id: u64,
    handle_token: String,
    pool: media_guard_service_core::DataServicePool,
    media_root: PathBuf,
}

async fn setup(ip_allowed: bool) -> Fixture {
    let dir = tempfile::tempdir().expect("tempdir");
    let url = temp_db_url(&dir);
    unsafe {
        std::env::set_var("MEDIA_GUARD_SERVICE_CORE_DATABASE_URL", &url);
    }
    let pool: media_guard_service_core::DataServicePool = std::sync::Arc::new(
        std::sync::Mutex::new(
            rusqlite::Connection::open(dir.path().join("media-guard.db")).expect("open db"),
        ),
    );
    let context = media_guard_service_core_web_axum::build_request_context(&pool);
    context.ensure_schema().await.expect("schema");

    let media_root = dir.path().join("media");
    std::fs::create_dir_all(media_root.join("videos")).expect("mkdir");
    std::fs::write(media_root.join("videos/demo.mp4"), b"0123456789ABCDEF").expect("write");

    let library = service::ensure_media_library(
        &context,
        "Test Library",
        &media_root.to_string_lossy(),
    )
    .await
    .expect("library");
    let library_id = E::media_library(&library).get_id().eval().unwrap_or(1);
    let asset = service::create_media_asset(
        &context,
        library_id,
        "Demo Video",
        "videos/demo.mp4",
        "video/mp4",
        16,
        1,
    )
    .await
    .expect("asset");
    let asset_id = E::media_asset(&asset).get_id().eval().unwrap_or(1);
    service::create_proxy_trust_rule(&context, library_id, "X-Forwarded-For", "10.0.0.0/8,127.0.0.1")
        .await
        .expect("proxy rule");
    if ip_allowed {
        service::grant_ip_access(
            &context,
            library_id,
            asset_id,
            "127.0.0.1",
            None,
            "test open allow",
        )
        .await
        .expect("grant");
    }
    let handle = service::ensure_share_handle(&context, library_id, asset_id)
        .await
        .expect("handle");
    let handle_token = E::share_handle(&handle)
        .get_handle_token()
        .eval()
        .expect("handle token");
    Fixture {
        _dir: dir,
        context,
        asset_id,
        handle_token,
        pool,
        media_root,
    }
}

fn peer_localhost() -> SocketAddr {
    "127.0.0.1:5555".parse().unwrap()
}

fn with_peer(mut req: Request<Body>, peer: SocketAddr) -> Request<Body> {
    req.extensions_mut().insert(axum::extract::ConnectInfo(peer));
    req
}

fn with_cookie(mut req: Request<Body>, fp: &str) -> Request<Body> {
    req.headers_mut().insert(
        axum::http::header::COOKIE,
        format!("{SESSION_COOKIE}={fp}").parse().unwrap(),
    );
    req
}

async fn body_text(body: Body) -> String {
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8_lossy(&bytes).to_string()
}

async fn body_json(body: Body) -> Value {
    let text = body_text(body).await;
    serde_json::from_str(&text).unwrap_or_else(|err| panic!("invalid JSON ({err}): {text}"))
}

fn single_item(body: &Value) -> &Value {
    body.get("data")
        .and_then(|d| d.as_array())
        .expect("data array")
        .first()
        .expect("one item")
}

/// Extract `mg_sid` from a `Set-Cookie` response header.
fn session_fp_from_set_cookie(headers: &axum::http::HeaderMap) -> String {
    for value in headers.get_all(axum::http::header::SET_COOKIE) {
        let raw = value.to_str().expect("Set-Cookie utf8");
        for part in raw.split(';') {
            let part = part.trim();
            if let Some(rest) = part.strip_prefix(SESSION_COOKIE) {
                if let Some(fp) = rest.strip_prefix('=') {
                    return fp.to_owned();
                }
            }
        }
    }
    panic!("expected Set-Cookie {SESSION_COOKIE}");
}

fn router_for(fix: &Fixture) -> axum::Router {
    app(AppState {
        media_root: fix.media_root.clone(),
        pool: fix.pool.clone(),
    })
}

/// Short auto-ids collide with random AEAD/base64 bytes and the `tqr1.` prefix;
/// only multi-digit ids make a substring leak check meaningful.
fn assert_no_raw_id_leak(text: &str, asset_id: u64) {
    assert!(
        asset_id < 10 || !text.contains(&asset_id.to_string()),
        "raw id leaked: {text}"
    );
}

#[tokio::test]
async fn open_handle_redirects_to_session_view_ref() {
    let fix = setup(true).await;
    let router = router_for(&fix);

    let req = with_peer(
        with_cookie(
            Request::builder()
                .uri(format!("/open/{}", fix.handle_token))
                .header("Accept", "text/html")
                .body(Body::empty())
                .unwrap(),
            FP_A,
        ),
        peer_localhost(),
    );
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .expect("Location")
        .to_owned();
    assert!(
        location.starts_with("/view/tqr1."),
        "redirect must use session ref: {location}"
    );
    assert!(
        !location.contains("SHD-"),
        "redirect must not contain handle token: {location}"
    );
    assert_no_raw_id_leak(&location, fix.asset_id);
    assert!(
        !location.contains(&fix.handle_token),
        "redirect must not echo the handle: {location}"
    );
    let media_ref = location.trim_start_matches("/view/");
    assert_eq!(
        decode_asset_ref_for_session(&fix.context, media_ref, FP_A).expect("decode with fp-a"),
        fix.asset_id
    );
}

#[tokio::test]
async fn two_sessions_get_different_refs_bound_to_their_cookies() {
    let fix = setup(true).await;
    let router = router_for(&fix);

    let req_a = with_peer(
        with_cookie(
            Request::builder()
                .uri(format!("/open/{}", fix.handle_token))
                .header("Accept", "text/html")
                .body(Body::empty())
                .unwrap(),
            FP_A,
        ),
        peer_localhost(),
    );
    let resp_a = router.clone().oneshot(req_a).await.unwrap();
    assert_eq!(resp_a.status(), StatusCode::SEE_OTHER);
    let loc_a = resp_a
        .headers()
        .get(axum::http::header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .unwrap()
        .to_owned();

    let req_b = with_peer(
        with_cookie(
            Request::builder()
                .uri(format!("/open/{}", fix.handle_token))
                .header("Accept", "text/html")
                .body(Body::empty())
                .unwrap(),
            FP_B,
        ),
        peer_localhost(),
    );
    let resp_b = router.clone().oneshot(req_b).await.unwrap();
    assert_eq!(resp_b.status(), StatusCode::SEE_OTHER);
    let loc_b = resp_b
        .headers()
        .get(axum::http::header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .unwrap()
        .to_owned();

    let ref_a = loc_a.trim_start_matches("/view/").to_owned();
    let ref_b = loc_b.trim_start_matches("/view/").to_owned();
    assert_ne!(ref_a, ref_b, "two sessions must mint two refs");

    assert_eq!(
        decode_asset_ref_for_session(&fix.context, &ref_a, FP_A).expect("ref_a with fp-a"),
        fix.asset_id
    );
    assert_eq!(
        decode_asset_ref_for_session(&fix.context, &ref_b, FP_B).expect("ref_b with fp-b"),
        fix.asset_id
    );
    assert!(decode_asset_ref_for_session(&fix.context, &ref_a, FP_B).is_err());
    assert!(decode_asset_ref_for_session(&fix.context, &ref_b, FP_A).is_err());
}

#[tokio::test]
async fn session_ref_only_works_with_minting_cookie() {
    let fix = setup(true).await;
    let router = router_for(&fix);

    let req = with_peer(
        with_cookie(
            Request::builder()
                .uri(format!("/open/{}", fix.handle_token))
                .header("Accept", "text/html")
                .body(Body::empty())
                .unwrap(),
            FP_A,
        ),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .unwrap()
        .to_owned();
    let media_ref = location.trim_start_matches("/view/").to_owned();

    // Minting cookie works on the media API.
    let req = with_peer(
        with_cookie(
            Request::builder()
                .uri(format!("/api/media/{media_ref}"))
                .body(Body::empty())
                .unwrap(),
            FP_A,
        ),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Same cookie works for streaming.
    let req = with_peer(
        with_cookie(
            Request::builder()
                .uri(format!("/media/{media_ref}"))
                .header("Range", "bytes=0-3")
                .body(Body::empty())
                .unwrap(),
            FP_A,
        ),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::PARTIAL_CONTENT);

    // Other session cookie → 400 invalid media reference.
    let req = with_peer(
        with_cookie(
            Request::builder()
                .uri(format!("/api/media/{media_ref}"))
                .body(Body::empty())
                .unwrap(),
            FP_B,
        ),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let text = body_text(resp.into_body()).await;
    assert!(text.contains("invalid media reference"), "body: {text}");
    assert!(!text.contains(&fix.handle_token), "handle leaked: {text}");

    // No cookie → 400 as well.
    let req = with_peer(
        Request::builder()
            .uri(format!("/media/{media_ref}"))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let text = body_text(resp.into_body()).await;
    assert!(text.contains("invalid media reference"), "body: {text}");
}

#[tokio::test]
async fn open_without_cookie_sets_mg_sid_and_returns_json_media_ref() {
    let fix = setup(true).await;
    let router = router_for(&fix);

    let req = with_peer(
        Request::builder()
            .uri(format!("/open/{}", fix.handle_token))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let set_cookie = resp
        .headers()
        .get(axum::http::header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .expect("Set-Cookie")
        .to_owned();
    assert!(set_cookie.starts_with("mg_sid="), "cookie: {set_cookie}");
    assert!(set_cookie.contains("HttpOnly"), "cookie: {set_cookie}");
    assert!(set_cookie.contains("Path=/"), "cookie: {set_cookie}");
    let fp = session_fp_from_set_cookie(resp.headers());

    let body = body_json(resp.into_body()).await;
    let item = single_item(&body);
    let media_ref = item.get("mediaRef").and_then(|v| v.as_str()).expect("mediaRef");
    assert!(media_ref.starts_with("tqr1."), "mediaRef={media_ref}");
    assert!(item.get("handleToken").is_none(), "guest JSON leaked handle: {item}");
    assert!(item.get("mediaAssetId").is_none(), "guest JSON leaked id: {item}");
    let text = body.to_string();
    assert!(!text.contains("SHD-"), "handle token leaked: {text}");
    assert_eq!(
        decode_asset_ref_for_session(&fix.context, media_ref, &fp).expect("decode"),
        fix.asset_id
    );
}

#[tokio::test]
async fn inactive_handle_returns_410() {
    let fix = setup(true).await;
    let handle = service::load_share_handle_by_token(&fix.context, &fix.handle_token)
        .await
        .expect("load")
        .expect("found");
    let handle_id = E::share_handle(&handle).get_id().eval().unwrap_or(0);
    service::set_share_handle_active(&fix.context, handle_id, false)
        .await
        .expect("deactivate");
    let router = router_for(&fix);

    let req = with_peer(
        with_cookie(
            Request::builder()
                .uri(format!("/open/{}", fix.handle_token))
                .header("Accept", "text/html")
                .body(Body::empty())
                .unwrap(),
            FP_A,
        ),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::GONE);
    let text = body_text(resp.into_body()).await;
    assert!(!text.contains("SHD-"), "handle leaked: {text}");
    assert_no_raw_id_leak(&text, fix.asset_id);

    let req = with_peer(
        Request::builder()
            .uri(format!("/open/{}", fix.handle_token))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::GONE);
    let body = body_json(resp.into_body()).await;
    let text = body.to_string();
    assert!(!text.contains("SHD-"), "handle leaked: {text}");
    assert_no_raw_id_leak(&text, fix.asset_id);
}

#[tokio::test]
async fn unknown_handle_returns_404() {
    let fix = setup(true).await;
    let router = router_for(&fix);

    let req = with_peer(
        Request::builder()
            .uri("/open/SHD-deadbeefdeadbeefdeadbeef")
            .header("Accept", "text/html")
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let text = body_text(resp.into_body()).await;
    assert!(!text.contains("SHD-"), "handle echoed: {text}");

    let req = with_peer(
        Request::builder()
            .uri("/open/SHD-deadbeefdeadbeefdeadbeef")
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let body = body_json(resp.into_body()).await;
    let text = body.to_string();
    assert!(!text.contains("SHD-"), "handle echoed: {text}");
    assert_no_raw_id_leak(&text, fix.asset_id);
}

#[tokio::test]
async fn unauthorized_ip_does_not_leak_asset_or_handle() {
    let fix = setup(false).await;
    let router = router_for(&fix);

    // HTML: bounce to the invite gate, no asset id / handle in Location.
    let req = with_peer(
        with_cookie(
            Request::builder()
                .uri(format!("/open/{}", fix.handle_token))
                .header("Accept", "text/html")
                .body(Body::empty())
                .unwrap(),
            FP_A,
        ),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .expect("Location")
        .to_owned();
    assert_eq!(location, "/?need_invite=1");
    assert!(!location.contains("SHD-"), "handle leaked: {location}");
    assert!(!location.contains("tqr1."), "ref leaked: {location}");
    assert_no_raw_id_leak(&location, fix.asset_id);

    // JSON: 403, no asset id / handle in body.
    let req = with_peer(
        Request::builder()
            .uri(format!("/open/{}", fix.handle_token))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    let body = body_json(resp.into_body()).await;
    let text = body.to_string();
    assert!(!text.contains("SHD-"), "handle leaked: {text}");
    assert!(!text.contains("tqr1."), "ref leaked: {text}");
    assert_no_raw_id_leak(&text, fix.asset_id);
}

#[tokio::test]
async fn admin_media_exposes_share_url_with_open_path() {
    let fix = setup(true).await;
    unsafe {
        std::env::set_var("MEDIA_GUARD_ADMIN_KEY", "open-test-admin-key");
    }
    let router = router_for(&fix);

    let req = with_peer(
        Request::builder()
            .uri("/admin/api/media")
            .header("x-admin-key", "open-test-admin-key")
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp.into_body()).await;
    let items = body
        .get("data")
        .and_then(|d| d.as_array())
        .expect("data array");
    let item = items
        .iter()
        .find(|it| it.get("id").and_then(|v| v.as_u64()) == Some(fix.asset_id))
        .expect("asset row");
    let share_url = item
        .get("shareUrl")
        .and_then(|v| v.as_str())
        .expect("shareUrl");
    assert!(
        share_url.contains(&format!("/open/{}", fix.handle_token)),
        "shareUrl={share_url}"
    );
    assert!(
        share_url.starts_with("/open/SHD-") || share_url.contains("/open/SHD-"),
        "shareUrl must use /open handle path: {share_url}"
    );
}
