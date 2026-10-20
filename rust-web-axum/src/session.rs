
//! Browser session cookie (`mg_sid`) for fingerprint-bound media refs ([S5]).

use axum::http::{HeaderMap, HeaderValue};

/// HttpOnly session cookie name.
pub const SESSION_COOKIE: &str = "mg_sid";

/// Cookie value form: `mg_sid=<fp>; HttpOnly; Path=/; SameSite=Lax`.
fn cookie_header_value(fp: &str) -> String {
    format!("{SESSION_COOKIE}={fp}; HttpOnly; Path=/; SameSite=Lax")
}

/// Opaque session fingerprint: 32 hex chars (16 random bytes).
pub fn generate_session_fp() -> String {
    let mut bytes = [0u8; 16];
    let mut filled = false;
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        use std::io::Read as _;
        filled = f.read_exact(&mut bytes).is_ok();
    }
    if !filled {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};
        let mut h = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
            .hash(&mut h);
        std::process::id().hash(&mut h);
        std::thread::current().id().hash(&mut h);
        let v = h.finish();
        bytes[..8].copy_from_slice(&v.to_le_bytes());
        let v2 = v.wrapping_mul(0x9e37_79b9_7f4a_7c15).rotate_left(17);
        bytes[8..].copy_from_slice(&v2.to_le_bytes());
    }
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// True for a well-formed session fingerprint (32 hex chars).
pub fn is_valid_session_fp(fp: &str) -> bool {
    fp.len() == 32 && fp.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Read one cookie value from `Cookie` request headers.
pub fn read_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    for value in headers.get_all(axum::http::header::COOKIE) {
        let Ok(raw) = value.to_str() else {
            continue;
        };
        for pair in raw.split(';') {
            let pair = pair.trim();
            let Some(rest) = pair.strip_prefix(name) else {
                continue;
            };
            let Some(val) = rest.strip_prefix('=') else {
                continue;
            };
            let val = val.trim();
            if !val.is_empty() {
                return Some(val.to_owned());
            }
        }
    }
    None
}

/// Existing session fingerprint from `mg_sid` (empty when absent or malformed).
///
/// Decode paths use this so a request without a cookie never fabricates an fp.
pub fn session_fp_from_cookie(headers: &HeaderMap) -> String {
    read_cookie(headers, SESSION_COOKIE)
        .filter(|v| is_valid_session_fp(v))
        .unwrap_or_default()
}

/// Session fingerprint for minting: reuse `mg_sid` or create one.
///
/// Returns `(fp, Some(set_cookie))` when the minting response must attach a
/// fresh `mg_sid` cookie ([S5]).
pub fn session_fp_from_headers(headers: &HeaderMap) -> (String, Option<String>) {
    if let Some(fp) = read_cookie(headers, SESSION_COOKIE).filter(|v| is_valid_session_fp(v)) {
        return (fp, None);
    }
    let fp = generate_session_fp();
    let cookie = cookie_header_value(&fp);
    (fp, Some(cookie))
}

/// Attach `Set-Cookie` when the session cookie was newly minted.
pub fn attach_session_cookie(
    resp: axum::response::Response,
    set_cookie: Option<String>,
) -> axum::response::Response {
    let Some(cookie) = set_cookie else {
        return resp;
    };
    let (mut parts, body) = resp.into_parts();
    if let Ok(val) = HeaderValue::from_str(&cookie) {
        parts.headers.append(axum::http::header::SET_COOKIE, val);
    }
    axum::response::Response::from_parts(parts, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers_with(cookie: &str) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(axum::http::header::COOKIE, HeaderValue::from_str(cookie).unwrap());
        h
    }

    #[test]
    fn generate_session_fp_is_32_hex() {
        let fp = generate_session_fp();
        assert!(is_valid_session_fp(&fp), "fp={fp}");
    }

    #[test]
    fn session_fp_reads_existing_cookie() {
        let fp = "a".repeat(32);
        let (got, cookie) = session_fp_from_headers(&headers_with(&format!("mg_sid={fp}; Path=/")));
        assert_eq!(got, fp);
        assert!(cookie.is_none());
    }

    #[test]
    fn session_fp_generates_when_missing_or_malformed() {
        let (got, cookie) = session_fp_from_headers(&HeaderMap::new());
        assert!(is_valid_session_fp(&got));
        let cookie = cookie.expect("Set-Cookie");
        assert!(cookie.starts_with("mg_sid="));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Lax"));

        let (got2, cookie2) = session_fp_from_headers(&headers_with("mg_sid=short"));
        assert!(is_valid_session_fp(&got2));
        assert!(cookie2.is_some());
    }

    #[test]
    fn session_fp_ignores_other_cookies_and_prefers_exact_name() {
        let fp = "c".repeat(32);
        let (got, cookie) =
            session_fp_from_headers(&headers_with(&format!("other=1; xmg_sid=zzz; mg_sid={fp}")));
        assert_eq!(got, fp);
        assert!(cookie.is_none());
    }

    #[test]
    fn session_fp_from_cookie_is_empty_without_valid_value() {
        assert_eq!(session_fp_from_cookie(&HeaderMap::new()), "");
        assert_eq!(session_fp_from_cookie(&headers_with("mg_sid=nope")), "");
    }
}
