# share_handle Cross-Session Media Links Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use compose:subagent (recommended) or compose:execute to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Durable `SHD-…` share handles (DB) open media across sessions; each open mints a **session-fingerprinted** `tqr1.…` ref that only that session can use.

**Architecture:** New KSML entity `share_handle` (masked `handle_token`). `GET /open/{handle_token}` resolves handle + IP ACL → cookie session `mg_sid` → `encode_asset_ref` with purpose `media-access:{fp}` → redirect `/view/{ref}`. Media routes decode and require matching fingerprint. Invite flow stays whitelist-only.

**Tech Stack:** Rust, Axum, TeaQL KSML + `cargo teaql` regen, `AeadEntityReferenceCodec`, React (minimal), SQLite.

## Global Constraints

- Fingerprinted media refs are **never** placed in shareable/durable URLs; share URLs use `SHD-…` only.
- `handle_token` is a masked secret (`_audit_mask_fields="handle_token"`); form `SHD-` + hex.
- Auto-create exactly one active `share_handle` when a `media_asset` is created (no duplicate if one exists).
- Session fingerprint is bound into ref purpose `media-access:{fp}`; mismatch → 400 `invalid media reference`.
- Session id cookie `mg_sid` is HttpOnly; opaque random; do not put asset ids in the cookie.
- Guest responses must not leak raw `media_asset.id` or `handle_token` outside intentional share URLs (admin shows `shareUrl`).
- Inactive/unknown handle → 404/410 without internal ids.
- Domain writes: generated Q/E + `audit_as` + `.comment`/`.purpose` on executes. Do not hand-edit `rust-lib-core` generated sources; regenerate from `models/model.xml`.
- Branch: `feat/encrypted-entity-ids` (continue) or `feat/share-handle` if isolation preferred — default **continue same branch**.
- After `web/src` edits: `PATH=/home/philip/.local/node/bin:$PATH npm run build` in `web/`.
- Spec: `specs/2026-10-20-share-handle-cross-session-design.md` ([S1]–[S9]).

---

### Task 1: KSML `share_handle` + regenerate domain lib

**Covers:** [S3]

**Files:**
- Modify: `models/model.xml`
- Regen: `app-playground/rust-lib-core/` (via `cargo teaql`)
- Test: `app-playground/rust-web-axum/tests/share_handle_model_test.rs` (optional smoke once Q compiles)

**Interfaces:**
- Consumes: existing `media_library`, `media_asset` entities
- Produces: generated `Q::share_handles()`, `E::share_handle(_)`, fields `handle_token`, `active`, `handle_note`, `media_asset_id`, `media_library_id` (exact getters after regen)

- [ ] **Step 1: Add entity to `models/model.xml`** (before `</root>`, after `proxy_trust_rule`):

```xml
  <share_handle _name="Share Handle"
                _audit_mask_fields="handle_token"
                _module="Access"
                _module_key="access"
                media_library="media_library()"
                media_asset="media_asset()"
                handle_token="SHD-7f3a9c2e5b1d"
                active="true"
                handle_note="Durable cross-session open link for demo video"
                create_time="createTime()"
                update_time="updateTime()"/>
```

- [ ] **Step 2: Evaluate + regenerate**

```bash
cd /home/philip/githome/safe-media-share-server
cargo teaql --input models/model.xml evaluate
cargo teaql --input models/model.xml rust-lib-core \
  --output rust-lib-core \
  --cwd "$PWD/app-playground"
```

Expected: 0 errors; `app-playground/rust-lib-core/lib/src/share_handle/` appears.

- [ ] **Step 3: Compile web workspace against new lib**

```bash
cd /home/philip/githome/safe-media-share-server/app-playground/rust-web-axum
cargo check
```

Expected: OK (entity unused yet).

- [ ] **Step 4: Commit model + regen**

```bash
cd /home/philip/githome/safe-media-share-server
git add models/model.xml app-playground/rust-lib-core
git commit -m "feat(model): add share_handle for durable cross-session media links"
```

If `rust-lib-core` is large/untracked: add the new `share_handle` module + updated files that the generator touched (`lib.rs`, `q.rs`, `e.rs`, etc.).

---

### Task 2: Session fingerprint refs + share_handle service

**Covers:** [S2], [S5], [S7]

**Files:**
- Modify: `app-playground/rust-web-axum/src/entity_ref.rs`
- Modify: `app-playground/rust-web-axum/src/service.rs`
- Test: `app-playground/rust-web-axum/tests/entity_ref_test.rs` (extend)

**Interfaces:**
- Consumes: Task 1 Q/E `share_handle`
- Produces:
  - `entity_ref::purpose_for_session(fp: &str) -> String` → `format!("media-access:{fp}")`
  - `entity_ref::encode_asset_ref_for_session(context, asset_id, session_fp) -> Result<String, String>`
  - `entity_ref::decode_asset_ref_for_session(context, token, session_fp) -> Result<u64, String>`
  - Keep existing `encode_asset_ref`/`decode_asset_ref` as wrappers with `fp = ""` **or** migrate all call sites — **prefer migrate call sites** to session variants in Task 3; for Task 2 add new fns + unit tests.
  - `service::ensure_share_handle(context, library_id, asset_id) -> Result<ShareHandle, _>`
  - `service::load_share_handle_by_token(context, token) -> Result<Option<ShareHandle>, _>`
  - `service::set_share_handle_active(context, handle_id, active) -> Result<(), _>`
  - `service::generate_handle_token() -> String` → `SHD-` + 24 hex chars

- [ ] **Step 1: Failing tests for session-bound refs**

```rust
#[test]
fn session_purpose_binds_fingerprint() {
    let context = /* UserContext with codec via build_request_context */;
    let token = encode_asset_ref_for_session(&context, 987654321012, "fp-a").unwrap();
    assert_eq!(decode_asset_ref_for_session(&context, &token, "fp-a").unwrap(), 987654321012);
    assert!(decode_asset_ref_for_session(&context, &token, "fp-b").is_err());
}
```

- [ ] **Step 2: Implement in `entity_ref.rs`**

```rust
pub fn purpose_for_session(fp: &str) -> String {
    if fp.is_empty() { PURPOSE.to_owned() } else { format!("{PURPOSE}:{fp}") }
}

pub fn encode_asset_ref_for_session(
    context: &UserContext,
    asset_id: u64,
    session_fp: &str,
) -> Result<String, String> {
    let purpose = purpose_for_session(session_fp);
    context
        .encode_entity_reference(
            ENTITY_TYPE, asset_id, 0, &purpose,
            Duration::from_secs(REF_LIFETIME_SECS),
        )
        .map_err(|err| map_token_err(&err))
}

pub fn decode_asset_ref_for_session(
    context: &UserContext,
    token: &str,
    session_fp: &str,
) -> Result<u64, String> {
    let purpose = purpose_for_session(session_fp);
    context
        .decode_entity_reference(token, ENTITY_TYPE, &purpose)
        .map(|claims| claims.id)
        .map_err(|err| map_token_err(&err))
}
```

- [ ] **Step 3: `service.rs` share_handle helpers**

Follow `create_invite` / `load_invite_by_token` patterns:

```rust
pub fn generate_handle_token() -> String {
    // SHD- + 12 random bytes hex (mirror generate_invite_token style)
}

pub async fn ensure_share_handle(
    context: &impl TeaqlRuntime,
    library_id: u64,
    media_asset_id: u64,
) -> Result<ShareHandle, Box<dyn std::error::Error + Send + Sync>> {
    // query share_handles with_media_asset_id_is(...).with_active_is(true) limit 1
    // if some → return it
    // else create with handle_token = generate_handle_token(), active true, note ""
}

pub async fn load_share_handle_by_token(...) -> ... // with_handle_token_is(token)
pub async fn set_share_handle_active(...) -> ... // update_active + audit_as
```

In `create_media_asset`, after successful save, call `ensure_share_handle` (best-effort: log error but still return asset if handle fails — **prefer propagate error** so invariant holds; document choice).

- [ ] **Step 4: Tests green + commit**

```bash
cargo test --test entity_ref_test
cargo test
git add app-playground/rust-web-axum/src/entity_ref.rs app-playground/rust-web-axum/src/service.rs app-playground/rust-web-axum/tests
git commit -m "feat: session-bound media refs and share_handle service helpers"
```

---

### Task 3: `GET /open/{handle_token}` + cookie session + media decode binding

**Covers:** [S4], [S5], [S6], [S7]

**Files:**
- Modify: `app-playground/rust-web-axum/src/handlers.rs`
- Create or modify: session cookie helpers (in `handlers.rs` or `src/session.rs`)
- Test: `app-playground/rust-web-axum/tests/share_handle_test.rs`

**Interfaces:**
- Consumes: Task 2 service + `encode_asset_ref_for_session` / `decode_asset_ref_for_session`
- Produces:
  - `session_fp_from_headers(headers) -> (String, Option<Set-Cookie>)` — read `mg_sid` or create
  - Route `GET /open/{handle_token}`
  - All media/API paths that decode refs require the same fp as encode

- [ ] **Step 1: Failing integration tests** (`tests/share_handle_test.rs`)

```rust
#[tokio::test]
async fn open_handle_redirects_to_session_view_ref() {
    // setup asset + handle + IP allow
    // GET /open/SHD-... with Cookie: mg_sid=fp-a
    // expect 303 Location /view/tqr1.… and decode with fp-a
}

#[tokio::test]
async fn session_ref_rejected_from_other_session() {
    // mint with fp-a; GET /api/media/{ref} with mg_sid=fp-b → 400
}

#[tokio::test]
async fn inactive_handle_is_not_found() { ... }

#[tokio::test]
async fn two_sessions_get_different_refs() { ... }
```

- [ ] **Step 2: Cookie session helper**

```rust
const SESSION_COOKIE: &str = "mg_sid";

fn session_fp_from_headers(headers: &HeaderMap) -> (String, Option<String>) {
    // parse Cookie header for mg_sid; if missing generate 32 hex, return Some(new_cookie_value)
}
```

On responses that mint refs, if new, set `Set-Cookie: mg_sid=…; HttpOnly; Path=/; SameSite=Lax`.

- [ ] **Step 3: Handler `open_share_handle`**

```rust
async fn open_share_handle(
    State(state): State<AppState>,
    TeaContext(context): TeaContext,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    Path(handle_token): Path<String>,
    headers: HeaderMap,
) -> Response {
    // load handle; 404 if none; 410 if !active
    // resolve_ip; if !is_ip_allowed(asset_id) → 303 /?need_invite=1 (HTML) or 403 JSON
    // session_fp = session_fp_from_headers
    // media_ref = encode_asset_ref_for_session(..., &session_fp)
    // 303 Location: /view/{media_ref} + Set-Cookie if new
}
```

Register: `.route("/open/{handle_token}", get(open_share_handle))`.

- [ ] **Step 4: Bind decode to session**

Wherever `decode_asset_ref` is used (`api_media_detail`, `serve_media`, `download_media`), switch to `decode_asset_ref_for_session(..., &session_fp_from_headers)`. Empty fp only if you keep a test escape — prefer always require cookie after `/open` (tests set Cookie).

Invite redirect may encode with empty fp (legacy) **or** also attach cookie + session purpose — **prefer session purpose + cookie** for consistency.

- [ ] **Step 5: Admin shareUrl**

In `admin_api_media` JSON add:

```rust
"shareUrl": join_external(&base, &format!("/open/{handle_token}")),
```

Load handles by asset id (batch query). Do not put `handle_token` bare in guest JSON.

- [ ] **Step 6: Tests + commit**

```bash
cargo test
git add app-playground/rust-web-axum/src app-playground/rust-web-axum/tests
git commit -m "feat: /open share handles mint session-bound media refs"
```

---

### Task 4: Frontend share link + e2e verification

**Covers:** [S6], [S8]

**Files:**
- Modify: `app-playground/rust-web-axum/web/src/pages/AdminMedia.jsx` (show/copy `shareUrl`)
- Modify: `docs/session/notes.md` (trial notes)
- Test: ensure suite green

**Steps:**

- [ ] **1.** AdminMedia: display copy button for `it.shareUrl` (fallback joinExternal `/open/…` only if API provides token — prefer API field).
- [ ] **2.** Rebuild `web/` (`npm run build`).
- [ ] **3.** `cargo test` full; optional curl smoke `/open/SHD-…`.
- [ ] **4.** Append notes: durable `SHD-` vs session `tqr1.`, fingerprint purpose, cookie `mg_sid`.
- [ ] **5.** Commit.

---

## Self-review notes

- S1–S9: S1 background; S2 Task 2–3; S3 Task 1; S4 Task 3; S5 Task 2–3; S6 Task 3–4; S7 Task 2–3; S8 Task 3–4; S9 out of scope.
- Purpose format `media-access:{fp}` is stable across tasks.
- `ensure_share_handle` used from `create_media_asset` and tests.
