# TeaQL Agent Kit — Work Complete

Outcome: Rust **Axum web workspace** (`rust-web-axum`) media server with IP-whitelist Range streaming and **one-shot invite URLs** (`GET /invite/{token}`): click 1 joins the whitelist, click 2 expires the IP from the whitelist, further clicks are rejected (410).
Model: `/home/philip/workspace/A001/models/model.xml`
Outputs:
- `/home/philip/workspace/A001/app-playground/rust-lib-core` (generated domain library, untouched)
- `/home/philip/workspace/A001/app-playground/rust-web-axum` (**web** workspace — not app-console)
Verification: evaluation 0 errors; `cargo check` clean; `cargo test` **16/16**; live smoke confirmed join→Range→expire→410.

## Original Modeling Prompt

Media server in Rust + SQLite protecting links via IP whitelist for video playback: (1) Range/resume; (2) multi-IP concurrent access with ~7-day expiry; (3) proxy-aware client IP. **Follow-up:** random invite codes with rules — click 1 join whitelist, click 2 expire from whitelist; invite only usable once; **only via a specific URL**. Use a **Rust web framework workspace**, not app console.

## Modeling Result

- Service: `media-guard-service`
- Domain root: `media_library`
- Objects: 5 business + 2 constants (`grant_status`, `invite_status` FRESH/CLAIMED/SPENT)
- Modules: Library, Access (`ip_access_grant`, `invite_code`, `proxy_trust_rule`)
- Contract: `invite_code.invite_token` is a masked secret; one token maps to one `media_asset`; lifecycle FRESH → CLAIMED (1st click + grant) → SPENT (2nd click + revoke); `ip_access_grant` holds `client_ip` + `expire_at` (~7 days)
- KSML evidence: `models/model.xml` — `_audit_mask_fields="invite_token"` after KSML-PRIVACY-001

## API Constraint Harness Evidence

- Generated guide: `app-playground/rust-web-axum/AGENTS.md` (Rust Axum web workspace)
- Target used: **`rust-web-axum`** (Generation Service advertised; not `rust-app-console`)
- Assist commands used (model path `/home/philip/workspace/A001/models/model.xml`):
  - query/create/update/expression for `invite_code`, `ip_access_grant`, `media_asset`
  - field: `invite_token`, `invite_status`, `client_ip`, `expire_at`, `active`, `file_path`
  - `rust-assist-runtime-custom`
- API facts used:
  - Invite states: `with_invite_status_is_fresh/_is_claimed/_is_spent`, `update_invite_status_to_fresh/_to_claimed/_to_spent`
  - Token lookup: `Q::invite_codes().with_invite_token_is(token)`
  - Writes: `audit_as(...).save(context)` on create/redeem/revoke
  - Queries: `.comment("what: …").purpose("why: …")` on every execute
  - Axum runtime: `ContextProvider`, `TeaContext`, `WebResponse`
- Query policy: all executions include purpose and comment
- Write policy: all writes include `audit_as`
- Generated files edited manually: none
- Compile/test/runtime:
  - `cargo check` clean (workspace warnings only in generated lib)
  - `cargo test` 10 unit + 6 integration = **16 passed**
  - Smoke on `127.0.0.1:18090` (`smoke-web.log`):
    - `GET /invite/{token}` click1 → `join_whitelist`, `expireAt=+7d`
    - `GET /media/1001` + `Range: bytes=0-15` → **206** `bytes 0-15/2048`
    - resume `bytes=1024-` → **206** `bytes 1024-2047/2048`
    - click2 → `expire_whitelist`, `revokedGrants: 1`
    - `GET /media/1001` → **403**
    - click3 → **410 Gone**
- Code evidence: `src/service.rs` (`redeem_invite`, `grant_ip_access`), `src/handlers.rs` (`redeem_invite_url`, `serve_media`), `src/proxy.rs`, `src/range.rs`

## Agent Kit Impact

- Token strategy: skill + golden example + progressive Assist (ephemeral); no full rule catalog
- Context: evaluation rounds 3 across model revisions (2 boolean renames; 1 privacy mask)
- Token usage: not reported
- Estimated context avoided: not measured (Assist-only API discovery; no generated-lib grepping for APIs)
- Review automation: KSML gate, compile, 16 tests, live HTTP smoke
- Human review focus: invite security (token entropy), admin GET endpoints, trusted-proxy CIDRs, 7-day TTL

## Simple Next Use

1. Open `app-playground/rust-web-axum`.
2. Run `MEDIA_GUARD_SERVICE_CORE_DATABASE_URL=sqlite:///tmp/mg.db MEDIA_ROOT=/path/to/media cargo run`.
3. Open the seeded `/invite/INV-…` URL once (join), play `/media/{id}` with Range, open invite again (expire).

## Execution Log

- [request] Media server + one-shot invite URL (join/expire); Rust web workspace
- [model] added `invite_code` + `invite_status`; privacy mask on `invite_token`
- [check] 0 errors after KSML-PRIVACY-001 fix
- [codegen] `rust-web-axum` (user-required web target)
- [harness] purpose/comment + audit_as throughout
- [build/test] 16/16 pass
- [smoke] join → 206 Range → expire → 403 → 410
- [ready] artifacts for parallel review

## Invite rules (implemented)

| Click | Status before | Effect | Status after |
|------|---------------|--------|--------------|
| 1st | FRESH | Add client IP to whitelist (7-day `ip_access_grant`) | CLAIMED |
| 2nd | CLAIMED | Expire/revoke that IP's whitelist entry | SPENT |
| 3rd+ | SPENT | Rejected — expired codes cannot be used | SPENT |

Access is **only** via the dedicated URL `GET /invite/{token}` (not `/media`).
