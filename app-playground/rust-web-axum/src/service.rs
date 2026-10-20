
//! Domain operations through generated TeaQL Q/E APIs only.

use chrono::{Duration, NaiveDateTime, Utc};
use media_guard_service_core::teaql_core::Entity;
use media_guard_service_core::{
    AuditedSave, E, InviteCode, IpAccessGrant, MediaAsset, MediaLibrary, Q, ShareHandle,
    TeaqlRuntime,
};

use crate::proxy::ProxyTrustConfig;

pub const DEFAULT_GRANT_DAYS: i64 = 7;

pub fn now_stamp() -> String {
    Utc::now().naive_utc().format("%Y-%m-%dT%H:%M:%S").to_string()
}

pub fn default_expire_at() -> String {
    (Utc::now().naive_utc() + Duration::days(DEFAULT_GRANT_DAYS))
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string()
}

fn is_expired(expire_at: &str) -> bool {
    match NaiveDateTime::parse_from_str(expire_at, "%Y-%m-%dT%H:%M:%S") {
        Ok(ts) => Utc::now().naive_utc() > ts,
        Err(_) => true,
    }
}

/// Random-looking invite token (timestamp mixed with address entropy).
pub fn generate_invite_token() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut h = DefaultHasher::new();
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0)
        .hash(&mut h);
    std::process::id().hash(&mut h);
    let a = h.finish();

    let mut h2 = DefaultHasher::new();
    a.hash(&mut h2);
    std::thread::current().id().hash(&mut h2);
    let b = h2.finish();

    format!("INV-{a:016x}{b:08x}")
}

/// Durable share handle token (`SHD-` + exactly 24 hex chars), same entropy
/// mix as [`generate_invite_token`] with the tail masked to 32 bits so the
/// width is stable.
pub fn generate_handle_token() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut h = DefaultHasher::new();
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0)
        .hash(&mut h);
    std::process::id().hash(&mut h);
    let a = h.finish();

    let mut h2 = DefaultHasher::new();
    a.hash(&mut h2);
    std::thread::current().id().hash(&mut h2);
    let b = h2.finish();

    format!("SHD-{a:016x}{:08x}", b as u32)
}

pub async fn ensure_media_library(
    context: &impl TeaqlRuntime,
    library_name: &str,
    root_path: &str,
) -> Result<MediaLibrary, Box<dyn std::error::Error + Send + Sync>> {
    let existing = Q::media_libraries_minimal()
        .order_by_id_asc()
        .limit(1)
        .comment("what: load existing media library root")
        .purpose("why: avoid duplicate library during bootstrap")
        .execute_for_one(context)
        .await?;
    if let Some(lib) = existing {
        return Ok(lib);
    }
    let mut entity = Q::media_libraries()
        .comment("what: initialize media library root")
        .purpose("why: bootstrap media-guard-service")
        .new_entity(context);
    entity.update_library_name(library_name.to_owned());
    entity.update_root_path(root_path.to_owned());
    let persisted = entity
        .audit_as("Bootstrap media library root for media-guard-service")
        .save(context)
        .await?;
    Ok(persisted)
}

pub async fn create_media_asset(
    context: &impl TeaqlRuntime,
    library_id: u64,
    display_name: &str,
    file_path: &str,
    content_type: &str,
    file_size: i64,
    duration_seconds: i64,
) -> Result<MediaAsset, Box<dyn std::error::Error + Send + Sync>> {
    let mut entity = Q::media_assets()
        .comment("what: initialize media asset")
        .purpose("why: register a protectable video resource")
        .new_entity(context);
    entity.update_media_library_id(library_id);
    entity.update_display_name(display_name.to_owned());
    entity.update_file_path(file_path.to_owned());
    entity.update_content_type(content_type.to_owned());
    entity.update_file_size(file_size);
    entity.update_duration_seconds(duration_seconds);
    entity.update_active(true);
    let persisted = entity
        .audit_as("Register media asset for IP-protected playback")
        .save(context)
        .await?;
    // Every asset gets a durable share handle up front ([S2] role split).
    let asset_id = E::media_asset(&persisted).get_id().eval().unwrap_or(0);
    ensure_share_handle(context, library_id, asset_id).await?;
    Ok(persisted)
}

pub async fn create_proxy_trust_rule(
    context: &impl TeaqlRuntime,
    library_id: u64,
    header_name: &str,
    trusted_proxy_cidrs: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut entity = Q::proxy_trust_rules()
        .comment("what: initialize proxy trust rule")
        .purpose("why: support client IP detection behind reverse proxies")
        .new_entity(context);
    entity.update_media_library_id(library_id);
    entity.update_header_name(header_name.to_owned());
    entity.update_trusted_proxy_cidrs(trusted_proxy_cidrs.to_owned());
    entity.update_enabled(true);
    entity
        .audit_as("Enable reverse-proxy client IP recognition")
        .save(context)
        .await?;
    Ok(())
}

pub async fn load_proxy_trust_configs(
    context: &impl TeaqlRuntime,
) -> Result<Vec<ProxyTrustConfig>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = Q::proxy_trust_rules()
        .select_header_name()
        .select_trusted_proxy_cidrs()
        .select_enabled()
        .order_by_id_asc()
        .limit(50)
        .comment("what: load proxy trust rules")
        .purpose("why: resolve client IP behind trusted reverse proxies")
        .execute_for_list(context)
        .await?;
    let mut out = Vec::new();
    for row in rows.iter() {
        out.push(ProxyTrustConfig {
            header_name: E::proxy_trust_rule(row)
                .get_header_name()
                .eval()
                .unwrap_or_default(),
            trusted_proxy_cidrs: E::proxy_trust_rule(row)
                .get_trusted_proxy_cidrs()
                .eval()
                .unwrap_or_default(),
            enabled: E::proxy_trust_rule(row).get_enabled().eval().unwrap_or(false),
        });
    }
    Ok(out)
}

pub async fn grant_ip_access(
    context: &impl TeaqlRuntime,
    library_id: u64,
    media_asset_id: u64,
    client_ip: &str,
    expire_at: Option<&str>,
    grant_note: &str,
) -> Result<IpAccessGrant, Box<dyn std::error::Error + Send + Sync>> {
    let expire_at = expire_at
        .map(|s| s.to_owned())
        .unwrap_or_else(default_expire_at);
    let mut entity = Q::ip_access_grants()
        .comment("what: initialize IP access grant")
        .purpose("why: whitelist a client IP for protected media playback")
        .new_entity(context);
    entity.update_media_library_id(library_id);
    entity.update_media_asset_id(media_asset_id);
    entity.update_grant_status_to_active();
    entity.update_client_ip(client_ip.to_owned());
    entity.update_expire_at(expire_at);
    entity.update_grant_note(grant_note.to_owned());
    let persisted = entity
        .audit_as("Grant time-limited IP access to media asset")
        .save(context)
        .await?;
    Ok(persisted)
}

pub async fn revoke_ip_access(
    context: &impl TeaqlRuntime,
    grant_id: u64,
    reason: &str,
) -> Result<Option<IpAccessGrant>, Box<dyn std::error::Error + Send + Sync>> {
    let loaded = Q::ip_access_grants()
        .with_id_is(grant_id)
        .comment("what: load grant for revocation")
        .purpose("why: remove IP whitelist entry")
        .execute_for_one(context)
        .await?;
    let Some(mut existing) = loaded else {
        return Ok(None);
    };
    existing.update_grant_status_to_revoked();
    let persisted = existing.audit_as(reason).save(context).await?;
    Ok(Some(persisted))
}

/// Revoke every active grant for this IP + asset (used by invite 2nd click).
pub async fn expire_grants_for_ip_asset(
    context: &impl TeaqlRuntime,
    media_asset_id: u64,
    client_ip: &str,
    reason: &str,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let rows = Q::ip_access_grants()
        .with_client_ip_is(client_ip)
        .with_grant_status_is_active()
        .with_media_asset_matching(Q::media_assets_minimal().with_id_is(media_asset_id))
        .order_by_id_asc()
        .limit(20)
        .comment("what: load active grants for IP and asset")
        .purpose("why: expire whitelist entries on invite second click")
        .execute_for_list(context)
        .await?;
    let mut n = 0;
    for row in rows.iter() {
        let id = E::ip_access_grant(row).get_id().eval().unwrap_or(0);
        if revoke_ip_access(context, id, reason).await?.is_some() {
            n += 1;
        }
    }
    Ok(n)
}

pub async fn is_ip_allowed(
    context: &impl TeaqlRuntime,
    media_asset_id: u64,
    client_ip: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let now = now_stamp();
    let rows = Q::ip_access_grants_minimal()
        .with_client_ip_is(client_ip)
        .with_grant_status_is_active()
        .with_expire_at_greater_than(now)
        .with_media_asset_matching(Q::media_assets_minimal().with_id_is(media_asset_id))
        .order_by_id_desc()
        .limit(5)
        .comment("what: check active IP whitelist entry for media asset")
        .purpose("why: authorize protected video playback")
        .execute_for_list(context)
        .await?;
    Ok(!rows.is_empty())
}

pub async fn load_active_asset(
    context: &impl TeaqlRuntime,
    media_asset_id: u64,
) -> Result<Option<MediaAsset>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = Q::media_assets()
        .with_id_is(media_asset_id)
        .which_are_active()
        .order_by_id_asc()
        .limit(1)
        .comment("what: load active media asset for playback")
        .purpose("why: serve protected media stream")
        .execute_for_list(context)
        .await?;
    Ok(rows.into_iter().next())
}

/// List active media assets for the library UI.
pub async fn list_active_assets(
    context: &impl TeaqlRuntime,
) -> Result<Vec<AssetView>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = Q::media_assets()
        .which_are_active()
        .order_by_id_asc()
        .limit(200)
        .comment("what: list active media assets for browse UI")
        .purpose("why: render media library index")
        .execute_for_list(context)
        .await?;
    Ok(rows.iter().map(asset_view).collect())
}

/// Guess Content-Type from file extension when the model value is empty.
pub fn detect_content_type(file_path: &str, declared: &str) -> String {
    if !declared.is_empty() && declared != "application/octet-stream" {
        return declared.to_owned();
    }
    let lower = file_path.to_ascii_lowercase();
    if lower.ends_with(".mp4") || lower.ends_with(".m4v") {
        "video/mp4".into()
    } else if lower.ends_with(".webm") {
        "video/webm".into()
    } else if lower.ends_with(".mp3") {
        "audio/mpeg".into()
    } else if lower.ends_with(".wav") {
        "audio/wav".into()
    } else if lower.ends_with(".png") {
        "image/png".into()
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg".into()
    } else if lower.ends_with(".gif") {
        "image/gif".into()
    } else if lower.ends_with(".webp") {
        "image/webp".into()
    } else if lower.ends_with(".pdf") {
        "application/pdf".into()
    } else if lower.ends_with(".txt") || lower.ends_with(".md") || lower.ends_with(".log") {
        "text/plain; charset=utf-8".into()
    } else if lower.ends_with(".html") || lower.ends_with(".htm") {
        "text/html; charset=utf-8".into()
    } else if lower.ends_with(".csv") {
        "text/csv; charset=utf-8".into()
    } else if lower.ends_with(".json") {
        "application/json".into()
    } else if lower.ends_with(".zip") {
        "application/zip".into()
    } else if lower.ends_with(".tar") {
        "application/x-tar".into()
    } else if lower.ends_with(".gz") || lower.ends_with(".tgz") {
        "application/gzip".into()
    } else if lower.ends_with(".7z") {
        "application/x-7z-compressed".into()
    } else {
        "application/octet-stream".into()
    }
}

/// True when the type should force a browser download (docs/archives).
pub fn prefers_download(file_path: &str, content_type: &str) -> bool {
    let ct = detect_content_type(file_path, content_type);
    let inline = ct.starts_with("video/")
        || ct.starts_with("audio/")
        || ct.starts_with("image/")
        || ct == "application/pdf"
        || ct.starts_with("text/html");
    !inline
}

/// Filename for Content-Disposition.
pub fn download_filename(display_name: &str, file_path: &str) -> String {
    let from_path = std::path::Path::new(file_path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .filter(|s| !s.is_empty());
    if !display_name.is_empty() {
        // Keep an extension from the path when the display name has none.
        if std::path::Path::new(display_name).extension().is_none() {
            if let Some(ext) = std::path::Path::new(file_path).extension() {
                return format!("{display_name}.{}", ext.to_string_lossy());
            }
        }
        return display_name.to_owned();
    }
    from_path.unwrap_or_else(|| "download".to_owned())
}

pub struct AssetView {
    pub id: u64,
    pub library_id: u64,
    pub display_name: String,
    pub file_path: String,
    pub content_type: String,
    pub file_size: i64,
}

pub fn asset_view(asset: &MediaAsset) -> AssetView {
    AssetView {
        id: E::media_asset(asset).get_id().eval().unwrap_or(0),
        library_id: E::media_asset(asset)
            .get_media_library_id()
            .eval()
            .unwrap_or(0),
        display_name: E::media_asset(asset)
            .get_display_name()
            .eval()
            .unwrap_or_default(),
        file_path: E::media_asset(asset)
            .get_file_path()
            .eval()
            .unwrap_or_default(),
        content_type: E::media_asset(asset)
            .get_content_type()
            .eval()
            .unwrap_or_default(),
        file_size: E::media_asset(asset).get_file_size().eval().unwrap_or(0),
    }
}

pub fn grant_is_still_valid(expire_at: &str, status_is_active: bool) -> bool {
    status_is_active && !is_expired(expire_at)
}

// ---------- Invite codes ----------
//
// Specific URL only: GET /invite/{invite_token}
// 1st click (FRESH  -> CLAIMED): add client IP to whitelist (7-day grant)
// 2nd click (CLAIMED -> SPENT) : expire that IP's whitelist entry
// further clicks (SPENT)       : rejected; code cannot be reused

pub async fn create_invite(
    context: &impl TeaqlRuntime,
    library_id: u64,
    media_asset_id: u64,
    grant_note: &str,
) -> Result<InviteCode, Box<dyn std::error::Error + Send + Sync>> {
    let mut entity = Q::invite_codes()
        .comment("what: initialize one-shot invite code")
        .purpose("why: share a specific URL that admits one IP to the whitelist")
        .new_entity(context);
    entity.update_media_library_id(library_id);
    entity.update_media_asset_id(media_asset_id);
    entity.update_invite_status_to_fresh();
    entity.update_invite_token(generate_invite_token());
    entity.update_first_client_ip("0.0.0.0".to_owned());
    entity.update_first_click_at("1970-01-01T00:00:00".to_owned());
    entity.update_second_client_ip("0.0.0.0".to_owned());
    entity.update_second_click_at("1970-01-01T00:00:00".to_owned());
    entity.update_grant_note(grant_note.to_owned());
    let persisted = entity
        .audit_as("Create one-shot invite link for protected media")
        .save(context)
        .await?;
    Ok(persisted)
}

pub async fn load_invite_by_token(
    context: &impl TeaqlRuntime,
    invite_token: &str,
) -> Result<Option<InviteCode>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = Q::invite_codes()
        .with_invite_token_is(invite_token)
        .order_by_id_desc()
        .limit(1)
        .comment("what: load invite code by secret token")
        .purpose("why: redeem invite from its dedicated URL")
        .execute_for_list(context)
        .await?;
    Ok(rows.into_iter().next())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InviteOutcome {
    /// First click: IP joined the whitelist.
    Joined {
        invite_id: u64,
        media_asset_id: u64,
        client_ip: String,
        expire_at: String,
    },
    /// Second click: IP left the whitelist.
    ExpiredFromWhitelist {
        invite_id: u64,
        media_asset_id: u64,
        client_ip: String,
        revoked_grants: usize,
    },
    /// Code already spent; cannot be used again.
    AlreadySpent { invite_id: u64 },
}

/// Load one invite by token in a given lifecycle state.
async fn load_invite_in_state(
    context: &impl TeaqlRuntime,
    invite_token: &str,
    state: InviteState,
) -> Result<Option<InviteCode>, Box<dyn std::error::Error + Send + Sync>> {
    let q = Q::invite_codes().with_invite_token_is(invite_token);
    let q = match state {
        InviteState::Fresh => q.with_invite_status_is_fresh(),
        InviteState::Claimed => q.with_invite_status_is_claimed(),
        InviteState::Spent => q.with_invite_status_is_spent(),
    };
    let rows = q
        .order_by_id_desc()
        .limit(1)
        .comment("what: load invite code by token and lifecycle state")
        .purpose("why: apply one-shot invite click rules")
        .execute_for_list(context)
        .await?;
    Ok(rows.into_iter().next())
}

#[derive(Clone, Copy)]
enum InviteState {
    Fresh,
    Claimed,
    Spent,
}

fn invite_ids(invite: &InviteCode) -> (u64, u64, u64) {
    let invite_id = E::invite_code(invite).get_id().eval().unwrap_or(0);
    let library_id = E::invite_code(invite)
        .get_media_library_id()
        .eval()
        .unwrap_or(0);
    let media_asset_id = E::invite_code(invite)
        .get_media_asset_id()
        .eval()
        .unwrap_or(0);
    (invite_id, library_id, media_asset_id)
}

/// Apply the invite click rules for `invite_token` from `client_ip`.
///
/// - FRESH   + click → add IP to whitelist (CLAIMED)
/// - CLAIMED + click → expire IP from whitelist (SPENT)
/// - SPENT   + click → rejected; the code cannot be used again
pub async fn redeem_invite(
    context: &impl TeaqlRuntime,
    invite_token: &str,
    client_ip: &str,
) -> Result<InviteOutcome, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(invite) = load_invite_in_state(context, invite_token, InviteState::Spent).await? {
        let (invite_id, _, _) = invite_ids(&invite);
        return Ok(InviteOutcome::AlreadySpent { invite_id });
    }

    if let Some(mut invite) = load_invite_in_state(context, invite_token, InviteState::Claimed).await?
    {
        let (invite_id, _, media_asset_id) = invite_ids(&invite);
        let revoked = expire_grants_for_ip_asset(
            context,
            media_asset_id,
            client_ip,
            "Invite second click: expire IP from whitelist",
        )
        .await?;
        invite.update_invite_status_to_spent();
        invite.update_second_client_ip(client_ip.to_owned());
        invite.update_second_click_at(now_stamp());
        invite
            .audit_as("Invite second click: expire whitelist entry")
            .save(context)
            .await?;
        return Ok(InviteOutcome::ExpiredFromWhitelist {
            invite_id,
            media_asset_id,
            client_ip: client_ip.to_owned(),
            revoked_grants: revoked,
        });
    }

    let Some(mut invite) = load_invite_in_state(context, invite_token, InviteState::Fresh).await?
    else {
        return Err("invite not found".into());
    };
    let (invite_id, library_id, media_asset_id) = invite_ids(&invite);
    let grant = grant_ip_access(
        context,
        library_id,
        media_asset_id,
        client_ip,
        None,
        "Joined whitelist via one-shot invite URL",
    )
    .await?;
    let expire_at = E::ip_access_grant(&grant)
        .get_expire_at()
        .eval()
        .unwrap_or_default();
    invite.update_invite_status_to_claimed();
    invite.update_first_client_ip(client_ip.to_owned());
    invite.update_first_click_at(now_stamp());
    invite
        .audit_as("Invite first click: join IP whitelist")
        .save(context)
        .await?;

    Ok(InviteOutcome::Joined {
        invite_id,
        media_asset_id,
        client_ip: client_ip.to_owned(),
        expire_at,
    })
}

// ---------- Share handles ([S2] durable SHD- token, not session refs) ----------
//
// Durable DB-backed open-link tokens. Session-local `tqr1.` fingerprinted
// refs never appear in shareable URLs.

/// Active share handle for the asset, created on first call (idempotent).
pub async fn ensure_share_handle(
    context: &impl TeaqlRuntime,
    library_id: u64,
    media_asset_id: u64,
) -> Result<ShareHandle, Box<dyn std::error::Error + Send + Sync>> {
    let existing = Q::share_handles()
        .which_are_active()
        .with_media_asset_matching(Q::media_assets_minimal().with_id_is(media_asset_id))
        .order_by_id_desc()
        .limit(1)
        .comment("what: load active share handle for media asset")
        .purpose("why: reuse the durable share handle across sessions")
        .execute_for_one(context)
        .await?;
    if let Some(handle) = existing {
        return Ok(handle);
    }
    let mut entity = Q::share_handles()
        .comment("what: initialize durable share handle")
        .purpose("why: provide a stable cross-session open link")
        .new_entity(context);
    entity.update_media_library_id(library_id);
    entity.update_media_asset_id(media_asset_id);
    entity.update_handle_token(generate_handle_token());
    entity.update_active(true);
    entity.update_handle_note("Auto-created durable share handle".to_owned());
    let persisted = entity
        .audit_as("Create durable share handle for media asset")
        .save(context)
        .await?;
    Ok(persisted)
}

/// Load one share handle by its secret token (active or not).
pub async fn load_share_handle_by_token(
    context: &impl TeaqlRuntime,
    handle_token: &str,
) -> Result<Option<ShareHandle>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = Q::share_handles()
        .with_handle_token_is(handle_token)
        .order_by_id_desc()
        .limit(1)
        .comment("what: load share handle by secret token")
        .purpose("why: resolve durable share link")
        .execute_for_list(context)
        .await?;
    Ok(rows.into_iter().next())
}

/// Enable or disable a share handle by id.
pub async fn set_share_handle_active(
    context: &impl TeaqlRuntime,
    handle_id: u64,
    active: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let Some(mut existing) = Q::share_handles()
        .with_id_is(handle_id)
        .comment("what: load share handle for activation change")
        .purpose("why: enable or disable a durable share link")
        .execute_for_one(context)
        .await?
    else {
        return Err("share handle not found".into());
    };
    existing.update_active(active);
    let reason = if active {
        "Reactivate durable share handle"
    } else {
        "Deactivate durable share handle"
    };
    existing.audit_as(reason).save(context).await?;
    Ok(())
}

// ---------- Admin dashboard listings ----------

/// One row of the admin whitelist table.
#[derive(Debug, Clone)]
pub struct WhitelistRow {
    pub grant_id: u64,
    pub client_ip: String,
    pub expire_at: String,
    pub grant_note: String,
    pub media_asset_id: u64,
    pub media_name: String,
    pub media_file: String,
}

/// Active, non-expired IP grants (current whitelist).
pub async fn list_current_whitelist(
    context: &impl TeaqlRuntime,
) -> Result<Vec<WhitelistRow>, Box<dyn std::error::Error + Send + Sync>> {
    let now = now_stamp();
    let rows = Q::ip_access_grants()
        .with_grant_status_is_active()
        .with_expire_at_greater_than(now)
        .select_media_asset()
        .order_by_id_desc()
        .limit(200)
        .comment("what: list active IP whitelist grants")
        .purpose("why: admin whitelist dashboard")
        .execute_for_list(context)
        .await?;
    let mut out = Vec::new();
    for row in rows.iter() {
        let grant_id = E::ip_access_grant(row).get_id().eval().unwrap_or(0);
        let client_ip = E::ip_access_grant(row).get_client_ip().eval().unwrap_or_default();
        let expire_at = E::ip_access_grant(row).get_expire_at().eval().unwrap_or_default();
        let grant_note = E::ip_access_grant(row).get_grant_note().eval().unwrap_or_default();
        let media_asset_id = E::ip_access_grant(row).get_media_asset_id().eval().unwrap_or(0);
        let mut media_name = String::new();
        let mut media_file = String::new();
        if let Some(asset) = E::ip_access_grant(row).get_media_asset().eval() {
            media_name = E::media_asset(asset).get_display_name().eval().unwrap_or_default();
            media_file = E::media_asset(asset).get_file_path().eval().unwrap_or_default();
        }
        out.push(WhitelistRow {
            grant_id,
            client_ip,
            expire_at,
            grant_note,
            media_asset_id,
            media_name,
            media_file,
        });
    }
    Ok(out)
}

/// Invite codes killed by the second click (SPENT).
#[derive(Debug, Clone)]
pub struct SpentInviteRow {
    pub invite_id: u64,
    pub invite_token: String,
    pub first_client_ip: String,
    pub first_click_at: String,
    pub second_client_ip: String,
    pub second_click_at: String,
    pub media_asset_id: u64,
    pub media_name: String,
    pub media_file: String,
    pub grant_note: String,
}

pub async fn list_spent_invites(
    context: &impl TeaqlRuntime,
) -> Result<Vec<SpentInviteRow>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = Q::invite_codes()
        .with_invite_status_is_spent()
        .select_media_asset()
        .order_by_id_desc()
        .limit(200)
        .comment("what: list spent invite codes")
        .purpose("why: admin review of invites disposed by second click")
        .execute_for_list(context)
        .await?;
    let mut out = Vec::new();
    for row in rows.iter() {
        let invite_id = E::invite_code(row).get_id().eval().unwrap_or(0);
        let invite_token = E::invite_code(row).get_invite_token().eval().unwrap_or_default();
        let first_client_ip = E::invite_code(row).get_first_client_ip().eval().unwrap_or_default();
        let first_click_at = E::invite_code(row).get_first_click_at().eval().unwrap_or_default();
        let second_client_ip = E::invite_code(row).get_second_client_ip().eval().unwrap_or_default();
        let second_click_at = E::invite_code(row).get_second_click_at().eval().unwrap_or_default();
        let media_asset_id = E::invite_code(row).get_media_asset_id().eval().unwrap_or(0);
        let grant_note = E::invite_code(row).get_grant_note().eval().unwrap_or_default();
        let mut media_name = String::new();
        let mut media_file = String::new();
        if let Some(asset) = E::invite_code(row).get_media_asset().eval() {
            media_name = E::media_asset(asset).get_display_name().eval().unwrap_or_default();
            media_file = E::media_asset(asset).get_file_path().eval().unwrap_or_default();
        }
        out.push(SpentInviteRow {
            invite_id,
            invite_token,
            first_client_ip,
            first_click_at,
            second_client_ip,
            second_click_at,
            media_asset_id,
            media_name,
            media_file,
            grant_note,
        });
    }
    Ok(out)
}

/// Fresh (unused) invite tokens: (invite_id, token, media_asset_id).
pub async fn list_fresh_invites(
    context: &impl TeaqlRuntime,
) -> Result<Vec<(u64, String, u64)>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = Q::invite_codes()
        .with_invite_status_is_fresh()
        .order_by_id_desc()
        .limit(200)
        .comment("what: list unused invite codes")
        .purpose("why: admin copy invite code")
        .execute_for_list(context)
        .await?;
    Ok(rows
        .iter()
        .map(|row| {
            (
                E::invite_code(row).get_id().eval().unwrap_or(0),
                E::invite_code(row).get_invite_token().eval().unwrap_or_default(),
                E::invite_code(row).get_media_asset_id().eval().unwrap_or(0),
            )
        })
        .collect())
}

// ---------- App settings (stored in DB under media_library) ----------

pub const SETTING_EXTERNAL_BASE_URL: &str = "external_base_url";

/// Read one setting by key.
pub async fn get_setting_value(
    context: &impl TeaqlRuntime,
    setting_key: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = Q::app_settings()
        .with_setting_key_is(setting_key)
        .order_by_id_desc()
        .limit(1)
        .comment("what: load app setting by key")
        .purpose("why: resolve runtime configuration from database")
        .execute_for_list(context)
        .await?;
    Ok(rows
        .iter()
        .next()
        .and_then(|row| E::app_setting(row).get_setting_value().eval()))
}

/// Insert or update a setting row.
pub async fn upsert_setting(
    context: &impl TeaqlRuntime,
    library_id: u64,
    setting_key: &str,
    setting_value: &str,
    setting_name: &str,
    setting_note: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let existing = Q::app_settings()
        .with_setting_key_is(setting_key)
        .order_by_id_desc()
        .limit(1)
        .comment("what: load existing app setting for upsert")
        .purpose("why: persist runtime configuration to database")
        .execute_for_one(context)
        .await?;
    if let Some(mut row) = existing {
        row.update_setting_value(setting_value.to_owned());
        if !setting_name.is_empty() {
            row.update_setting_name(setting_name.to_owned());
        }
        if !setting_note.is_empty() {
            row.update_setting_note(setting_note.to_owned());
        }
        row.audit_as("Update application setting")
            .save(context)
            .await?;
        return Ok(());
    }
    let mut entity = Q::app_settings()
        .comment("what: initialize application setting")
        .purpose("why: persist runtime configuration to database")
        .new_entity(context);
    entity.update_media_library_id(library_id);
    entity.update_setting_key(setting_key.to_owned());
    entity.update_setting_value(setting_value.to_owned());
    entity.update_setting_name(setting_name.to_owned());
    entity.update_setting_note(setting_note.to_owned());
    entity
        .audit_as("Create application setting")
        .save(context)
        .await?;
    Ok(())
}

/// List all settings for admin UI.
pub async fn list_settings(
    context: &impl TeaqlRuntime,
) -> Result<Vec<(String, String, String, String)>, Box<dyn std::error::Error + Send + Sync>> {
    // (key, value, name, note)
    let rows = Q::app_settings()
        .order_by_setting_key_asc()
        .limit(200)
        .comment("what: list application settings")
        .purpose("why: admin settings dashboard")
        .execute_for_list(context)
        .await?;
    Ok(rows
        .iter()
        .map(|row| {
            (
                E::app_setting(row).get_setting_key().eval().unwrap_or_default(),
                E::app_setting(row).get_setting_value().eval().unwrap_or_default(),
                E::app_setting(row).get_setting_name().eval().unwrap_or_default(),
                E::app_setting(row).get_setting_note().eval().unwrap_or_default(),
            )
        })
        .collect())
}

/// External base URL from DB (empty means use browser origin).
pub async fn external_base_url(
    context: &impl TeaqlRuntime,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    Ok(get_setting_value(context, SETTING_EXTERNAL_BASE_URL)
        .await?
        .unwrap_or_default())
}
