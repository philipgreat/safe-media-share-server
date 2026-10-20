#[derive(Clone)]
pub struct MediaLibraryExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::MediaLibrary>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MediaLibraryExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::MediaLibrary>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::MediaLibrary> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::MediaLibrary> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::MediaLibrary {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_library_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("library_name", |entity| entity.eval_library_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_root_path(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("root_path", |entity| entity.eval_root_path());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_create_time(self) -> crate::ValueExpression<'a, teaql_core::time::Timestamp> {
        let next = self.result.and_then("create_time", |entity| entity.eval_create_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_update_time(self) -> crate::ValueExpression<'a, teaql_core::time::Timestamp> {
        let next = self.result.and_then("update_time", |entity| entity.eval_update_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_media_asset_list(self) -> crate::MediaAssetListExpression<'a> {
        let next = self.result.and_then("media_asset_list", |entity| entity.eval_media_asset_list());
        crate::MediaAssetListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_grant_status_list(self) -> crate::GrantStatusListExpression<'a> {
        let next = self.result.and_then("grant_status_list", |entity| entity.eval_grant_status_list());
        crate::GrantStatusListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_ip_access_grant_list(self) -> crate::IpAccessGrantListExpression<'a> {
        let next = self.result.and_then("ip_access_grant_list", |entity| entity.eval_ip_access_grant_list());
        crate::IpAccessGrantListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_invite_status_list(self) -> crate::InviteStatusListExpression<'a> {
        let next = self.result.and_then("invite_status_list", |entity| entity.eval_invite_status_list());
        crate::InviteStatusListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_invite_code_list(self) -> crate::InviteCodeListExpression<'a> {
        let next = self.result.and_then("invite_code_list", |entity| entity.eval_invite_code_list());
        crate::InviteCodeListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_app_setting_list(self) -> crate::AppSettingListExpression<'a> {
        let next = self.result.and_then("app_setting_list", |entity| entity.eval_app_setting_list());
        crate::AppSettingListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_proxy_trust_rule_list(self) -> crate::ProxyTrustRuleListExpression<'a> {
        let next = self.result.and_then("proxy_trust_rule_list", |entity| entity.eval_proxy_trust_rule_list());
        crate::ProxyTrustRuleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_share_handle_list(self) -> crate::ShareHandleListExpression<'a> {
        let next = self.result.and_then("share_handle_list", |entity| entity.eval_share_handle_list());
        crate::ShareHandleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_user_session_list(self) -> crate::UserSessionListExpression<'a> {
        let next = self.result.and_then("user_session_list", |entity| entity.eval_user_session_list());
        crate::UserSessionListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct MediaLibraryListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MediaLibrary>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MediaLibraryListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MediaLibrary>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::MediaLibrary>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::MediaLibrary>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::MediaLibrary> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::MediaLibraryExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MediaLibraryExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::MediaLibraryExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MediaLibraryExpression::new(next, self.root_desc.clone())
    }
}