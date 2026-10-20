# Encrypted Media Asset IDs Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use compose:subagent (recommended) or compose:execute to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Guest-facing media URLs and JSON expose TeaQL encrypted entity refs (`tqr1.…`) instead of raw `media_asset.id`.

**Architecture:** Install `AeadEntityReferenceCodec` on every request `UserContext`. Encode asset ids on the way out; decode path params on the way in. A small `entity_ref` module owns key loading and the `MediaAsset` / `media-access` constants.

**Tech Stack:** Rust, Axum, `teaql-runtime` 5.0.7 (`AeadEntityReferenceCodec`), React + Vite.

## Global Constraints

- Guest list/view/download/invite redirects must not expose raw numeric asset ids.
- Entity type string is `MediaAsset`; purpose is `media-access`; ref lifetime is 7 days.
- Decode failure → HTTP 400 `invalid media reference` with no internal id in the body.
- Admin JSON may keep `id` for operators; guest `GET /api/media` and `GET /api/media/{ref}` use `ref` only.
- Do not parse raw `u64` as a fallback in guest media routes.
- Generated `lib/` / `rust-lib-core` must not be hand-edited.
- Branch: `feat/encrypted-entity-ids`.
- After `web/src` edits rebuild with `PATH=/home/philip/.local/node/bin:$PATH npm run build`.
- Domain Q/E calls stay on generated APIs (`.comment` + `.purpose` on executes; writes `audit_as`).

---

### Task 1: Branch + entity_ref module + codec wiring

**Covers:** [S3], [S7]

**Files:**
- Create: `app-playground/rust-web-axum/src/entity_ref.rs`
- Modify: `app-playground/rust-web-axum/src/lib.rs`
- Modify: `app-playground/rust-web-axum/src/handlers.rs:23-34` (AppState ContextProvider)
- Test: `app-playground/rust-web-axum/tests/entity_ref_test.rs`

**Interfaces:**
- Consumes: `teaql_runtime::{AeadEntityReferenceCodec, UserContext}`
- Produces:
  - `entity_ref::ENTITY_TYPE: &str = "MediaAsset"`
  - `entity_ref::PURPOSE: &str = "media-access"`
  - `entity_ref::REF_LIFETIME_SECS: u64 = 7 * 24 * 3600`
  - `entity_ref::shared_codec() -> Arc<AeadEntityReferenceCodec>`
  - `entity_ref::encode_asset_ref(context: &UserContext, asset_id: u64) -> Result<String, String>`
  - `entity_ref::decode_asset_ref(context: &UserContext, token: &str) -> Result<u64, String>`
  - `build_request_context` installs the shared codec via `set_entity_reference_codec`

- [ ] **Step 1: Create branch**

```bash
cd /home/philip/githome/safe-media-share-server
git checkout -b feat/encrypted-entity-ids
```

- [ ] **Step 2: Write failing unit test for encode/decode**

Create `app-playground/rust-web-axum/tests/entity_ref_test.rs`:

```rust
//! Entity reference codec smoke tests (guest ID hiding).

use std::sync::Arc;
use std::time::Duration;

use teaql_runtime::{
    AeadEntityReferenceCodec, EntityReferenceCodec, EntityReferenceTokenError,
};

fn codec() -> AeadEntityReferenceCodec {
    AeadEntityReferenceCodec::new(1, [(1u32, vec![7u8; 32])]).expect("codec")
}

#[test]
fn encode_decode_round_trip_hides_raw_id() {
    let codec = codec();
    let token = codec
        .encode("MediaAsset", 42, 0, "media-access", Duration::from_secs(60))
        .expect("encode");
    assert!(token.starts_with("tqr1."), "token={token}");
    assert!(!token.contains("42"), "raw id leaked in token");
    let claims = codec
        .decode(&token, "MediaAsset", "media-access")
        .expect("decode");
    assert_eq!(claims.id, 42);
    assert_eq!(claims.entity_type, "MediaAsset");
    assert_eq!(claims.purpose, "media-access");
}

#[test]
fn decode_rejects_wrong_purpose() {
    let codec = codec();
    let token = codec
        .encode("MediaAsset", 42, 0, "media-access", Duration::from_secs(60))
        .expect("encode");
    let err = codec
        .decode(&token, "MediaAsset", "other")
        .expect_err("purpose must match");
    assert_eq!(err.code(), "ENTITY_REFERENCE_INVALID");
}

#[test]
fn decode_rejects_garbage() {
    let codec = codec();
    let err = codec
        .decode("tqr1.not-a-token", "MediaAsset", "media-access")
        .expect_err("garbage");
    assert_eq!(err.code(), "ENTITY_REFERENCE_INVALID");
    let _ = EntityReferenceTokenError::invalid();
}
```

- [ ] **Step 3: Run test to verify it compiles/passes (codec itself)**

```bash
cd /home/philip/githome/safe-media-share-server/app-playground/rust-web-axum
cargo test --test entity_ref_test
```

Expected: 3 passed (validates runtime codec API before app wiring).

- [ ] **Step 4: Add `entity_ref` module**

Create `app-playground/rust-web-axum/src/entity_ref.rs`:

```rust
//! Guest-facing media asset references: encrypted tokens only.

use std::sync::{Arc, OnceLock};
use std::time::Duration;

use teaql_runtime::{AeadEntityReferenceCodec, UserContext};

pub const ENTITY_TYPE: &str = "MediaAsset";
pub const PURPOSE: &str = "media-access";
pub const REF_LIFETIME_SECS: u64 = 7 * 24 * 3600;

fn active_key() -> [u8; 32] {
    if let Ok(raw) = std::env::var("MEDIA_GUARD_REF_KEY") {
        let trimmed = raw.trim();
        if let Ok(bytes) = const_hex::decode(trimmed) {
            if let Ok(arr) = <[u8; 32]>::try_from(bytes.as_slice()) {
                return arr;
            }
        }
        if let Ok(bytes) = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            trimmed,
        ) {
            if let Ok(arr) = <[u8; 32]>::try_from(bytes.as_slice()) {
                return arr;
            }
        }
    }
    // Trial fallback: process-local key. Restart invalidates outstanding refs.
    let mut key = [0u8; 32];
    getrandom_fill(&mut key);
    eprintln!(
        "WARN: MEDIA_GUARD_REF_KEY unset; generated ephemeral entity-ref key (trial only)"
    );
    key
}

fn getrandom_fill(buf: &mut [u8]) {
    use std::io::Read;
    let mut f = std::fs::File::open("/dev/urandom").expect("urandom");
    f.read_exact(buf).expect("fill key");
}

pub fn shared_codec() -> Arc<AeadEntityReferenceCodec> {
    static CODEC: OnceLock<Arc<AeadEntityReferenceCodec>> = OnceLock::new();
    CODEC
        .get_or_init(|| {
            let key = active_key();
            Arc::new(
                AeadEntityReferenceCodec::new(1u32, [(1u32, key.to_vec())])
                    .expect("entity reference codec"),
            )
        })
        .clone()
}

pub fn encode_asset_ref(context: &UserContext, asset_id: u64) -> Result<String, String> {
    context
        .encode_entity_reference(
            ENTITY_TYPE,
            asset_id,
            0,
            PURPOSE,
            Duration::from_secs(REF_LIFETIME_SECS),
        )
        .map_err(|e| e.to_string())
}

pub fn decode_asset_ref(context: &UserContext, token: &str) -> Result<u64, String> {
    let claims = context
        .decode_entity_reference(token, ENTITY_TYPE, PURPOSE)
        .map_err(|_| "invalid media reference".to_string())?;
    Ok(claims.id)
}
```

**Note:** If `const_hex` / `base64` are not already dependency names in this crate, parse hex/base64 with a tiny local helper or add the crates that `teaql-runtime` already pulls (`base64`, `hex` if present). Prefer adding to `Cargo.toml`:

```toml
base64 = "0.22"
hex = "0.4"
```

and use `hex::decode` instead of `const_hex::decode` (adjust the import). Keep the helper in this file only.

- [ ] **Step 5: Wire module + context**

In `src/lib.rs` add:

```rust
pub mod entity_ref;
```

Change `build_request_context` to install the codec after existing wiring:

```rust
    context.register_executor(executor.clone());
    context.insert_resource(executor);
    context.set_entity_reference_codec(crate::entity_ref::shared_codec());
    context
```

- [ ] **Step 6: Keep AppState ContextProvider unchanged in shape**

`handlers.rs` `ContextProvider::build_context` already calls `build_request_context`; after Step 5 every request context has the codec. No AppState field required.

- [ ] **Step 7: Run compile + tests**

```bash
cd /home/philip/githome/safe-media-share-server/app-playground/rust-web-axum
cargo test --test entity_ref_test
cargo check
```

Expected: all green.

- [ ] **Step 8: Commit**

```bash
cd /home/philip/githome/safe-media-share-server
git add app-playground/rust-web-axum/src/entity_ref.rs \
        app-playground/rust-web-axum/src/lib.rs \
        app-playground/rust-web-axum/tests/entity_ref_test.rs \
        app-playground/rust-web-axum/Cargo.toml \
        app-playground/rust-web-axum/Cargo.lock
git commit -m "feat: install TeaQL entity reference codec for media asset IDs"
```

---

### Task 2: Guest API and media routes take encrypted refs

**Covers:** [S2], [S4], [S6]

**Files:**
- Modify: `app-playground/rust-web-axum/src/handlers.rs`
  - `api_list_media` (~461-482)
  - `api_media_detail` (~484-507)
  - `serve_media` (~680+)
  - `download_media` (~811+)
  - `redeem_invite_url` redirect + JSON (~546-590)
  - `asset_json` helper (~440-458)
- Test: extend `app-playground/rust-web-axum/tests/invite_test.rs` or new `tests/guest_ref_test.rs`

**Interfaces:**
- Consumes: `entity_ref::{encode_asset_ref, decode_asset_ref}`
- Produces: guest JSON field `ref: String`; path params `Path(asset_ref): Path<String>`; no guest `id` field.

- [ ] **Step 1: Write failing integration test**

Add `app-playground/rust-web-axum/tests/guest_ref_test.rs` (reuse `setup()` patterns from `invite_test.rs`):

```rust
//! Guest media routes must accept encrypted refs and not leak raw ids.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use media_guard_service_core_web_axum::entity_ref::{decode_asset_ref, encode_asset_ref};
use media_guard_service_core_web_axum::handlers::{app, AppState};

// Reuse the same temp-db setup helpers as invite_test.rs (copy the functions
// needed: temp_db_url, setup, peer_localhost). Keep them local to this file.

#[tokio::test]
async fn guest_list_uses_ref_not_raw_id() {
    let (_dir, _ctx, _lib, asset_id, _invite, pool) = setup().await;
    let state = AppState {
        media_root: /* media_root from setup — adjust setup to also return PathBuf */,
        pool,
    };
    let router = app(state);
    let response = router
        .oneshot(
            Request::builder()
                .uri("/api/media")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let text = String::from_utf8_lossy(&bytes);
    assert!(!text.contains(&format!("\"id\":{asset_id}")), "raw id leaked: {text}");
    assert!(text.contains("tqr1."), "expected encrypted ref: {text}");
}

#[tokio::test]
async fn guest_media_stream_accepts_ref() {
    // encode ref with a context that has the codec (build_request_context),
    // GET /media/{ref} with Range, expect 206 and no raw id in errors.
}
```

**Setup adjustment:** Prefer a shared test helper module `tests/common/mod.rs` exporting `setup() -> (TempDir, UserContext, u64 library_id, u64 asset_id, String invite, PathBuf media_root, DataServicePool)`. If that is too invasive for the trial, copy the existing `setup()` into `guest_ref_test.rs` and return `media_root` as well.

- [ ] **Step 2: Run test to verify it fails**

```bash
cd /home/philip/githome/safe-media-share-server/app-playground/rust-web-axum
cargo test --test guest_ref_test
```

Expected: FAIL — list still emits `"id":…` and no `tqr1.`.

- [ ] **Step 3: Encode on the way out**

In `handlers.rs`, add near the top:

```rust
use crate::entity_ref::{decode_asset_ref, encode_asset_ref};
```

Change `asset_json` to accept `asset_id: u64` and emit `ref` instead of raw `id` for guest use. If `asset_json` is shared with admin, split:

```rust
fn guest_asset_json(
    context: &UserContext,
    view: &service::AssetView,
    authorized: bool,
) -> Value {
    let r = encode_asset_ref(context, view.id).unwrap_or_default();
    json!({
        "ref": r,
        "displayName": view.display_name,
        "filePath": view.file_path,
        "contentType": service::detect_content_type(&view.file_path, &view.content_type),
        "fileSize": view.file_size,
        "authorized": authorized,
    })
}
```

(Keep the existing `asset_json` for admin if it is used there; do not remove admin `id`.)

Update `api_list_media` map body:

```rust
                    let r = encode_asset_ref(&context, v.id).unwrap_or_default();
                    json!({
                        "ref": r,
                        "displayName": v.display_name,
                        "filePath": v.file_path,
                        "contentType": service::detect_content_type(&v.file_path, &v.content_type),
                        "fileSize": v.file_size,
                    })
```

Update `api_media_detail` signature and body:

```rust
    Path(asset_ref): Path<String>,
    // ...
    let asset_id = match decode_asset_ref(&context, &asset_ref) {
        Ok(id) => id,
        Err(msg) => {
            return (StatusCode::BAD_REQUEST, WebResponse::<Value>::fail(msg)).into_response();
        }
    };
```

Then load/authorize as today using `asset_id`, respond with `guest_asset_json` (no `id`).

- [ ] **Step 4: Media stream/download accept refs**

Change `serve_media` and `download_media` from `Path(asset_id): Path<u64>` to:

```rust
    Path(asset_ref): Path<String>,
```

at the start:

```rust
    let asset_id = match decode_asset_ref(&context, &asset_ref) {
        Ok(id) => id,
        Err(msg) => {
            return (StatusCode::BAD_REQUEST, WebResponse::<Value>::fail(msg)).into_response();
        }
    };
```

Continue using `asset_id` for whitelist + load. Error messages must use the token or a generic phrase, not `asset {asset_id}`. Replace:

```rust
                "IP {client_ip} is not authorized for asset {asset_id}"
```

with:

```rust
                "IP {client_ip} is not authorized for this media"
```

- [ ] **Step 5: Invite redirect / JSON uses refs**

In `redeem_invite_url` success paths:

```rust
                    .header(axum::http::header::LOCATION, format!(
                        "/view/{}",
                        encode_asset_ref(&context, media_asset_id).unwrap_or_default()
                    ))
```

and JSON fields (both join and expire outcomes that expose `mediaAssetId`):

```rust
                "mediaRef": encode_asset_ref(&context, media_asset_id).unwrap_or_default(),
```

Keep `mediaAssetId` only if admin smoke helpers require it; for guest JSON prefer `mediaRef` only. Update `UserHome` navigation accordingly in Task 3.

- [ ] **Step 6: Run tests**

```bash
cargo test --test guest_ref_test
cargo test
```

Expected: guest ref tests pass; existing invite/range tests updated if they call `/media/{numeric}` (see Step 7).

- [ ] **Step 7: Fix existing tests that use raw ids**

In `tests/invite_test.rs`, wherever a test requests `/media/{id}` or `/api/media/{id}`:

1. Encode with `build_request_context(&pool)` or `encode_asset_ref`.
2. Request `/media/{token}`.

Example helper at bottom of the test file:

```rust
fn asset_ref(pool: &media_guard_service_core::DataServicePool, asset_id: u64) -> String {
    let ctx = media_guard_service_core_web_axum::build_request_context(pool);
    media_guard_service_core_web_axum::entity_ref::encode_asset_ref(&ctx, asset_id)
        .expect("encode ref")
}
```

- [ ] **Step 8: Commit**

```bash
cd /home/philip/githome/safe-media-share-server
git add app-playground/rust-web-axum/src/handlers.rs \
        app-playground/rust-web-axum/tests
git commit -m "feat: guest media routes use encrypted entity refs"
```

---

### Task 3: Frontend carries `ref` only

**Covers:** [S5]

**Files:**
- Modify: `app-playground/rust-web-axum/web/src/pages/UserHome.jsx`
- Modify: `app-playground/rust-web-axum/web/src/pages/ViewPage.jsx`
- Modify: `app-playground/rust-web-axum/web/src/pages/AdminMedia.jsx` (view/download links only)
- Modify: `app-playground/rust-web-axum/web/src/App.jsx` (route param name optional)

**Interfaces:**
- Consumes: API JSON fields `ref`, `mediaRef`; `/api/media/{ref}`; `/media/{ref}`; `/view/{ref}`
- Produces: UI never concatenates numeric ids into media URLs.

- [ ] **Step 1: UserHome list + invite navigate**

In `UserHome.jsx`:

- `key={it.ref}`
- `to={`/view/${encodeURIComponent(it.ref)}`}`
- `href={`/media/${encodeURIComponent(it.ref)}/download`}`
- After join: `navigate(\`/view/${encodeURIComponent(item.mediaRef || item.ref || '')}\`)`  
  If backend only returns `mediaRef` on invite, use that; remove `item.mediaAssetId` from guest navigation.

- [ ] **Step 2: ViewPage uses route token as ref**

`ViewPage.jsx`:

- `const { ref } = useParams()` (or keep `id` name but treat value as opaque token — prefer rename to `ref`).
- `api(\`/api/media/${encodeURIComponent(ref)}\`)`
- media elements: `src={`/media/${encodeURIComponent(ref)}`}`
- download: `/media/${encodeURIComponent(ref)}/download`

`App.jsx`:

```jsx
      <Route path="/view/:ref" element={<ViewPage />} />
```

- [ ] **Step 3: AdminMedia share links**

Where AdminMedia builds `viewUrl` / `downloadUrl` from `it.id`, switch those **link URLs** to `it.ref` if present; keep displaying `it.id` as text for operators. If admin list JSON still only has `id`, add `ref` in `admin_api_media` (encode alongside `id`) so copy links work:

```rust
                    let r = encode_asset_ref(&context, v.id).unwrap_or_default();
                    json!({
                        "id": v.id,
                        "ref": r,
                        // ...
                        "viewUrl": join_external(&base, &format!("/view/{r}")),
                        "downloadUrl": join_external(&base, &format!("/media/{r}/download")),
                        "mediaUrl": join_external(&base, &format!("/media/{r}")),
                    })
```

- [ ] **Step 4: Rebuild frontend**

```bash
cd /home/philip/githome/safe-media-share-server/app-playground/rust-web-axum/web
PATH=/home/philip/.local/node/bin:$PATH npm run build
```

Expected: build succeeds, `web/dist` updated.

- [ ] **Step 5: Commit**

```bash
cd /home/philip/githome/safe-media-share-server
git add app-playground/rust-web-axum/web/src \
        app-playground/rust-web-axum/web/dist \
        app-playground/rust-web-axum/src/handlers.rs
git commit -m "feat: frontend uses encrypted media refs in routes and links"
```

---

### Task 4: End-to-end verification + notes

**Covers:** [S6], [S7]

**Files:**
- Test: `app-playground/rust-web-axum/tests/guest_ref_test.rs` (complete stream/download cases)
- Modify: `docs/session/notes.md` (trial observations)

**Interfaces:**
- Consumes: full stack from Tasks 1–3
- Produces: green `cargo test`, smoke evidence that responses contain `tqr1.` and no guest raw id.

- [ ] **Step 1: Complete guest stream/download tests**

In `guest_ref_test.rs`:

```rust
#[tokio::test]
async fn guest_stream_and_download_work_with_ref() {
    // setup with media file bytes; encode ref; Range request → 206;
    // GET /media/{ref}/download → 200 attachment.
}

#[tokio::test]
async fn garbage_ref_is_bad_request_without_raw_id() {
    // GET /api/media/tqr1.bogus → 400; body must not contain "\"id\":".
}
```

- [ ] **Step 2: Run full test suite**

```bash
cd /home/philip/githome/safe-media-share-server/app-playground/rust-web-axum
cargo test
```

Expected: all tests pass (legacy + new).

- [ ] **Step 3: Manual smoke (optional if server can run)**

```bash
# list should contain tqr1. and not raw guest ids
curl -s http://127.0.0.1:18090/api/media | head
# stream with a ref copied from the list
curl -sI "http://127.0.0.1:18090/media/<ref>" | head
```

Record findings in `docs/session/notes.md`: token shape, URL length, expiry behavior, DX friction.

- [ ] **Step 4: Commit**

```bash
cd /home/philip/githome/safe-media-share-server
git add app-playground/rust-web-axum/tests docs/session/notes.md
git commit -m "test: cover encrypted media refs end to end"
```

- [ ] **Step 5: Summarize trial UX for the user**

Report: what the token looks like, what broke / felt awkward, whether full `ContextBoundReferenceRuntime` is worth it next.

---

## Self-review notes

- Spec [S1]–[S8]: S1/S2 covered by Tasks 2–3; S3 Task 1; S4 Task 2; S5 Task 3; S6 Tasks 2/4; S7 Tasks 1/2/4; S8 left out of scope (no tasks) intentionally.
- Placeholder: key parsing uses `hex`/`base64` crates — Task 1 Step 4 tells implementer to adjust imports if names differ.
- Type consistency: `encode_asset_ref` / `decode_asset_ref` names are stable across tasks; entity type `MediaAsset`.
