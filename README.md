# Safe Media Share Server

IP-whitelist protected media server with one-shot invite links.

**Stack:** Rust · SQLite · TeaQL · Axum · React

Guests open a fixed page, paste an invite code, and enter a video or document.
Owners manage media, whitelist, spent invites, upload, and external URL from an admin console.

## Features

- **HTTP Range / resume** — video seek in browser or player
- **One-shot invite codes**
  - 1st use: client IP joins the whitelist (~7 days)
  - 2nd use: IP is removed from the whitelist
  - further uses: rejected (code spent)
- **Proxy-aware client IP** — `X-Forwarded-For` / `X-Real-IP` + trusted CIDR rules
- **Multiple concurrent IPs** per asset, each grant with expiry
- **Documents & ZIP download** — `Content-Disposition: attachment`
- **File / folder upload** with per-file progress in admin UI
- **Settings in the database** (`app_setting` under `media_library`), including external base URL for share links
- **Admin super key** via environment variable (separate from guest invite codes)

## Layout

```
models/model.xml                 # KSML domain model (source of truth)
rust-lib-core/                   # Generated domain library (do not hand-edit)
rust-web-axum/                   # Axum web service (API, ACL, Range, admin)
  src/                           # handlers, service, session, entity_ref
  tests/                         # integration tests
react-frontend/                  # React + Vite frontend
work-complete.md                 # build evidence notes
```

## Requirements

- Rust 1.80+
- Node.js 20+ (frontend build)
- Optional for model regeneration: `cargo install cargo-teaql --version 2.0.14`

## Quick start

```bash
# 1) Frontend
cd react-frontend
npm install
npm run build

# 2) Server
cd ../rust-web-axum
export MEDIA_GUARD_SERVICE_CORE_DATABASE_URL="sqlite:///tmp/media-guard/data/media.db"
export MEDIA_ROOT="/path/to/media"
export MEDIA_GUARD_SERVICE_CORE_BIND_ADDR="0.0.0.0:18090"
export MEDIA_GUARD_ADMIN_KEY="change-me"

cargo run --release
```

| Page | URL |
|------|-----|
| Public (invite entry) | http://127.0.0.1:18090/ |
| Admin console | http://127.0.0.1:18090/admin |
| Health | http://127.0.0.1:18090/health |

## Invite flow

1. Admin → **Media** → create / copy invite token or link.
2. Guest pastes the code on `/` (or opens `/invite/INV-…`).
3. **First submit:** IP whitelisted → `/view/{id}` (play or download).
4. **Second submit:** IP removed; code becomes **SPENT**.

## Admin API

Requires header `X-Admin-Key: <MEDIA_GUARD_ADMIN_KEY>`.

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/admin/api/ping` | Auth check |
| GET/POST | `/admin/api/media` | List / upload media |
| GET/POST | `/admin/api/invites` | Create invites |
| GET | `/admin/api/whitelist` | Active IP grants |
| GET | `/admin/api/spent-invites` | Spent invite codes |
| GET/POST | `/admin/api/settings` | DB settings (external URL) |

```bash
curl -H "X-Admin-Key: change-me" \
  -F "file=@video.mp4" \
  -F "file_path=videos/video.mp4" \
  http://127.0.0.1:18090/admin/api/media
```

## Domain model (TeaQL)

Source of truth: [`models/model.xml`](models/model.xml).

Root `media_library` → `media_asset`, `ip_access_grant`, `invite_code`, `proxy_trust_rule`, `app_setting`  
Constants: `grant_status`, `invite_status`.

Regenerate after model edits:

```bash
cargo teaql --input models/model.xml evaluate
cargo teaql --input models/model.xml rust-lib-core \
  --output rust-lib-core
```

## Tests

```bash
cd rust-web-axum
cargo test
```

## Configuration

| Variable | Required | Description |
|----------|----------|-------------|
| `MEDIA_GUARD_SERVICE_CORE_DATABASE_URL` | yes | e.g. `sqlite:///path/to/data.db` |
| `MEDIA_ROOT` | yes | Media directory |
| `MEDIA_GUARD_SERVICE_CORE_BIND_ADDR` | no | Default `127.0.0.1:3000` |
| `MEDIA_GUARD_ADMIN_KEY` | yes (admin) | Super key for `/admin` |
| `MEDIA_GUARD_EXTERNAL_URL` | no | Initial external URL prefix |

External URL lives in DB key `external_base_url` (Admin → Settings) and is used for invite / open / download links.

## License

See [LICENSE](LICENSE).

## Session notes

Development session artifacts are under [`docs/session/`](docs/session/):

- `checkpoint.md` — structured session state
- `PROJECT_MEMORY.md` — durable project notes
- `notes.md` — scratch notes
- `MANIFEST.md` — what was migrated and from where

