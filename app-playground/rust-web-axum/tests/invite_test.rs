
//! Invite one-shot URL + IP ACL + Range tests for the Axum web workspace.

use std::net::SocketAddr;
use std::path::PathBuf;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use media_guard_service_core::{service_runtime_from_pool, E, Q};
use tower::ServiceExt;

use media_guard_service_core_web_axum::entity_ref::encode_asset_ref;
use media_guard_service_core_web_axum::handlers::{app, AppState};
use media_guard_service_core_web_axum::service::{self, InviteOutcome};

fn temp_db_url(dir: &tempfile::TempDir) -> String {
    format!("sqlite://{}/media-guard.db", dir.path().display())
}

async fn setup() -> (
    tempfile::TempDir,
    teaql_runtime::UserContext,
    u64,
    u64,
    String,
    media_guard_service_core::DataServicePool,
) {
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
    let runtime = service_runtime_from_pool(pool.clone()).await.expect("runtime");
    runtime.ensure_schema().await.expect("schema");

    let media_root = dir.path().join("media");
    std::fs::create_dir_all(media_root.join("videos")).expect("mkdir");
    std::fs::write(media_root.join("videos/demo.mp4"), b"0123456789ABCDEF").expect("write");

    let context = runtime;
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
    let invite = service::create_invite(&context, library_id, asset_id, "test invite")
        .await
        .expect("invite");
    let token = E::invite_code(&invite).get_invite_token().eval().unwrap();
    (dir, context, library_id, asset_id, token, pool)
}

fn peer_localhost() -> SocketAddr {
    "127.0.0.1:5555".parse().unwrap()
}

fn peer_proxy() -> SocketAddr {
    "10.0.0.2:443".parse().unwrap()
}

fn with_peer(mut req: Request<Body>, peer: SocketAddr) -> Request<Body> {
    req.extensions_mut().insert(axum::extract::ConnectInfo(peer));
    req
}

async fn body_text(body: Body) -> String {
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8_lossy(&bytes).to_string()
}

#[tokio::test]
async fn invite_first_click_joins_whitelist() {
    let (_dir, context, _lib, asset_id, token, _executor) = setup().await;

    let outcome = service::redeem_invite(&context, &token, "192.168.1.10")
        .await
        .expect("first click");
    match outcome {
        InviteOutcome::Joined {
            client_ip,
            media_asset_id,
            expire_at,
            ..
        } => {
            assert_eq!(client_ip, "192.168.1.10");
            assert_eq!(media_asset_id, asset_id);
            assert!(service::grant_is_still_valid(&expire_at, true));
        }
        other => panic!("expected Joined, got {other:?}"),
    }
    assert!(service::is_ip_allowed(&context, asset_id, "192.168.1.10")
        .await
        .unwrap());
}

#[tokio::test]
async fn invite_second_click_expires_from_whitelist() {
    let (_dir, context, _lib, asset_id, token, _executor) = setup().await;

    let _ = service::redeem_invite(&context, &token, "192.168.1.10")
        .await
        .expect("first");
    assert!(service::is_ip_allowed(&context, asset_id, "192.168.1.10")
        .await
        .unwrap());

    let outcome = service::redeem_invite(&context, &token, "192.168.1.10")
        .await
        .expect("second");
    match outcome {
        InviteOutcome::ExpiredFromWhitelist {
            client_ip,
            revoked_grants,
            ..
        } => {
            assert_eq!(client_ip, "192.168.1.10");
            assert!(revoked_grants >= 1);
        }
        other => panic!("expected ExpiredFromWhitelist, got {other:?}"),
    }
    assert!(!service::is_ip_allowed(&context, asset_id, "192.168.1.10")
        .await
        .unwrap());
}

#[tokio::test]
async fn invite_third_click_rejected_as_spent() {
    let (_dir, context, _lib, _asset, token, _executor) = setup().await;
    let _ = service::redeem_invite(&context, &token, "192.168.1.10").await.unwrap();
    let _ = service::redeem_invite(&context, &token, "192.168.1.10").await.unwrap();
    let outcome = service::redeem_invite(&context, &token, "192.168.1.10").await.unwrap();
    assert!(matches!(outcome, InviteOutcome::AlreadySpent { .. }));
}

#[tokio::test]
async fn invite_http_url_join_then_expire() {
    let (dir, _context, _lib, asset_id, token, executor) = setup().await;
    // Encode via production request-context wiring (installs the ref codec).
    let encode_ctx = media_guard_service_core_web_axum::build_request_context(&executor);
    let asset_ref = encode_asset_ref(&encode_ctx, asset_id).expect("encode ref");
    let media_root = dir.path().join("media");
    let router = app(AppState {
        media_root: PathBuf::from(&media_root),
        pool: executor,
    });

    // 1st click via dedicated URL
    let req = with_peer(
        Request::builder()
            .uri(format!("/invite/{token}"))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let text = body_text(resp.into_body()).await;
    assert!(text.contains("join_whitelist"), "body: {text}");

    // After join, media is playable for this IP with Range
    let req = with_peer(
        Request::builder()
            .uri(format!("/media/{asset_ref}"))
            .header("Range", "bytes=0-3")
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::PARTIAL_CONTENT);

    // 2nd click expires from whitelist
    let req = with_peer(
        Request::builder()
            .uri(format!("/invite/{token}"))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let text = body_text(resp.into_body()).await;
    assert!(text.contains("expire_whitelist"), "body: {text}");

    // Media now denied
    let req = with_peer(
        Request::builder()
            .uri(format!("/media/{asset_ref}"))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // 3rd click -> 410 Gone
    let req = with_peer(
        Request::builder()
            .uri(format!("/invite/{token}"))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::GONE);
}

#[tokio::test]
async fn proxy_client_ip_uses_forwarded_header() {
    let (dir, context, _lib, asset_id, token, executor) = setup().await;
    let media_root = dir.path().join("media");
    let router = app(AppState {
        media_root: PathBuf::from(&media_root),
        pool: executor,
    });

    // Join via proxy: peer 10.0.0.2 is trusted, XFF client is 203.0.113.50
    let req = with_peer(
        Request::builder()
            .uri(format!("/invite/{token}"))
            .header("X-Forwarded-For", "8.8.8.8, 203.0.113.50")
            .body(Body::empty())
            .unwrap(),
        peer_proxy(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    let text = body_text(resp.into_body()).await;
    assert!(text.contains("203.0.113.50"), "body: {text}");
    assert!(text.contains("join_whitelist"), "body: {text}");

    assert!(service::is_ip_allowed(&context, asset_id, "203.0.113.50")
        .await
        .unwrap());
    assert!(!service::is_ip_allowed(&context, asset_id, "8.8.8.8")
        .await
        .unwrap());
}

#[tokio::test]
#[should_panic(expected = "query purpose must not be empty")]
async fn empty_purpose_comment_rejected() {
    let (_dir, context, _lib, _asset, _token, _executor) = setup().await;
    let _ = Q::media_assets_minimal()
        .order_by_id_asc()
        .limit(1)
        .comment("")
        .purpose("")
        .execute_for_list(&context)
        .await;
}
