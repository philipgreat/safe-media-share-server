//! Encrypted entity references for guest-facing media asset IDs.
//!
//! Outbound tokens are AES-256-GCM envelopes (`tqr1.` + URL-safe base64) so
//! guests never see backend numeric ids. Admin surfaces may still show ids.

use std::sync::{Arc, OnceLock};
use std::time::Duration;

use base64::Engine as _;
use teaql_runtime::{AeadEntityReferenceCodec, UserContext};

/// TeaQL entity name used in token claims.
pub const ENTITY_TYPE: &str = "MediaAsset";
/// Purpose claim bound to media access routes.
pub const PURPOSE: &str = "media-access";
/// Token lifetime: 7 days (aligned with IP grant TTL).
pub const REF_LIFETIME_SECS: u64 = 7 * 24 * 3600;

const KEY_VERSION: u32 = 1;
const KEY_ENV: &str = "MEDIA_GUARD_REF_KEY";

/// Opaque invalid-reference message. Single owner of the string; callers must
/// not hardcode it.
pub const INVALID_REF: &str = "invalid media reference";
/// Missing-codec config-bug code. Single owner of the string.
pub const CODEC_REQUIRED: &str = "ENTITY_REFERENCE_CODEC_REQUIRED";

/// Map token errors to a stable message. Missing codec is a config bug and
/// keeps its code; everything else collapses to one opaque message so raw ids
/// and crypto details never leak.
fn map_token_err(err: &teaql_runtime::EntityReferenceTokenError) -> String {
    if err.code() == CODEC_REQUIRED {
        CODEC_REQUIRED.to_owned()
    } else {
        INVALID_REF.to_owned()
    }
}

/// Process-wide codec, key version 1, installed on every request context.
pub fn shared_codec() -> Arc<AeadEntityReferenceCodec> {
    static CODEC: OnceLock<Arc<AeadEntityReferenceCodec>> = OnceLock::new();
    CODEC
        .get_or_init(|| {
            let key = load_key();
            Arc::new(
                AeadEntityReferenceCodec::new(KEY_VERSION, [(KEY_VERSION, key.to_vec())])
                    .expect("ref key is 32 bytes"),
            )
        })
        .clone()
}

/// Encode a media asset id into an encrypted guest-facing reference.
pub fn encode_asset_ref(context: &UserContext, asset_id: u64) -> Result<String, String> {
    context
        .encode_entity_reference(
            ENTITY_TYPE,
            asset_id,
            0,
            PURPOSE,
            Duration::from_secs(REF_LIFETIME_SECS),
        )
        .map_err(|err| map_token_err(&err))
}

/// Decode a guest-facing reference back to the real media asset id.
///
/// All failures map to one message so raw ids and crypto details never leak,
/// except missing-codec which is a distinct configuration error.
pub fn decode_asset_ref(context: &UserContext, token: &str) -> Result<u64, String> {
    context
        .decode_entity_reference(token, ENTITY_TYPE, PURPOSE)
        .map(|claims| claims.id)
        .map_err(|err| map_token_err(&err))
}

/// Purpose claim for a session-bound media ref.
///
/// Empty `fp` keeps bare `media-access` so older refs stay interoperable;
/// any other fingerprint yields `media-access:{fp}` and is only valid for the
/// session that minted it ([S5]).
pub fn purpose_for_session(fp: &str) -> String {
    if fp.is_empty() {
        PURPOSE.to_owned()
    } else {
        format!("{PURPOSE}:{fp}")
    }
}

/// Encode a media asset id bound to one session fingerprint.
pub fn encode_asset_ref_for_session(
    context: &UserContext,
    asset_id: u64,
    session_fp: &str,
) -> Result<String, String> {
    context
        .encode_entity_reference(
            ENTITY_TYPE,
            asset_id,
            0,
            &purpose_for_session(session_fp),
            Duration::from_secs(REF_LIFETIME_SECS),
        )
        .map_err(|err| map_token_err(&err))
}

/// Decode a session-bound reference; only the minting fingerprint succeeds.
///
/// Failures collapse to the same opaque message as [`decode_asset_ref`].
pub fn decode_asset_ref_for_session(
    context: &UserContext,
    token: &str,
    session_fp: &str,
) -> Result<u64, String> {
    context
        .decode_entity_reference(token, ENTITY_TYPE, &purpose_for_session(session_fp))
        .map(|claims| claims.id)
        .map_err(|err| map_token_err(&err))
}

/// Key load order: `MEDIA_GUARD_REF_KEY` as 64-char hex, standard base64 of
/// 32 bytes, or exactly 32 raw bytes (any bytes, not necessarily UTF-8).
/// When the variable is unset, an ephemeral random key is used (trial only).
fn load_key() -> [u8; 32] {
    match std::env::var_os(KEY_ENV) {
        Some(raw) => {
            use std::os::unix::ffi::OsStrExt as _;
            parse_key_material(raw.as_bytes()).unwrap_or_else(|err| panic!("{err}"))
        }
        None => {
            eprintln!("WARN: {KEY_ENV} not set; using ephemeral entity reference key (trial only, refs die on restart)");
            ephemeral_key()
        }
    }
}

/// Parse `MEDIA_GUARD_REF_KEY` material from the env value's raw bytes.
///
/// Accepted forms, checked in order:
/// 1. 64-char hex string encoding 32 bytes
/// 2. standard base64 of exactly 32 bytes
/// 3. exactly 32 raw bytes (any bytes; need not be UTF-8)
pub fn parse_key_material(bytes: &[u8]) -> Result<[u8; 32], String> {
    // String forms may carry shell whitespace (e.g. trailing newline).
    let trimmed = bytes.trim_ascii();
    if let Some(key) = std::str::from_utf8(trimmed)
        .ok()
        .and_then(decode_hex_32)
    {
        return Ok(key);
    }
    if let Some(key) = std::str::from_utf8(trimmed)
        .ok()
        .and_then(decode_base64_32)
    {
        return Ok(key);
    }
    // Raw form: the bytes themselves are the key. Prefer the untrimmed value so
    // a key that ends in ASCII whitespace is not silently shortened.
    if bytes.len() == 32 {
        return Ok(bytes.try_into().expect("32 bytes"));
    }
    if trimmed.len() == 32 {
        return Ok(trimmed.try_into().expect("32 bytes"));
    }
    Err(format!(
        "{KEY_ENV} must be 32 bytes as 64-char hex, standard base64 of 32 bytes, or 32 raw bytes"
    ))
}

fn decode_hex_32(input: &str) -> Option<[u8; 32]> {
    if input.len() != 64 {
        return None;
    }
    let bytes = input.as_bytes();
    let mut out = [0u8; 32];
    for (i, slot) in out.iter_mut().enumerate() {
        let hi = hex_nibble(bytes[2 * i])?;
        let lo = hex_nibble(bytes[2 * i + 1])?;
        *slot = (hi << 4) | lo;
    }
    Some(out)
}

fn hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn decode_base64_32(input: &str) -> Option<[u8; 32]> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(input)
        .ok()?;
    decoded.try_into().ok()
}

fn ephemeral_key() -> [u8; 32] {
    use std::io::Read;
    let mut key = [0u8; 32];
    std::fs::File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut key))
        .expect("read /dev/urandom");
    key
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    /// Serializes the tests that mutate `MEDIA_GUARD_REF_KEY` (process-global).
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn env_guard() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn load_key_reads_raw_32_bytes_from_env() {
        let _guard = env_guard();
        std::env::set_var(KEY_ENV, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        let key = load_key();
        std::env::remove_var(KEY_ENV);
        assert_eq!(&key, b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    }

    #[test]
    fn load_key_reads_non_utf8_raw_bytes_from_env() {
        let _guard = env_guard();
        // No NUL bytes: env values cannot hold them.
        let mut raw = vec![0xffu8; 32];
        raw[0] = 0xfe;
        use std::os::unix::ffi::OsStringExt as _;
        std::env::set_var(KEY_ENV, std::ffi::OsString::from_vec(raw.clone()));
        let key = load_key();
        std::env::remove_var(KEY_ENV);
        assert_eq!(key.as_slice(), raw.as_slice());
    }

    #[test]
    #[should_panic(expected = "64-char hex")]
    fn load_key_panics_on_malformed_env() {
        let _guard = env_guard();
        std::env::set_var(KEY_ENV, "not-a-valid-key");
        let _ = load_key();
    }
}
