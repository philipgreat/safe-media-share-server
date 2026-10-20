//! Entity reference codec smoke tests (guest ID hiding).

use std::time::Duration;

use teaql_runtime::{AeadEntityReferenceCodec, EntityReferenceCodec};

fn codec() -> AeadEntityReferenceCodec {
    AeadEntityReferenceCodec::new(1, [(1u32, vec![7u8; 32])]).expect("codec")
}

#[test]
fn encode_decode_round_trip_hides_raw_id() {
    let codec = codec();
    let token = codec
        .encode("MediaAsset", 42, 0, "media-access", Duration::from_secs(60))
        .expect("encode");
    assert!(token.starts_with("tqr1."), "token={token}");
    assert!(!token.contains("42"), "raw id leaked in token");
    let claims = codec
        .decode(&token, "MediaAsset", "media-access")
        .expect("decode");
    assert_eq!(claims.id, 42);
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
        let context = context();
        let token = encode_asset_ref(&context, 42).expect("encode");
        assert!(token.starts_with("tqr1."), "token={token}");
        assert!(!token.contains("42"), "raw id leaked in token");
        assert_eq!(decode_asset_ref(&context, &token).expect("decode"), 42);
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
