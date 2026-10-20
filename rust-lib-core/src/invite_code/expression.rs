#[derive(Clone)]
pub struct InviteCodeExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::InviteCode>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InviteCodeExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::InviteCode>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::InviteCode> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::InviteCode> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::InviteCode {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_invite_token(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("invite_token", |entity| entity.eval_invite_token());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_first_client_ip(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("first_client_ip", |entity| entity.eval_first_client_ip());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_first_click_at(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("first_click_at", |entity| entity.eval_first_click_at());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_second_client_ip(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("second_client_ip", |entity| entity.eval_second_client_ip());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_second_click_at(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("second_click_at", |entity| entity.eval_second_click_at());
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

    pub fn get_invite_status_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("invite_status_id", |entity| entity.eval_invite_status_id());
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

    pub fn get_invite_status(self) -> crate::InviteStatusExpression<'a> {
        let next = self.result.and_then("invite_status", |entity| entity.eval_invite_status());
        crate::InviteStatusExpression::new(next, self.root_desc.clone())
    }
    pub fn invite_status_is_fresh(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("invite_status_id", |entity| {
            if !entity.is_loaded("invite_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "invite_status_id".to_string(), attempted_path: "invite_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.invite_status_is_fresh())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn invite_status_is_claimed(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("invite_status_id", |entity| {
            if !entity.is_loaded("invite_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "invite_status_id".to_string(), attempted_path: "invite_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.invite_status_is_claimed())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn invite_status_is_spent(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("invite_status_id", |entity| {
            if !entity.is_loaded("invite_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "invite_status_id".to_string(), attempted_path: "invite_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.invite_status_is_spent())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct InviteCodeListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InviteCode>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InviteCodeListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InviteCode>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::InviteCode>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::InviteCode>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::InviteCode> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::InviteCodeExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InviteCodeExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::InviteCodeExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InviteCodeExpression::new(next, self.root_desc.clone())
    }
}