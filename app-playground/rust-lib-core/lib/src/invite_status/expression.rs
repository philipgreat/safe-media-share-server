#[derive(Clone)]
pub struct InviteStatusExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::InviteStatus>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InviteStatusExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::InviteStatus>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::InviteStatus> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::InviteStatus> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::InviteStatus {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_code(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("code", |entity| entity.eval_code());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_display_order(self) -> crate::ValueExpression<'a, rust_decimal::Decimal> {
        let next = self.result.and_then("display_order", |entity| entity.eval_display_order());
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
    pub fn get_media_library(self) -> crate::MediaLibraryExpression<'a> {
        let next = self.result.and_then("media_library", |entity| entity.eval_media_library());
        crate::MediaLibraryExpression::new(next, self.root_desc.clone())
    }
    pub fn get_invite_code_list(self) -> crate::InviteCodeListExpression<'a> {
        let next = self.result.and_then("invite_code_list", |entity| entity.eval_invite_code_list());
        crate::InviteCodeListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct InviteStatusListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InviteStatus>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InviteStatusListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InviteStatus>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::InviteStatus>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::InviteStatus>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::InviteStatus> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::InviteStatusExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InviteStatusExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::InviteStatusExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InviteStatusExpression::new(next, self.root_desc.clone())
    }
}