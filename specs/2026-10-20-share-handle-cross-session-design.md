# share_handle: durable cross-session media links

## [S1] Problem

Encrypted media refs (`tqr1.…`) are **session-scoped and fingerprinted** (bound to the issuing session). Invite codes are cross-session but one-shot (FRESH→CLAIMED→SPENT) and must not embed those refs. We need a **durable, shareable handle** stored in the database that opens the same media across sessions, while each open **re-issues a session-local fingerprinted ref**.

## [S2] Role split

| Handle | Role | Lifetime | Storage |
|--------|------|----------|---------|
| `INV-…` invite code | one-shot IP whitelist join/expire | FRESH→CLAIMED→SPENT | `invite_code` (existing) |
| `SHD-…` share handle | cross-session open media | with media; revocable | `share_handle` (new) |
| `tqr1.…` session ref | play/download APIs | current session | not durable |

Rule: **never put fingerprinted media refs in shareable URLs.** Share links use `SHD-…` only.

## [S3] Domain entity `share_handle`

KSML (`models/model.xml`), then regenerate `rust-lib-core` via `cargo teaql`.

```text
share_handle
  media_library          # domain root link
  media_asset            # target asset
  handle_token           # secret; _audit_mask_fields="handle_token"; form SHD-<hex>
  active                 # boolean revocation
  handle_note            # optional text
  create_time, update_time
```

- Auto-create **one** `share_handle` row when a `media_asset` is created (upload / admin create / seed).
- Token is a masked secret (same class as `invite_token`).
- Idempotent create: do not add a second handle for the same asset on re-upload of same path if one active exists (create only when missing).

## [S4] Open flow (cross-session)

```text
GET /open/{handle_token}
  1. Load share_handle by handle_token; require active.
  2. Resolve client IP (existing proxy rules).
  3. Require IP allowed for that media_asset (existing whitelist / invite grants).
     If not allowed → redirect/error telling user to use invite (no asset id).
  4. Mint a **session-local** media ref for the current session
     (fingerprint/purpose bound to this session).
  5. 303 → /view/{session_ref} (HTML) or JSON { mediaRef } for API clients.
```

- Opening again from a **new session** mints a **different** ref (new fingerprint).
- `handle_token` itself never appears in `/media/...` or `/view/...` after redirect.
- Invalid/inactive handle → 404/410 without echoing internal ids.

## [S5] Session fingerprint binding

Hard constraint: media refs are only valid for the session that minted them.

Trial implementation (acceptable for this branch):

- Session identified by HttpOnly cookie `mg_sid` (opaque random id created on first need) **or** an in-memory session key derived per request if cookie absent (then ref is request-scoped only — prefer cookie).
- `encode_asset_ref` purpose stays `media-access` for codec compatibility with existing tests **or** purpose becomes `media-access:{session_fp}` and decode requires matching fp (stronger). Prefer **stronger**: purpose `media-access:{fp}` where `fp` is the cookie session id (or hash of it).
- `decode_asset_ref` on `/media/{ref}`, `/api/media/{ref}` must verify current session fp matches purpose (or reject).
- Optional audit columns / log: `last_session_fp`, `last_open_at` on `share_handle` (nice-to-have if cheap).

Upgrade path (out of scope): `ContextBoundReferenceRuntime` actor fingerprint.

## [S6] API / routes

| Endpoint | Behavior |
|----------|----------|
| `GET /open/{handle_token}` | durable entry; mint session ref; redirect or JSON |
| Guest media routes | unchanged shape — still session `tqr1.` refs only |
| Admin list | add `shareUrl` = external `/open/{handle_token}`; may show `shareHandleId` but **not** require numeric id in share URL |
| Admin revoke | `active=false` (minimal: admin API toggle; UI button if small) |

Invite redeem flow **unchanged** (whitelist only). Do not return session refs from invite as durable bookmarks; invite redirect may still mint a **session** ref for immediate UX.

## [S7] Error handling

- Unknown / inactive handle → 404 or 410, message without asset id.
- IP not allowed → redirect `/?need_invite=1` or JSON error; no asset id.
- Session fp mismatch on media ref → 400 `invalid media reference`.
- Missing codec / bad key → existing `ENTITY_REFERENCE_CODEC_REQUIRED` / invalid path.

## [S8] Testing

- Create asset → handle exists; token form `SHD-…`; token not equal raw id.
- `GET /open/{token}` with allowed IP → redirect to `/view/tqr1.…`; **no** `SHD-` or raw id in view path.
- Same handle, two different session cookies → two different refs; each ref works only with its cookie.
- Inactive handle → 404/410.
- IP not whitelisted → no media, no raw id leak.
- Full suite stays green (`cargo test`).

## [S9] Out of scope

- Multi-handle per asset, ACLs per handle, password-protected handles
- Replacing invite one-shot semantics
- Full `ContextBoundReferenceRuntime` production key ops
- QR/short-link UX
