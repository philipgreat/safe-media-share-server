//! Entity reference codec smoke tests (guest ID hiding).

use std::time::Duration;

use teaql_runtime::{AeadEntityReferenceCodec, EntityReferenceCodec};

use media_guard_service_core_web_axum::entity_ref::parse_key_material;

fn codec() -> AeadEntityReferenceCodec {
    AeadEntityReferenceCodec::new(1, [(1u32, vec![7u8; 32])]).expect("codec")
}

#[test]
fn encode_decode_round_trip_hides_raw_id() {
    // Long id: substring "contains" checks against short numbers (e.g. 42)
    // flake on random nonce/ciphertext bytes in the base64 envelope.
    const ASSET_ID: u64 = 987_654_321_012;
    let codec = codec();
    let token = codec
        .encode(
            "MediaAsset",
            ASSET_ID,
            0,
            "media-access",
            Duration::from_secs(60),
        )
        .expect("encode");
    assert!(token.starts_with("tqr1."), "token={token}");
    assert!(
        !token.contains(&ASSET_ID.to_string()),
        "raw id leaked in token"
    );
    let claims = codec
        .decode(&token, "MediaAsset", "media-access")
        .expect("decode");
    assert_eq!(claims.id, ASSET_ID);
    assert_eq!(claims.entity_type, "MediaAsset");
    assert_eq!(claims.purpose, "media-access");
}

#[test]
fn decode_rejects_wrong_purpose() {
    let codec = codec();
    let token = codec
        .encode("MediaAsset", 42, 0, "media-access", Duration::from_secs(60))
        .expect("encode");
    let err = codec
        .decode(&token, "MediaAsset", "other")
        .expect_err("purpose must match");
    assert_eq!(err.code(), "ENTITY_REFERENCE_INVALID");
}

#[test]
fn decode_rejects_garbage() {
    let codec = codec();
    let err = codec
        .decode("tqr1.not-a-token", "MediaAsset", "media-access")
        .expect_err("garbage");
    assert_eq!(err.code(), "ENTITY_REFERENCE_INVALID");
}

/// `encode_asset_ref` / `decode_asset_ref` go through `UserContext` and the
/// shared codec installed by `build_request_context`.
mod helpers {
    use std::sync::Arc;
    use std::time::Duration;

    use teaql_runtime::{AeadEntityReferenceCodec, UserContext};

    use media_guard_service_core_web_axum::entity_ref::{
        self, decode_asset_ref, encode_asset_ref, ENTITY_TYPE, PURPOSE, REF_LIFETIME_SECS,
    };

    fn context() -> UserContext {
        let codec = AeadEntityReferenceCodec::new(1, [(1u32, vec![9u8; 32])]).expect("codec");
        let mut context = UserContext::new();
        context.set_entity_reference_codec(Arc::new(codec));
        context
    }

    #[test]
    fn constants_match_spec() {
        assert_eq!(ENTITY_TYPE, "MediaAsset");
        assert_eq!(PURPOSE, "media-access");
        assert_eq!(REF_LIFETIME_SECS, 7 * 24 * 3600);
    }

    #[test]
    fn helper_round_trip_hides_raw_id() {
        // Long id: avoid flaky substring checks against random token bytes.
        const ASSET_ID: u64 = 987_654_321_012;
        let context = context();
        let token = encode_asset_ref(&context, ASSET_ID).expect("encode");
        assert!(token.starts_with("tqr1."), "token={token}");
        assert!(
            !token.contains(&ASSET_ID.to_string()),
            "raw id leaked in token"
        );
        assert_eq!(decode_asset_ref(&context, &token).expect("decode"), ASSET_ID);
    }

    #[test]
    fn helper_rejects_garbage_and_wrong_purpose() {
        let context = context();
        let err = decode_asset_ref(&context, "tqr1.not-a-token").expect_err("garbage");
        assert_eq!(err, "invalid media reference");

        let token = encode_asset_ref(&context, 42).expect("encode");
        let claims = context
            .decode_entity_reference(&token, ENTITY_TYPE, "other")
            .expect_err("purpose must match");
        assert_eq!(claims.code(), "ENTITY_REFERENCE_INVALID");
    }

    #[test]
    fn shared_codec_is_constructible_and_encodes() {
        let codec = entity_ref::shared_codec();
        use teaql_runtime::EntityReferenceCodec as _;
        let token = codec
            .encode(ENTITY_TYPE, 7, 0, PURPOSE, Duration::from_secs(60))
            .expect("encode");
        assert!(token.starts_with("tqr1."));
    }
}

/// Spec [S5] session fingerprint binding: refs are only valid for the session
/// that minted them (`purpose = media-access:{fp}`; empty fp keeps bare
/// `media-access` for backward compat).
mod session_purpose {
    use std::sync::Arc;

    use teaql_runtime::{AeadEntityReferenceCodec, UserContext};

    use media_guard_service_core_web_axum::entity_ref::{
        decode_asset_ref, decode_asset_ref_for_session, encode_asset_ref,
        encode_asset_ref_for_session, purpose_for_session, INVALID_REF, PURPOSE,
    };

    fn context() -> UserContext {
        let codec = AeadEntityReferenceCodec::new(1, [(1u32, vec![9u8; 32])]).expect("codec");
        let mut context = UserContext::new();
        context.set_entity_reference_codec(Arc::new(codec));
        context
    }

    #[test]
    fn purpose_for_session_empty_fp_keeps_base_purpose() {
        assert_eq!(purpose_for_session(""), PURPOSE);
        assert_eq!(purpose_for_session(""), "media-access");
    }

    #[test]
    fn purpose_for_session_binds_fingerprint() {
        assert_eq!(purpose_for_session("abc123"), "media-access:abc123");
    }

    #[test]
    fn same_fingerprint_round_trip_hides_raw_id() {
        const ASSET_ID: u64 = 987_654_321_012;
        let context = context();
        let token =
            encode_asset_ref_for_session(&context, ASSET_ID, "fp-session-1").expect("encode");
        assert!(token.starts_with("tqr1."), "token={token}");
        assert!(
            !token.contains(&ASSET_ID.to_string()),
            "raw id leaked in token"
        );
        assert_eq!(
            decode_asset_ref_for_session(&context, &token, "fp-session-1").expect("decode"),
            ASSET_ID
        );
    }

    #[test]
    fn different_fingerprint_rejects_token() {
        let context = context();
        let token = encode_asset_ref_for_session(&context, 42, "fp-session-1").expect("encode");
        let err = decode_asset_ref_for_session(&context, &token, "fp-session-2")
            .expect_err("other session must not decode");
        assert_eq!(err, INVALID_REF);
    }

    #[test]
    fn empty_fp_is_interoperable_with_media_access() {
        const ASSET_ID: u64 = 987_654_321_012;
        let context = context();

        // Session helper with empty fp == legacy `media-access` behavior.
        let token = encode_asset_ref_for_session(&context, ASSET_ID, "").expect("encode");
        assert_eq!(
            decode_asset_ref(&context, &token).expect("legacy decode"),
            ASSET_ID
        );
        assert_eq!(
            decode_asset_ref_for_session(&context, &token, "").expect("session decode"),
            ASSET_ID
        );

        // Legacy encode remains readable through the session helper with empty fp.
        let legacy = encode_asset_ref(&context, ASSET_ID).expect("legacy encode");
        assert_eq!(
            decode_asset_ref_for_session(&context, &legacy, "").expect("session decode"),
            ASSET_ID
        );
    }

    #[test]
    fn session_ref_not_readable_by_empty_fp_or_vice_versa() {
        let context = context();
        let bound = encode_asset_ref_for_session(&context, 7, "fp-a").expect("encode");
        let err = decode_asset_ref_for_session(&context, &bound, "").expect_err("empty fp");
        assert_eq!(err, INVALID_REF);

        let bare = encode_asset_ref_for_session(&context, 7, "").expect("encode");
        let err = decode_asset_ref_for_session(&context, &bare, "fp-a").expect_err("bound fp");
        assert_eq!(err, INVALID_REF);
    }
}

/// Spec [S3] key forms: 64-char hex, standard base64 of 32 bytes, 32 raw bytes.
mod key_material {
    use base64::Engine as _;

    use super::parse_key_material;

    #[test]
    fn accepts_64_char_hex() {
        let hex = "a1b2c3d4".repeat(8); // 64 chars
        let key = parse_key_material(hex.as_bytes()).expect("hex key");
        assert_eq!(key[0], 0xa1);
        assert_eq!(key[1], 0xb2);
        assert_eq!(key[31], 0xd4);
    }

    #[test]
    fn accepts_standard_base64_of_32_bytes() {
        let material = [7u8; 32];
        let encoded = base64::engine::general_purpose::STANDARD.encode(material);
        let key = parse_key_material(encoded.as_bytes()).expect("base64 key");
        assert_eq!(key, material);
    }

    #[test]
    fn accepts_raw_32_bytes() {
        // 32 ASCII bytes that are neither 64-char hex nor base64 of 32 bytes.
        let raw = b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        let key = parse_key_material(raw).expect("raw key");
        assert_eq!(&key, raw);
    }

    #[test]
    fn accepts_raw_32_bytes_that_are_not_utf8() {
        let raw = [0xffu8; 32];
        let key = parse_key_material(&raw).expect("non-utf8 raw key");
        assert_eq!(key, raw);
    }

    #[test]
    fn rejects_malformed_with_message_listing_all_forms() {
        for bad in [&b""[..], b"short", &b"z".repeat(64)[..], &[0xffu8; 33][..]] {
            let err = parse_key_material(bad).expect_err("must reject");
            assert!(err.contains("64-char hex"), "err={err}");
            assert!(err.contains("base64"), "err={err}");
            assert!(err.contains("raw"), "err={err}");
        }
    }
}
