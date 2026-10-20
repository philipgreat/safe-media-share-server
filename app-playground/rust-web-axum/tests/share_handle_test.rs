//! Share handle service tests (durable `SHD-` tokens, [S2]).

use media_guard_service_core::{service_runtime_from_pool, E};
use media_guard_service_core_web_axum::service;

fn temp_db_url(dir: &tempfile::TempDir) -> String {
    format!("sqlite://{}/media-guard.db", dir.path().display())
}

async fn setup() -> (
    tempfile::TempDir,
    teaql_runtime::UserContext,
    u64,
    u64,
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
    (dir, context, library_id, asset_id)
}

#[test]
fn generate_handle_token_has_shd_prefix_and_24_hex() {
    let token = service::generate_handle_token();
    assert!(token.starts_with("SHD-"), "token={token}");
    let hex = token.strip_prefix("SHD-").unwrap();
    assert_eq!(hex.len(), 24, "token={token}");
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()), "token={token}");
}

#[tokio::test]
async fn create_media_asset_auto_creates_share_handle() {
    let (_dir, context, _lib, asset_id) = setup().await;

    let handle = service::ensure_share_handle(&context, 1, asset_id)
        .await
        .expect("ensure");
    let token = E::share_handle(&handle)
        .get_handle_token()
        .eval()
        .expect("token");
    assert!(token.starts_with("SHD-"), "token={token}");
    assert!(E::share_handle(&handle).get_active().eval().unwrap_or(false));
    assert_eq!(
        E::share_handle(&handle).get_media_asset_id().eval().unwrap_or(0),
        asset_id
    );
}

#[tokio::test]
async fn ensure_share_handle_is_idempotent() {
    let (_dir, context, library_id, asset_id) = setup().await;

    // create_media_asset already ensured one; a second call returns the same row.
    let first = service::ensure_share_handle(&context, library_id, asset_id)
        .await
        .expect("first");
    let second = service::ensure_share_handle(&context, library_id, asset_id)
        .await
        .expect("second");
    assert_eq!(
        E::share_handle(&first).get_id().eval(),
        E::share_handle(&second).get_id().eval()
    );
    assert_eq!(
        E::share_handle(&first).get_handle_token().eval(),
        E::share_handle(&second).get_handle_token().eval()
    );
}

#[tokio::test]
async fn load_share_handle_by_token_round_trip() {
    let (_dir, context, library_id, asset_id) = setup().await;

    let handle = service::ensure_share_handle(&context, library_id, asset_id)
        .await
        .expect("ensure");
    let token = E::share_handle(&handle)
        .get_handle_token()
        .eval()
        .expect("token");

    let loaded = service::load_share_handle_by_token(&context, &token)
        .await
        .expect("load")
        .expect("found");
    assert_eq!(
        E::share_handle(&loaded).get_id().eval(),
        E::share_handle(&handle).get_id().eval()
    );

    let missing = service::load_share_handle_by_token(&context, "SHD-deadbeefdeadbeefdeadbeef")
        .await
        .expect("load");
    assert!(missing.is_none());
}

#[tokio::test]
async fn set_share_handle_active_toggles_and_ensure_recreates() {
    let (_dir, context, library_id, asset_id) = setup().await;

    let handle = service::ensure_share_handle(&context, library_id, asset_id)
        .await
        .expect("ensure");
    let handle_id = E::share_handle(&handle).get_id().eval().unwrap_or(0);
    let first_token = E::share_handle(&handle)
        .get_handle_token()
        .eval()
        .expect("token");

    service::set_share_handle_active(&context, handle_id, false)
        .await
        .expect("deactivate");
    let loaded = service::load_share_handle_by_token(&context, &first_token)
        .await
        .expect("load")
        .expect("still resolvable while inactive");
    assert!(!E::share_handle(&loaded).get_active().eval().unwrap_or(true));

    // No active handle remains: ensure mints a fresh one.
    let fresh = service::ensure_share_handle(&context, library_id, asset_id)
        .await
        .expect("recreate");
    assert_ne!(
        E::share_handle(&fresh).get_id().eval(),
        Some(handle_id),
        "inactive handle must not be reused"
    );
    assert_ne!(
        E::share_handle(&fresh).get_handle_token().eval(),
        Some(first_token.clone())
    );

    service::set_share_handle_active(&context, handle_id, true)
        .await
        .expect("reactivate");
    let loaded = service::load_share_handle_by_token(&context, &first_token)
        .await
        .expect("load")
        .expect("found");
    assert!(E::share_handle(&loaded).get_active().eval().unwrap_or(false));
}

#[tokio::test]
async fn set_share_handle_active_unknown_id_errors() {
    let (_dir, context, _lib, _asset) = setup().await;
    let err = service::set_share_handle_active(&context, 999_999, false)
        .await
        .expect_err("unknown id");
    assert!(err.to_string().contains("share handle not found"));
}
