
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use media_guard_service_core_web_axum::handlers::{app, AppState};
use media_guard_service_core_web_axum::{build_request_context, service};
use teaql_runtime::UserContext;

/// Seed library, demo asset, proxy rule, and one demo invite when empty.
async fn seed_if_empty(
    context: &UserContext,
    media_root: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let library = service::ensure_media_library(context, "Home Video Library", media_root).await?;
    let library_id = media_guard_service_core::E::media_library(&library)
        .get_id()
        .eval()
        .unwrap_or(0);

    let existing = media_guard_service_core::Q::media_assets_minimal()
        .order_by_id_asc()
        .limit(1)
        .comment("what: check whether demo asset exists")
        .purpose("why: avoid duplicate seed rows")
        .execute_for_one(context)
        .await?;
    if existing.is_none() {
        let asset = service::create_media_asset(
            context,
            library_id,
            "Demo Video",
            "videos/demo.mp4",
            "video/mp4",
            1024,
            1,
        )
        .await?;
        service::create_proxy_trust_rule(
            context,
            library_id,
            "X-Forwarded-For",
            "10.0.0.0/8,127.0.0.1",
        )
        .await?;
        let asset_id = media_guard_service_core::E::media_asset(&asset)
            .get_id()
            .eval()
            .unwrap_or(1);
        let invite = service::create_invite(
            context,
            library_id,
            asset_id,
            "Demo one-shot invite for seed asset",
        )
        .await?;
        let token = media_guard_service_core::E::invite_code(&invite)
            .get_invite_token()
            .eval()
            .unwrap_or_default();
        let invite_id = media_guard_service_core::E::invite_code(&invite)
            .get_id()
            .eval()
            .unwrap_or(0);
        println!("seed invite id={invite_id} url=/invite/{token}");
    }
    Ok(())
}

/// Open the SQLite pool from `MEDIA_GUARD_SERVICE_CORE_DATABASE_URL`.
fn open_pool_from_env() -> Result<
    media_guard_service_core::DataServicePool,
    Box<dyn std::error::Error + Send + Sync>,
> {
    let url = std::env::var(media_guard_service_core::DATABASE_URL_ENV)?;
    let sanitized = url
        .strip_prefix("sqlite:")
        .unwrap_or(&url)
        .trim_start_matches("//");
    let pure_file_path = sanitized.split('?').next().unwrap_or(sanitized);
    let path = std::path::Path::new(pure_file_path);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let conn = rusqlite::Connection::open(pure_file_path)?;
    Ok(Arc::new(Mutex::new(conn)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[{}] Starting media-guard-service (rust-web-axum)...",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
    );

    let pool = open_pool_from_env()?;
    let runtime = media_guard_service_core::service_runtime_from_pool(pool.clone()).await?;
    runtime.ensure_schema().await?;

    let media_root = std::env::var("MEDIA_ROOT").unwrap_or_else(|_| "/srv/media".to_owned());

    let bootstrap_ctx = build_request_context(&pool);
    seed_if_empty(&bootstrap_ctx, &media_root).await?;

    let bind_addr = std::env::var("MEDIA_GUARD_SERVICE_CORE_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_owned());
    let addr: SocketAddr = bind_addr.parse()?;

    let state = AppState {
        media_root: PathBuf::from(&media_root),
        pool,
    };
    let router = app(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on http://{addr}  media_root={media_root}");
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
