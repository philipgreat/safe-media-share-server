#[derive(Clone)]
pub struct IpAccessGrantExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::IpAccessGrant>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> IpAccessGrantExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::IpAccessGrant>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::IpAccessGrant> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::IpAccessGrant> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::IpAccessGrant {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_client_ip(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("client_ip", |entity| entity.eval_client_ip());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_expire_at(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("expire_at", |entity| entity.eval_expire_at());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_grant_note(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("grant_note", |entity| entity.eval_grant_note());
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
    pub fn get_media_library_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("media_library_id", |entity| entity.eval_media_library_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_media_asset_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("media_asset_id", |entity| entity.eval_media_asset_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_grant_status_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("grant_status_id", |entity| entity.eval_grant_status_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_media_library(self) -> crate::MediaLibraryExpression<'a> {
        let next = self.result.and_then("media_library", |entity| entity.eval_media_library());
        crate::MediaLibraryExpression::new(next, self.root_desc.clone())
    }

    pub fn get_media_asset(self) -> crate::MediaAssetExpression<'a> {
        let next = self.result.and_then("media_asset", |entity| entity.eval_media_asset());
        crate::MediaAssetExpression::new(next, self.root_desc.clone())
    }

    pub fn get_grant_status(self) -> crate::GrantStatusExpression<'a> {
        let next = self.result.and_then("grant_status", |entity| entity.eval_grant_status());
        crate::GrantStatusExpression::new(next, self.root_desc.clone())
    }
    pub fn grant_status_is_active(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("grant_status_id", |entity| {
            if !entity.is_loaded("grant_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "grant_status_id".to_string(), attempted_path: "grant_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.grant_status_is_active())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn grant_status_is_expired(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("grant_status_id", |entity| {
            if !entity.is_loaded("grant_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "grant_status_id".to_string(), attempted_path: "grant_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.grant_status_is_expired())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn grant_status_is_revoked(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("grant_status_id", |entity| {
            if !entity.is_loaded("grant_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "grant_status_id".to_string(), attempted_path: "grant_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.grant_status_is_revoked())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct IpAccessGrantListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::IpAccessGrant>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> IpAccessGrantListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::IpAccessGrant>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::IpAccessGrant>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::IpAccessGrant>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::IpAccessGrant> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::IpAccessGrantExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::IpAccessGrantExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::IpAccessGrantExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::IpAccessGrantExpression::new(next, self.root_desc.clone())
    }
}