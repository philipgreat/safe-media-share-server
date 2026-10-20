
// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/proxy_trust_rule
use std::collections::BTreeMap;

use teaql_macros::{teaql_entity, TeaqlEntity};


/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[teaql_entity]
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(indexed_layout)]
#[teaql(entity = "ProxyTrustRule", table = "proxy_trust_rule_data", data_service = "sqlite", audit_mask_fields = "")]
pub struct ProxyTrustRule {
#[teaql(id)]
    id: u64,

// @source model.xml:106
#[teaql(max_length = 100)]
    header_name: String,

// @source model.xml:106
#[teaql(max_length = 100)]
    trusted_proxy_cidrs: String,

// @source model.xml:106
    enabled: bool,

// @source model.xml:106
    create_time: teaql_core::time::Timestamp,

// @source model.xml:106
    update_time: teaql_core::time::Timestamp,
#[teaql(version)]
    version: i64,
// @source model.xml:106
#[teaql(column = "media_library")]
    media_library_id: u64,
// @source model.xml:106
#[teaql(relation(target = "MediaLibrary", local_key = "media_library_id", foreign_key = "id"))]
    media_library: Option<Box<crate::MediaLibrary>>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl ProxyTrustRule {
    pub const ENTITY_NAME: &'static str = "Proxy Trust Rule";

    // Framework metadata: complete fixed layout, including overflow slots.
    pub const __TEAQL_FIELD_LAYOUT_REVISION: &'static str = "5caeeea9d49ba2e0a5af1ac1ef3312621b84fd90441cf2ab5c5065be0691ee0c";
    pub const __TEAQL_FIXED_FIELD_INDEXES: &'static [(&'static str, usize)] = &[
        ("id", 0),
        ("media_library", 1),
        ("header_name", 2),
        ("trusted_proxy_cidrs", 3),
        ("enabled", 4),
        ("create_time", 5),
        ("update_time", 6),
        ("version", 7),
    ];
    pub const __TEAQL_FIXED_FIELD_MAPPINGS: &'static [(&'static str, &'static str, &'static str)] = &[
        ("id", "id", "id"),
        ("media_library", "media_library_id", "media_library"),
        ("header_name", "header_name", "header_name"),
        ("trusted_proxy_cidrs", "trusted_proxy_cidrs", "trusted_proxy_cidrs"),
        ("enabled", "enabled", "enabled"),
        ("create_time", "create_time", "create_time"),
        ("update_time", "update_time", "update_time"),
        ("version", "version", "version"),
    ];

    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRuntimeState) -> Self {
        Self {
            id: 0_u64,
            header_name: String::new(),
            trusted_proxy_cidrs: String::new(),
            enabled: false,
            create_time: teaql_core::time::Timestamp::now(),
            update_time: teaql_core::time::Timestamp::now(),
            version: 0_i64,
            media_library_id: 0_u64,
            media_library: None,
            dynamic: BTreeMap::new(),
            __teaql_runtime_state: root,
            __load_state: teaql_core::eval::LoadState::NotLoaded.into_indexed(
                <Self as teaql_core::TeaqlEntity>::field_layout().expect("generated field layout").expect("indexed layout")
            ).expect("new entity load state"),
        }
    }

    pub fn attach_runtime_state_recursive(&mut self, root: teaql_runtime::EntityRuntimeState) {
        root.adopt_mutations_from(self.__teaql_runtime_state());
        self.__teaql_replace_runtime_state(root.clone());
        if let Some(entity) = &mut self.media_library {
            entity.attach_runtime_state_recursive(root.clone());
        }
    }

    pub fn is_loaded(&self, field_or_relation: &str) -> bool {
        self.__load_state.is_loaded(field_or_relation)
    }

    pub fn set_load_state(&mut self, state: teaql_core::eval::LoadState) {
        self.__load_state = state.into_indexed(
            <Self as teaql_core::TeaqlEntity>::field_layout().expect("generated field layout").expect("indexed layout")
        ).expect("compatible entity load state");
    }

    pub fn id(&self) -> u64 {
        self.changed_id().and_then(|value| value.try_u64()).unwrap_or(self.id)
    }

    pub fn update_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.id = value.try_u64().unwrap_or(self.id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "id", value);
        self.__load_state.mark_loaded("id").expect("generated field layout");
        self
    }

    pub fn changed_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "id")
    }

    pub fn eval_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "id".to_string(), attempted_path: "id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.id())
                }}


    pub fn header_name(&self) -> String {
        self.changed_header_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.header_name.clone())
    }

    pub fn update_header_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.header_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.header_name.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "header_name", value);
        self.__load_state.mark_loaded("header_name").expect("generated field layout");
        self
    }

    pub fn changed_header_name(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "header_name")
    }

    pub fn eval_header_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("header_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "header_name".to_string(), attempted_path: "header_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.header_name())
                }}


    pub fn trusted_proxy_cidrs(&self) -> String {
        self.changed_trusted_proxy_cidrs().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.trusted_proxy_cidrs.clone())
    }

    pub fn update_trusted_proxy_cidrs(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.trusted_proxy_cidrs = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.trusted_proxy_cidrs.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "trusted_proxy_cidrs", value);
        self.__load_state.mark_loaded("trusted_proxy_cidrs").expect("generated field layout");
        self
    }

    pub fn changed_trusted_proxy_cidrs(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "trusted_proxy_cidrs")
    }

    pub fn eval_trusted_proxy_cidrs(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("trusted_proxy_cidrs") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "trusted_proxy_cidrs".to_string(), attempted_path: "trusted_proxy_cidrs".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.trusted_proxy_cidrs())
                }}


    pub fn enabled(&self) -> bool {
        self.changed_enabled().and_then(|value| value.try_bool()).unwrap_or(self.enabled)
    }

    pub fn update_enabled(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.enabled = value.try_bool().unwrap_or(self.enabled.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "enabled", value);
        self.__load_state.mark_loaded("enabled").expect("generated field layout");
        self
    }

    pub fn changed_enabled(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "enabled")
    }

    pub fn eval_enabled(&self) -> teaql_core::eval::EvalResult<bool> {
        if !self.is_loaded("enabled") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "enabled".to_string(), attempted_path: "enabled".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.enabled())
                }}


    pub fn create_time(&self) -> teaql_core::time::Timestamp {
        self.changed_create_time().and_then(|value| value.try_timestamp()).unwrap_or(self.create_time)
    }

    pub fn update_create_time(&mut self, value: teaql_core::time::Timestamp) -> &mut Self {
        self.create_time = value;
        let value = teaql_core::Value::from(value);
        self.__teaql_runtime_state().set(self.entity_key(), "create_time", value);
        self.__load_state.mark_loaded("create_time").expect("generated field layout");
        self
    }
    pub fn changed_create_time(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "create_time")
    }

    pub fn eval_create_time(&self) -> teaql_core::eval::EvalResult<teaql_core::time::Timestamp> {
        if !self.is_loaded("create_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "create_time".to_string(), attempted_path: "create_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.create_time())
                }}


    pub fn update_time(&self) -> teaql_core::time::Timestamp {
        self.changed_update_time().and_then(|value| value.try_timestamp()).unwrap_or(self.update_time)
    }

    pub fn update_update_time(&mut self, value: teaql_core::time::Timestamp) -> &mut Self {
        self.update_time = value;
        let value = teaql_core::Value::from(value);
        self.__teaql_runtime_state().set(self.entity_key(), "update_time", value);
        self.__load_state.mark_loaded("update_time").expect("generated field layout");
        self
    }
    pub fn changed_update_time(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "update_time")
    }

    pub fn eval_update_time(&self) -> teaql_core::eval::EvalResult<teaql_core::time::Timestamp> {
        if !self.is_loaded("update_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "update_time".to_string(), attempted_path: "update_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.update_time())
                }}


    pub fn version(&self) -> i64 {
        self.changed_version().and_then(|value| value.try_i64()).unwrap_or(self.version)
    }

    pub fn update_version(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version = value.try_i64().unwrap_or(self.version.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "version", value);
        self.__load_state.mark_loaded("version").expect("generated field layout");
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "version")
    }

    pub fn eval_version(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("version") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version".to_string(), attempted_path: "version".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version())
                }}

    pub fn media_library_id(&self) -> u64 {
        self.changed_media_library_id().and_then(|value| value.try_u64()).unwrap_or(self.media_library_id)
    }

    pub fn update_media_library_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.media_library_id = value.try_u64().unwrap_or(self.media_library_id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "media_library_id", value);
        self.__load_state.mark_loaded("media_library_id").expect("generated field layout");
        self
    }

    pub fn changed_media_library_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "media_library_id")
    }

    pub fn eval_media_library_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("media_library_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "media_library_id".to_string(), attempted_path: "media_library_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.media_library_id())
                }}
    pub fn media_library(&self) -> Option<&crate::MediaLibrary> {
        if self.changed_media_library_id().is_some()
            || !self.is_loaded("media_library") {
            return None;
        }
        if let Some(edge) = self.__teaql_runtime_state().resolve_relation_option::<crate::MediaLibrary>(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME, self.id(), "media_library",
        ) {
            return edge.as_ref();
        }
        self.media_library.as_deref().or_else(|| {
            self.__teaql_runtime_state().resolve_entity(self.media_library_id())})
    }

    pub fn eval_media_library(&self) -> teaql_core::eval::EvalResult<&crate::MediaLibrary> {
        match self.media_library() {
            Some(v) => teaql_core::eval::EvalResult::Value(v),
            None => teaql_core::eval::EvalResult::NotLoaded { failed_node: "media_library".to_string(), attempted_path: "media_library".to_string() },
        }
    }

}

