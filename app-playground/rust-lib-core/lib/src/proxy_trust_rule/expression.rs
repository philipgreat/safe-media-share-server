#[derive(Clone)]
pub struct ProxyTrustRuleExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::ProxyTrustRule>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ProxyTrustRuleExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::ProxyTrustRule>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::ProxyTrustRule> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::ProxyTrustRule> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::ProxyTrustRule {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_header_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("header_name", |entity| entity.eval_header_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_trusted_proxy_cidrs(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("trusted_proxy_cidrs", |entity| entity.eval_trusted_proxy_cidrs());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_enabled(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("enabled", |entity| entity.eval_enabled());
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
    pub fn get_media_library(self) -> crate::MediaLibraryExpression<'a> {
        let next = self.result.and_then("media_library", |entity| entity.eval_media_library());
        crate::MediaLibraryExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct ProxyTrustRuleListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ProxyTrustRule>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ProxyTrustRuleListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ProxyTrustRule>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::ProxyTrustRule>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::ProxyTrustRule>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::ProxyTrustRule> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::ProxyTrustRuleExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ProxyTrustRuleExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::ProxyTrustRuleExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ProxyTrustRuleExpression::new(next, self.root_desc.clone())
    }
}