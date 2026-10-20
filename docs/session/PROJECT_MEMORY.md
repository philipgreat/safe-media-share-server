Topic: Media server in Rust+SQLite — IP-whitelist link protection for video via TeaQL agent kit

# Project memory
_Durable project-level knowledge. Persists across all sessions in this project. Edit only content under italic instructions._

## Project context
_What is this project? What's its goal? High-level identity._

A simple media server written in Rust with SQLite, built by following the TeaQL Agent Kit (`https://github.com/teaql/teaql-agent-kit`, skill `build-teaql-app`) model-first workflow. Purpose: serve videos with link protection — IP whitelist per resource. Core capabilities: HTTP Range/resume, multi-IP concurrent access with ~7-day expiry per IP grant, proxy-aware client IP detection, one-shot invite links that join/expire the whitelist, and a browse UI (play media inline; download documents/zips). Delivery workspace is a Rust web-framework target (`rust-web-axum`), not the console app.

## Rules
_Hard constraints from user that every session must respect._

- Follow the teaql-agent-kit instructions from `https://github.com/teaql/teaql-agent-kit` (treat the GitHub repo as the updated source of truth) for the delivery workflow: KSML model → evaluate/repair → generate → implement → verify → evidence-backed `work-complete.md`.
- Stack is fixed: Rust + SQLite.
- Media server must (1) support resume/Range, (2) allow multiple IPs per resource with ~7-day expiry, (3) resolve real client IP behind proxies.
- Invite codes: randomly generated; click 1 = join whitelist, click 2 = expire from whitelist; only one grant click; expired/spent codes unusable; invite only via one specific URL.
- Use a Rust web-framework TeaQL workspace (`rust-web-axum`), not `rust-app-console`.
- Never edit generated domain-library source; application logic stays in the editable workspace. Use progressive `rust-assist-*` for APIs — do not guess method names or grep generated lib.
- Frontend requirement: page UI to view media; documents, zip, etc. must support direct download.

## Architecture decisions
_Major design choices with rationale. The "why" matters more than the "what" for future sessions._

- Link protection is IP-whitelist based (user explicitly wants to set allowed IPs per resource for video access), not primarily tokens/signed URLs.
- Access grants are multi-IP and time-limited (~7 days), so the domain model pairs resource ↔ IP with an expiry timestamp; runtime enforces both grant status and `expire_at` (fail closed).
- Trust boundary for client IP accounts for reverse proxies via modelled `proxy_trust_rule` rows; untrusted peers' forwarded headers are ignored (anti-spoof). Prefer app-side rightmost-untrusted XFF resolution over `WebContextExt::client_ip()` (leftmost, spoofable).
- HTTP Range/resume is application-layer (axum 206/Content-Range), not domain-modelled.
- Invite codes are a one-shot toggle into that whitelist: `invite_code` + `invite_status` FRESH/CLAIMED/SPENT; click1 creates `ip_access_grant`, click2 revokes/expires it; only `GET /invite/{token}` redeems; `invite_token` is audit-masked secret.
- Frontend is a **React + Vite SPA** (`rust-web-axum/web/`, built to `web/dist`, served at runtime with SPA fallback): `/` fixed invite-code entry (type `INV-…` / `/invite/…` / full URL → redeem → go to `/view/{id}`), `/view/{id}` plays video/audio/image/pdf inline or shows a download card; `/media/{id}/download` always `Content-Disposition: attachment` for docs/zip; content stays IP-gated while metadata list is visible. Deep link `/?code=INV-…` auto-submits. Browser `Accept: text/html` on `/invite/{token}` 303-redirects (join → view; expire/spent/invalid → `/?…` flags). After editing `web/src`, rebuild with `PATH=/home/philip/.local/node/bin:$PATH npm run build`.
- External URL prefix (`external_base_url`) must drive invite **copy links and media-center open/download links** (`viewUrl`/`downloadUrl`) when LAN host ≠ public host. Empty prefix → browser origin; bare host becomes `https://`.
- Per-request `UserContext` is rebuilt from a shared `DataServicePool` via `build_request_context` (id generator + sqlite provider + executor). `UserContext` is not Clone.
- Privileged management ("超级邀请码") is an **env-var Admin Key** (`MEDIA_GUARD_ADMIN_KEY`), not a domain invite code: unlocks a sidebar UI (媒体中心 / 当前白名单 / 二次点击作废邀请码 / 设置) and copyable per-media invite codes. Auth is header `X-Admin-Key` (unset key → 503 disable; wrong → 401). Implement as app-level auth over existing Q listings; keep out of KSML unless more admin state is modeled.
- **Settings live in the database**, not a config file: `app_setting` rows under domain root `media_library` with `setting_key`, `setting_value`, `setting_name`, `setting_note` (multiple entries). External URL prefix (`external_base_url`) is one such row; admin 设置 page reads/writes via Q/E. Prefer this over JSON `SettingsStore` / `admin-settings.json`.

## Discovered durable knowledge
_Cross-task facts that survive across sessions. Promoted from session checkpoints' §7 when proven durable._

### Discovered
- TeaQL Agent Kit workflow: requirement → inspectable KSML domain model → deterministic evaluation + repair → generated typed Q/E contract → workspace implementation → build/test/runtime evidence → `work-complete.md`.
- Kit files: `skills/build-teaql-app/SKILL.md`, `references/golden-example.xml`, `references/toolchains.md`, `references/work-complete.md`.
- Live `cargo teaql services` catalog is authoritative over kit tables: Rust targets include `rust-lib-core`, `rust-app-console`, `rust-web-axum`, `rust-web-topcoat`.
- `rust-web-axum` uses `teaql_web_integration_axum::{TeaContext, WebResponse, AxumTeaError, ContextProvider}`; bind `MEDIA_GUARD_SERVICE_CORE_BIND_ADDR`. Workspace crate name is `media_guard_service_core_web_axum`.
- **Runnable UserContext needs full data-service wiring.** `into_context()` alone fails with `missing typed resource: …ServiceRuntimeExecutor`. Also, executor-only install is insufficient for creates (`UNIQUE constraint` on ids). Recipe: `DataServiceMutationExecutor::new(pool)` → `DataServiceIdGenerator::from_executor` → `set_internal_id_generator` → `use_sqlite_provider` → `ServiceRuntimeExecutor::new` → `register_executor` + `insert_resource` (see `service_runtime_from_pool` in generated `runtime.rs`).
- `ServiceRuntime` is a type alias for `UserContext` (not Clone). Share `DataServicePool` (`Arc<Mutex<rusqlite::Connection>>`) and rebuild context per request.
- Axum `Handler` requires `Send` futures: application service errors must be `Box<dyn std::error::Error + Send + Sync>`.
- Rust client pin: `cargo-teaql` **exactly 2.0.14** (`cargo install cargo-teaql --version 2.0.14 --force`; `cargo-teaql install-links`).
- KSML pitfalls: no `is_` boolean prefixes; two-word entity/field names; non-empty representative attribute values; constants need complete `<_value>` rows; all entities connected to domain root; secret fields need `_audit_mask_fields`.
- Generated Rust APIs: `Q::…().select_*().which_*().purpose().comment().execute_for_*(&ctx)`, `E::…().get_*().eval()`; writes carry `audit_as` + `save`; filters before `.purpose()`; full `Q::entity()` before save or expression access; invite lifecycle via `with_invite_status_is_fresh/claimed/spent`.
- Default grant TTL: 7 days; `expire_at` string `%Y-%m-%dT%H:%M:%S`.
- Regenerating a workspace overwrites `src/lib.rs`/`src/main.rs` but keeps other app-owned files.
- Env: `MEDIA_GUARD_SERVICE_CORE_DATABASE_URL`, `MEDIA_ROOT`, `MEDIA_GUARD_SERVICE_CORE_BIND_ADDR`. Tests need `axum::extract::ConnectInfo(peer)` injected into request extensions.
- Invite codes are minted via `GET /admin/invites?asset_id=N&note=…` (legacy smoke, no key) and admin `GET/POST /admin/api/invites` with `X-Admin-Key` (also seeded as `url=/invite/INV-…` on empty DB). Token form `INV-<hex>`.
- Frontend build: Node 22 at `/home/philip/.local/node/bin` (put on `PATH` before `npm run build`); Axum reads `web/dist` at runtime (no `include_str!` rebuild of the Rust binary needed for UI changes).
- Admin API surface: `/admin/api/ping|media|whitelist|spent-invites|invites|settings`. Settings JSON field: `externalBaseUrl` (keep this contract even when backing store is `app_setting`).
- Bind `0.0.0.0:18090` for LAN (`192.168.2.206` on this host). If `HTTP_PROXY`/`ALL_PROXY` are set, use `curl --noproxy '*'` for local/LAN smoke or it will time out through the proxy.
- TeaQL services: `https://api.teaql.io/latest/`; open-source partial generator `teaql-forge-rs`; Rust runtime `teaql-rs`.

### Dead ends
- `_minimal()` query projections must not be used for entities that will be saved — load with the full `Q::entity()` request first.
- `rust-app-console` is the wrong delivery surface when the user wants a web framework — use `rust-web-axum`.
- `Box<dyn std::error::Error>` in async axum handlers/service signatures breaks `Handler` (`!Send`).
- Executor-only `register_executor`+`insert_resource` is not enough for DB writes; also install id generator + `use_sqlite_provider`.
