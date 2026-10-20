//! Persisted browser sessions (`user_session` rows for `mg_sid` fingerprints).

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use media_guard_service_core::E;
use serde_json::Value;
use tower::ServiceExt;

use media_guard_service_core_web_axum::handlers::{app, AppState};
use media_guard_service_core_web_axum::service;
use media_guard_service_core_web_axum::session::SESSION_COOKIE;

const FP_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const FP_UNKNOWN: &str = "cccccccccccccccccccccccccccccccc";

fn temp_db_url(dir: &tempfile::TempDir) -> String {
    format!("sqlite://{}/media-guard.db", dir.path().display())
}

fn open_pool(db_path: &std::path::Path) -> media_guard_service_core::DataServicePool {
    Arc::new(Mutex::new(
        rusqlite::Connection::open(db_path).expect("open db"),
    ))
}

struct Fixture {
    _dir: tempfile::TempDir,
    context: teaql_runtime::UserContext,
    library_id: u64,
    handle_token: String,
    pool: media_guard_service_core::DataServicePool,
    db_path: std::path::PathBuf,
    media_root: PathBuf,
}

async fn setup() -> Fixture {
    let dir = tempfile::tempdir().expect("tempdir");
    let url = temp_db_url(&dir);
    unsafe {
        std::env::set_var("MEDIA_GUARD_SERVICE_CORE_DATABASE_URL", &url);
    }
    let db_path = dir.path().join("media-guard.db");
    let pool = open_pool(&db_path);
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
    let handle = service::ensure_share_handle(&context, library_id, asset_id)
        .await
        .expect("handle");
    let handle_token = E::share_handle(&handle)
        .get_handle_token()
        .eval()
        .expect("handle token");
    service::create_proxy_trust_rule(&context, library_id, "X-Forwarded-For", "127.0.0.1/32")
        .await
        .expect("proxy rule");
    service::grant_ip_access(
        &context,
        library_id,
        asset_id,
        "127.0.0.1",
        None,
        "test whitelist",
    )
    .await
    .expect("grant");
    Fixture {
        _dir: dir,
        context,
        library_id,
        handle_token,
        pool,
        db_path,
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

#[tokio::test]
async fn ensure_user_session_creates_row() {
    let fix = setup().await;

    let session = service::ensure_user_session(&fix.context, fix.library_id, FP_A)
        .await
        .expect("ensure");
    let id = E::user_session(&session).get_id().eval().expect("id");
    assert!(id > 0, "id={id}");
    assert_eq!(
        E::user_session(&session)
            .get_session_fp()
            .eval()
            .expect("session_fp"),
        FP_A
    );
    assert!(E::user_session(&session).get_active().eval().unwrap_or(false));
    assert_eq!(
        E::user_session(&session)
            .get_media_library_id()
            .eval()
            .unwrap_or(0),
        fix.library_id
    );
    let last_seen = E::user_session(&session)
        .get_last_seen_at()
        .eval()
        .unwrap_or_default();
    assert!(!last_seen.is_empty(), "last_seen_at must be stamped");
    let expire = E::user_session(&session)
        .get_expire_at()
        .eval()
        .unwrap_or_default();
    assert!(!expire.is_empty(), "expire_at must be stamped");
    assert!(service::grant_is_still_valid(&expire, true));
}

#[tokio::test]
async fn ensure_user_session_touches_same_row() {
    let fix = setup().await;

    let first = service::ensure_user_session(&fix.context, fix.library_id, FP_A)
        .await
        .expect("first");
    let first_id = E::user_session(&first).get_id().eval().expect("id");
    let first_seen = E::user_session(&first)
        .get_last_seen_at()
        .eval()
        .unwrap_or_default();

    let second = service::ensure_user_session(&fix.context, fix.library_id, FP_A)
        .await
        .expect("second");
    let second_id = E::user_session(&second).get_id().eval().expect("id");
    let second_seen = E::user_session(&second)
        .get_last_seen_at()
        .eval()
        .unwrap_or_default();

    assert_eq!(first_id, second_id, "re-ensure must update, not recreate");
    assert!(second_seen >= first_seen, "last_seen must advance");
    assert!(service::grant_is_still_valid(
        &E::user_session(&second)
            .get_expire_at()
            .eval()
            .unwrap_or_default(),
        true
    ));
}

#[tokio::test]
async fn is_session_active_after_ensure_and_false_for_unknown() {
    let fix = setup().await;

    assert!(
        !service::is_session_active(&fix.context, FP_UNKNOWN)
            .await
            .expect("unknown check"),
        "unknown fingerprint must be inactive"
    );
    assert!(
        !service::is_session_active(&fix.context, "").await.expect("empty"),
        "empty fingerprint must be inactive"
    );

    service::ensure_user_session(&fix.context, fix.library_id, FP_A)
        .await
        .expect("ensure");
    assert!(
        service::is_session_active(&fix.context, FP_A)
            .await
            .expect("known check"),
        "ensured fingerprint must be active"
    );
}

#[tokio::test]
async fn deactivate_user_session_revokes() {
    let fix = setup().await;

    service::ensure_user_session(&fix.context, fix.library_id, FP_A)
        .await
        .expect("ensure");
    service::deactivate_user_session(&fix.context, FP_A)
        .await
        .expect("deactivate");
    assert!(
        !service::is_session_active(&fix.context, FP_A)
            .await
            .expect("check"),
        "deactivated fingerprint must be inactive"
    );
    // Unknown fingerprints stay a no-op.
    service::deactivate_user_session(&fix.context, FP_UNKNOWN)
        .await
        .expect("deactivate unknown");
}

#[tokio::test]
async fn session_survives_restart_on_same_db() {
    let fix = setup().await;

    service::ensure_user_session(&fix.context, fix.library_id, FP_A)
        .await
        .expect("ensure");
    assert!(
        service::is_session_active(&fix.context, FP_A).await.expect("check"),
        "session must be active before restart"
    );

    // Simulate a process restart: drop the first connection, reopen the same file.
    drop(fix.context);
    drop(fix.pool);

    let pool2 = open_pool(&fix.db_path);
    let context2 = media_guard_service_core_web_axum::build_request_context(&pool2);
    assert!(
        service::is_session_active(&context2, FP_A).await.expect("restart check"),
        "session must survive restart on the same DB"
    );
    assert!(
        !service::is_session_active(&context2, FP_UNKNOWN)
            .await
            .expect("unknown after restart"),
        "unknown fingerprint stays inactive after restart"
    );
}

#[tokio::test]
async fn open_handle_persists_session_and_media_works_after_restart() {
    let fix = setup().await;
    let router = app(AppState {
        media_root: fix.media_root.clone(),
        pool: fix.pool.clone(),
    });

    // Mint through /open with a pre-set cookie fingerprint.
    let req = with_peer(
        with_cookie(
            Request::builder()
                .uri(format!("/open/{}", fix.handle_token))
                .body(Body::empty())
                .unwrap(),
            FP_A,
        ),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp.into_body()).await;
    let item = body
        .get("data")
        .and_then(|d| d.as_array())
        .expect("data array")
        .first()
        .expect("one item");
    let media_ref = item
        .get("mediaRef")
        .and_then(|v| v.as_str())
        .expect("mediaRef")
        .to_owned();
    let text = body.to_string();
    assert!(
        !text.contains(FP_A) && !text.contains("sessionFp") && !text.contains("session_fp"),
        "guest JSON must not leak session_fp: {text}"
    );

    // Cookie-less /open mints a fresh persisted session + cookie.
    let resp = router
        .clone()
        .oneshot(with_peer(
            Request::builder()
                .uri(format!("/open/{}", fix.handle_token))
                .body(Body::empty())
                .unwrap(),
            peer_localhost(),
        ))
        .await
        .unwrap();
    let minted_fp = session_fp_from_set_cookie(resp.headers());
    assert!(service::is_session_active(&fix.context, &minted_fp).await.expect("minted"));

    // "Restart": new connection over the same DB file.
    drop(fix.context);
    drop(fix.pool);
    let pool2 = open_pool(&fix.db_path);
    let context2 = media_guard_service_core_web_axum::build_request_context(&pool2);
    assert!(
        service::is_session_active(&context2, FP_A).await.expect("fp_a"),
        "cookie session must survive restart"
    );
    assert!(
        service::is_session_active(&context2, &minted_fp).await.expect("minted"),
        "minted session must survive restart"
    );

    let router2 = app(AppState {
        media_root: fix.media_root.clone(),
        pool: pool2,
    });

    // Original cookie + ref still streams after restart.
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
    let resp = router2.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::PARTIAL_CONTENT);

    // Unknown cookie fingerprint is rejected even for a well-formed ref.
    let req = with_peer(
        with_cookie(
            Request::builder()
                .uri(format!("/media/{media_ref}"))
                .header("Range", "bytes=0-3")
                .body(Body::empty())
                .unwrap(),
            FP_UNKNOWN,
        ),
        peer_localhost(),
    );
    let resp = router2.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let text = body_text(resp.into_body()).await;
    assert!(text.contains("invalid media reference"), "body: {text}");

    // Deactivate → same cookie is rejected after restart as well.
    service::deactivate_user_session(&context2, FP_A)
        .await
        .expect("deactivate");
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
    let resp = router2.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let text = body_text(resp.into_body()).await;
    assert!(text.contains("invalid media reference"), "body: {text}");
}
