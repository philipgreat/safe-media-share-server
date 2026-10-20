
//! Guest API / media routes must take encrypted entity refs (`tqr1.` tokens),
//! never raw numeric media asset ids.

use std::net::SocketAddr;
use std::path::PathBuf;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use media_guard_service_core::E;
use serde_json::Value;
use tower::ServiceExt;

use media_guard_service_core_web_axum::entity_ref::{decode_asset_ref, encode_asset_ref};
use media_guard_service_core_web_axum::handlers::{app, AppState};
use media_guard_service_core_web_axum::service;

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
    // Same wiring as production request contexts (installs the entity ref codec).
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
    let invite = service::create_invite(&context, library_id, asset_id, "test invite")
        .await
        .expect("invite");
    let token = E::invite_code(&invite).get_invite_token().eval().unwrap();
    (dir, context, library_id, asset_id, token, pool)
}

fn peer_localhost() -> SocketAddr {
    "127.0.0.1:5555".parse().unwrap()
}

fn with_peer(mut req: Request<Body>, peer: SocketAddr) -> Request<Body> {
    req.extensions_mut().insert(axum::extract::ConnectInfo(peer));
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

fn guest_items(body: &Value) -> Vec<&Value> {
    body.get("data")
        .and_then(|d| d.as_array())
        .expect("data array")
        .iter()
        .collect()
}

/// Assert guest JSON has `ref` (tqr1.) and no raw `"id"` field.
fn assert_guest_asset_json(item: &Value, asset_id: u64) {
    let asset_ref = item.get("ref").and_then(|v| v.as_str()).expect("ref field");
    assert!(asset_ref.starts_with("tqr1."), "ref={asset_ref}");
    // Substring checks against short auto-ids flake: the "tqr1." prefix itself
    // contains "1", and random AEAD bytes collide with 1-2 digit strings. Only
    // multi-digit ids make a "contains" leak check meaningful.
    assert!(
        asset_id < 10 || !asset_ref.contains(&asset_id.to_string()),
        "raw id leaked in ref: {asset_ref}"
    );
    assert!(
        item.get("id").is_none(),
        "guest JSON must not expose raw id: {item}"
    );
    let text = item.to_string();
    assert!(
        !text.contains(&format!("\"id\":{asset_id}")) && !text.contains(&format!("\"id\": {asset_id}")),
        "raw id leaked in guest JSON: {text}"
    );
}

#[tokio::test]
async fn api_list_media_exposes_refs_not_raw_ids() {
    let (_dir, context, _lib, asset_id, _token, pool) = setup().await;
    let router = app(AppState {
        media_root: PathBuf::from("/unused"),
        pool,
    });

    let req = with_peer(
        Request::builder()
            .uri("/api/media")
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp.into_body()).await;
    let items = guest_items(&body);
    assert!(!items.is_empty(), "expected at least one media item");
    for item in &items {
        assert_guest_asset_json(item, asset_id);
    }
    // Double-check the full body does not carry a guest `"id"` key.
    let text = body.to_string();
    assert!(!text.contains("\"id\":"), "guest list leaked id field: {text}");
    let _ = context;
}

#[tokio::test]
async fn api_media_detail_accepts_encoded_ref() {
    let (_dir, context, _lib, asset_id, _token, pool) = setup().await;
    let asset_ref = encode_asset_ref(&context, asset_id).expect("encode ref");
    let router = app(AppState {
        media_root: PathBuf::from("/unused"),
        pool,
    });

    let req = with_peer(
        Request::builder()
            .uri(format!("/api/media/{asset_ref}"))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp.into_body()).await;
    let items = guest_items(&body);
    assert_eq!(items.len(), 1);
    assert_guest_asset_json(items[0], asset_id);
    assert_eq!(items[0].get("displayName").and_then(|v| v.as_str()), Some("Demo Video"));
}

#[tokio::test]
async fn api_media_detail_rejects_garbage_ref() {
    let (_dir, _context, _lib, _asset_id, _token, pool) = setup().await;
    let router = app(AppState {
        media_root: PathBuf::from("/unused"),
        pool,
    });

    let req = with_peer(
        Request::builder()
            .uri("/api/media/tqr1.bogus")
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let text = body_text(resp.into_body()).await;
    assert!(
        !text.contains("\"id\""),
        "error body leaked id field: {text}"
    );
    // Must not echo a raw numeric asset id field/value pattern.
    assert!(
        !text.contains("\"data\":[{\"id\""),
        "error body leaked raw id: {text}"
    );
}

#[tokio::test]
async fn api_media_detail_rejects_raw_numeric_id_path() {
    // Do not fall back to raw-id parsing in guest routes.
    let (_dir, _context, _lib, asset_id, _token, pool) = setup().await;
    let router = app(AppState {
        media_root: PathBuf::from("/unused"),
        pool,
    });

    let req = with_peer(
        Request::builder()
            .uri(format!("/api/media/{asset_id}"))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let text = body_text(resp.into_body()).await;
    assert!(
        !text.contains(&format!("\"id\":{asset_id}")),
        "error body leaked raw id: {text}"
    );
}

#[tokio::test]
async fn media_stream_and_download_work_with_refs() {
    let (dir, context, _lib, asset_id, token, pool) = setup().await;
    let asset_ref = encode_asset_ref(&context, asset_id).expect("encode ref");
    let media_root = dir.path().join("media");
    let router = app(AppState {
        media_root: PathBuf::from(&media_root),
        pool,
    });

    // Join whitelist first (needed for playback).
    let req = with_peer(
        Request::builder()
            .uri(format!("/invite/{token}"))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Range stream via encrypted ref → 206.
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
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&bytes[..], b"0123");

    // Download via encrypted ref → attachment.
    let req = with_peer(
        Request::builder()
            .uri(format!("/media/{asset_ref}/download"))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let disposition = resp
        .headers()
        .get(axum::http::header::CONTENT_DISPOSITION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    assert!(
        disposition.contains("attachment"),
        "expected attachment disposition: {disposition}"
    );
}

#[tokio::test]
async fn media_stream_rejects_garbage_ref_without_raw_id_leak() {
    let (dir, _context, _lib, asset_id, _token, pool) = setup().await;
    let media_root = dir.path().join("media");
    let router = app(AppState {
        media_root: PathBuf::from(&media_root),
        pool,
    });

    let req = with_peer(
        Request::builder()
            .uri("/media/tqr1.bogus")
            .header("Range", "bytes=0-3")
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let text = body_text(resp.into_body()).await;
    assert!(
        asset_id < 10 || !text.contains(&asset_id.to_string()),
        "unexpected raw id leak for small auto id; body: {text}"
    );
    assert!(!text.contains("\"id\""), "error body leaked id field: {text}");
}

#[tokio::test]
async fn invite_json_and_redirect_use_media_ref() {
    let (_dir, context, _lib, asset_id, token, pool) = setup().await;
    let router = app(AppState {
        media_root: PathBuf::from("/unused"),
        pool,
    });

    // JSON redeem: mediaRef, no raw mediaAssetId.
    let req = with_peer(
        Request::builder()
            .uri(format!("/invite/{token}"))
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp.into_body()).await;
    let items = guest_items(&body);
    assert_eq!(items.len(), 1);
    let item = items[0];
    let media_ref = item
        .get("mediaRef")
        .and_then(|v| v.as_str())
        .expect("mediaRef field");
    assert!(media_ref.starts_with("tqr1."), "mediaRef={media_ref}");
    assert_eq!(decode_asset_ref(&context, media_ref).expect("decode"), asset_id);
    assert!(
        item.get("mediaAssetId").is_none(),
        "guest invite JSON must not expose raw mediaAssetId: {item}"
    );
    assert!(
        item.get("inviteId").is_none(),
        "guest invite JSON must not expose raw inviteId: {item}"
    );
    let text = body.to_string();
    assert!(
        !text.contains(&format!("\"mediaAssetId\":{asset_id}")),
        "raw id leaked in invite JSON: {text}"
    );
    assert!(
        !text.contains("\"inviteId\""),
        "inviteId leaked in guest invite JSON: {text}"
    );

    // HTML redeem redirect: Location is /view/{ref}, not /view/{id}.
    // Use a second invite via admin-free service call... but invite is one-shot.
    // Re-setup a fresh invite for the HTML path.
    let (_dir2, context2, _lib2, asset_id2, token2, pool2) = setup().await;
    let router2 = app(AppState {
        media_root: PathBuf::from("/unused"),
        pool: pool2,
    });
    let req = with_peer(
        Request::builder()
            .uri(format!("/invite/{token2}"))
            .header("Accept", "text/html")
            .body(Body::empty())
            .unwrap(),
        peer_localhost(),
    );
    let resp = router2.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .expect("Location header")
        .to_owned();
    // Tokens are AEAD envelopes with random nonces, so compare the decoded
    // claim rather than the exact ciphertext string.
    assert!(
        location.starts_with("/view/tqr1."),
        "redirect must use encrypted ref: {location}"
    );
    let location_ref = location.trim_start_matches("/view/");
    assert_eq!(
        decode_asset_ref(&context2, location_ref).expect("decode location ref"),
        asset_id2
    );
    assert!(
        !location.contains(&format!("/view/{asset_id2}")),
        "redirect must not use raw id: {location}"
    );
}
