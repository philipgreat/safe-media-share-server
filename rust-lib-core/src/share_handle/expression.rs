#[derive(Clone)]
pub struct ShareHandleExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::ShareHandle>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ShareHandleExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::ShareHandle>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::ShareHandle> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::ShareHandle> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::ShareHandle {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_handle_token(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("handle_token", |entity| entity.eval_handle_token());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_active(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("active", |entity| entity.eval_active());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_handle_note(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("handle_note", |entity| entity.eval_handle_note());
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
    pub fn get_media_library(self) -> crate::MediaLibraryExpression<'a> {
        let next = self.result.and_then("media_library", |entity| entity.eval_media_library());
        crate::MediaLibraryExpression::new(next, self.root_desc.clone())
    }

    pub fn get_media_asset(self) -> crate::MediaAssetExpression<'a> {
        let next = self.result.and_then("media_asset", |entity| entity.eval_media_asset());
        crate::MediaAssetExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct ShareHandleListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ShareHandle>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ShareHandleListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ShareHandle>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::ShareHandle>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::ShareHandle>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::ShareHandle> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::ShareHandleExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ShareHandleExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::ShareHandleExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ShareHandleExpression::new(next, self.root_desc.clone())
    }
}