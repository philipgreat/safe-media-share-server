use teaql_core::Expr;

use crate::*;

pub struct PurposedQuery<T> {
    pub inner: T,
    pub purpose: String,
}

impl<T> PurposedQuery<T> {
    pub fn new(inner: T, purpose: impl Into<String>) -> Self {
        let purpose = purpose.into();
        assert!(!purpose.trim().is_empty(), "query purpose must not be empty");
        Self { inner, purpose }
    }
}

pub struct Q;

impl Q {
    pub fn media_libraries() -> MediaLibraryRequest {
        MediaLibraryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn media_libraries_minimal() -> MediaLibraryRequest {
        MediaLibraryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn media_libraries_with_children() -> MediaLibraryRequest {
        MediaLibraryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }



    pub fn media_assets() -> MediaAssetRequest {
        MediaAssetRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn media_assets_minimal() -> MediaAssetRequest {
        MediaAssetRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn media_assets_with_children() -> MediaAssetRequest {
        MediaAssetRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }



    pub fn grant_statuses() -> GrantStatusRequest {
        GrantStatusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn grant_statuses_minimal() -> GrantStatusRequest {
        GrantStatusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn grant_statuses_with_children() -> GrantStatusRequest {
        GrantStatusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }



    pub fn ip_access_grants() -> IpAccessGrantRequest {
        IpAccessGrantRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn ip_access_grants_minimal() -> IpAccessGrantRequest {
        IpAccessGrantRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn ip_access_grants_with_children() -> IpAccessGrantRequest {
        IpAccessGrantRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }



    pub fn invite_statuses() -> InviteStatusRequest {
        InviteStatusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invite_statuses_minimal() -> InviteStatusRequest {
        InviteStatusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invite_statuses_with_children() -> InviteStatusRequest {
        InviteStatusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }



    pub fn invite_codes() -> InviteCodeRequest {
        InviteCodeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invite_codes_minimal() -> InviteCodeRequest {
        InviteCodeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invite_codes_with_children() -> InviteCodeRequest {
        InviteCodeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }



    pub fn app_settings() -> AppSettingRequest {
        AppSettingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn app_settings_minimal() -> AppSettingRequest {
        AppSettingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn app_settings_with_children() -> AppSettingRequest {
        AppSettingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }



    pub fn proxy_trust_rules() -> ProxyTrustRuleRequest {
        ProxyTrustRuleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn proxy_trust_rules_minimal() -> ProxyTrustRuleRequest {
        ProxyTrustRuleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn proxy_trust_rules_with_children() -> ProxyTrustRuleRequest {
        ProxyTrustRuleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }



    pub fn share_handles() -> ShareHandleRequest {
        ShareHandleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn share_handles_minimal() -> ShareHandleRequest {
        ShareHandleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn share_handles_with_children() -> ShareHandleRequest {
        ShareHandleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }



    pub fn user_sessions() -> UserSessionRequest {
        UserSessionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_sessions_minimal() -> UserSessionRequest {
        UserSessionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_sessions_with_children() -> UserSessionRequest {
        UserSessionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }


}