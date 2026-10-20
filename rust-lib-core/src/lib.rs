
//! Generated TeaQL domain crate for `media-guard-service-core`.
//!
//! **Before writing queries**, read the `AGENTS.md` at the workspace root.
//! It contains the entity list and the exact `cargo teaql` commands to fetch API prompts.
//!
//! The generated library is not the API-discovery surface. Read the generated
//! application's `AGENTS.md`, then request model-aware object/field Assist.
//! A registry dependency does not require vendoring or browsing generated
//! domain-library source to learn method names. If Assist lacks an operation,
//! report `MISSING_ASSIST` for that path.

pub mod e;
pub mod q;
pub mod request_support;
pub mod runtime;
pub mod sample_data;
pub mod media_library;
pub mod media_asset;
pub mod grant_status;
pub mod ip_access_grant;
pub mod invite_status;
pub mod invite_code;
pub mod app_setting;
pub mod proxy_trust_rule;
pub mod share_handle;
pub mod user_session;

pub use teaql_core;
pub use teaql_runtime::LedgerEntity;
pub use e::*;
pub use q::*;
pub use request_support::*;
pub use runtime::*;
pub use sample_data::*;
pub use media_library::*;
pub use media_asset::*;
pub use grant_status::*;
pub use ip_access_grant::*;
pub use invite_status::*;
pub use invite_code::*;
pub use app_setting::*;
pub use proxy_trust_rule::*;
pub use share_handle::*;
pub use user_session::*;