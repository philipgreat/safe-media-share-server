# Encrypted media asset IDs (trial)

## [S1] Problem

Guest and public JSON expose raw `media_asset.id` in URLs and API bodies (`/view/{id}`, `/media/{id}`, list payloads). TeaQL runtime already provides entity-reference encryption so the frontend never needs internal IDs. This branch trials that path on media asset IDs only.

## [S2] Solution overview

Use `teaql_runtime::AeadEntityReferenceCodec` via `UserContext::encode_entity_reference` / `decode_entity_reference`.

- Outbound: replace raw `id` with encrypted `ref` (`tqr1.` + URL-safe base64 AES-256-GCM envelope).
- Inbound: route params take `ref`, decode with expected entity type `media_asset` and purpose `media-access`, then load by real id.
- Admin console may still show internal ids for operator diagnostics; guest-facing list/view/download must not.

## [S3] Runtime wiring

In `build_request_context`:

1. Construct `AeadEntityReferenceCodec::new(active_key_version, keys)`.
2. Install with `context.with_entity_reference_codec(...)`.
3. Key from env `MEDIA_GUARD_REF_KEY` (32 raw bytes, base64, or hex). Missing key → generate ephemeral key and log a startup warning (trial only).

Lifetime for issued refs: 7 days (aligns with IP grant TTL).

## [S4] API surface

| Endpoint | Change |
|----------|--------|
| `GET /api/media` | `id` becomes `ref`; optional `id` removed from guest JSON |
| `GET /api/media/{ref}` | path param is ref |
| `GET /view/{ref}` | SPA route uses ref |
| `GET /media/{ref}` | stream |
| `GET /media/{ref}/download` | attachment |
| Admin list/invites | may keep internal ids; invite copy links embed encrypted view refs |

Decode failure → 400/404 without echoing internal ids.

## [S5] Frontend

React list and `ViewPage` carry `ref` (string) instead of numeric id. Download/open links built from `ref`. No client-side crypto.

## [S6] Error handling

- Bad/foreign/expired ref → `400` or `404` (`ENTITY_REFERENCE_INVALID`).
- Missing codec on context → `ENTITY_REFERENCE_CODEC_REQUIRED` (config bug).
- Do not fall back to raw-id parsing in guest routes (keeps trial honest).

## [S7] Testing

- Unit/integration: encode → decode round-trip; list response has no raw numeric id; stream/download works with ref; garbage ref rejected.
- Manual smoke: curl list (no plaintext id), follow ref stream, confirm frontend routes.

## [S8] Out of scope

- Document snapshot open/accept (`ContextualDocument`)
- Full `ContextBoundReferenceRuntime` governance (principals, document scope)
- Invite token format changes (`INV-…` stays)
- Production key management / rotation ops
