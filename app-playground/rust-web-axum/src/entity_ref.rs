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
const INVALID_REF: &str = "invalid media reference";

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
        .map_err(|_| INVALID_REF.to_owned())
}

/// Decode a guest-facing reference back to the real media asset id.
///
/// All failures map to one message so raw ids and crypto details never leak.
pub fn decode_asset_ref(context: &UserContext, token: &str) -> Result<u64, String> {
    context
        .decode_entity_reference(token, ENTITY_TYPE, PURPOSE)
        .map(|claims| claims.id)
        .map_err(|_| INVALID_REF.to_owned())
}

/// Key load order: `MEDIA_GUARD_REF_KEY` as 64-char hex, then standard base64
/// of 32 bytes, else an ephemeral random key (trial only).
fn load_key() -> [u8; 32] {
    if let Ok(raw) = std::env::var(KEY_ENV) {
        let raw = raw.trim();
        if let Some(key) = decode_hex_32(raw) {
            return key;
        }
        if let Some(key) = decode_base64_32(raw) {
            return key;
        }
        panic!("{KEY_ENV} must be 32 bytes as 64-char hex or standard base64");
    }
    eprintln!("WARN: {KEY_ENV} not set; using ephemeral entity reference key (trial only, refs die on restart)");
    ephemeral_key()
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
