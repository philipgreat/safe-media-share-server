
// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/app_setting
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
#[teaql(entity = "AppSetting", table = "app_setting_data", data_service = "sqlite", audit_mask_fields = "")]
pub struct AppSetting {
#[teaql(id)]
    id: u64,

// @source model.xml:96
#[teaql(max_length = 100)]
    setting_key: String,

// @source model.xml:96
#[teaql(max_length = 100)]
    setting_value: String,

// @source model.xml:96
#[teaql(max_length = 100)]
    setting_name: String,

// @source model.xml:96
#[teaql(max_length = 100)]
    setting_note: String,

// @source model.xml:96
    create_time: teaql_core::time::Timestamp,

// @source model.xml:96
    update_time: teaql_core::time::Timestamp,
#[teaql(version)]
    version: i64,
// @source model.xml:96
#[teaql(column = "media_library")]
    media_library_id: u64,
// @source model.xml:96
#[teaql(relation(target = "MediaLibrary", local_key = "media_library_id", foreign_key = "id"))]
    media_library: Option<Box<crate::MediaLibrary>>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl AppSetting {
    pub const ENTITY_NAME: &'static str = "App Setting";

    // Framework metadata: complete fixed layout, including overflow slots.
    pub const __TEAQL_FIELD_LAYOUT_REVISION: &'static str = "63975e24a345df7b8ecf4ebec9aa96a8ea067f96e4efdae63bb750e599d786bb";
    pub const __TEAQL_FIXED_FIELD_INDEXES: &'static [(&'static str, usize)] = &[
        ("id", 0),
        ("media_library", 1),
        ("setting_key", 2),
        ("setting_value", 3),
        ("setting_name", 4),
        ("setting_note", 5),
        ("create_time", 6),
        ("update_time", 7),
        ("version", 8),
    ];
    pub const __TEAQL_FIXED_FIELD_MAPPINGS: &'static [(&'static str, &'static str, &'static str)] = &[
        ("id", "id", "id"),
        ("media_library", "media_library_id", "media_library"),
        ("setting_key", "setting_key", "setting_key"),
        ("setting_value", "setting_value", "setting_value"),
        ("setting_name", "setting_name", "setting_name"),
        ("setting_note", "setting_note", "setting_note"),
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
            setting_key: String::new(),
            setting_value: String::new(),
            setting_name: String::new(),
            setting_note: String::new(),
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


    pub fn setting_key(&self) -> String {
        self.changed_setting_key().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.setting_key.clone())
    }

    pub fn update_setting_key(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.setting_key = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.setting_key.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "setting_key", value);
        self.__load_state.mark_loaded("setting_key").expect("generated field layout");
        self
    }

    pub fn changed_setting_key(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "setting_key")
    }

    pub fn eval_setting_key(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("setting_key") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "setting_key".to_string(), attempted_path: "setting_key".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.setting_key())
                }}


    pub fn setting_value(&self) -> String {
        self.changed_setting_value().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.setting_value.clone())
    }

    pub fn update_setting_value(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.setting_value = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.setting_value.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "setting_value", value);
        self.__load_state.mark_loaded("setting_value").expect("generated field layout");
        self
    }

    pub fn changed_setting_value(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "setting_value")
    }

    pub fn eval_setting_value(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("setting_value") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "setting_value".to_string(), attempted_path: "setting_value".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.setting_value())
                }}


    pub fn setting_name(&self) -> String {
        self.changed_setting_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.setting_name.clone())
    }

    pub fn update_setting_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.setting_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.setting_name.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "setting_name", value);
        self.__load_state.mark_loaded("setting_name").expect("generated field layout");
        self
    }

    pub fn changed_setting_name(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "setting_name")
    }

    pub fn eval_setting_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("setting_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "setting_name".to_string(), attempted_path: "setting_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.setting_name())
                }}


    pub fn setting_note(&self) -> String {
        self.changed_setting_note().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.setting_note.clone())
    }

    pub fn update_setting_note(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.setting_note = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.setting_note.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "setting_note", value);
        self.__load_state.mark_loaded("setting_note").expect("generated field layout");
        self
    }

    pub fn changed_setting_note(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "setting_note")
    }

    pub fn eval_setting_note(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("setting_note") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "setting_note".to_string(), attempted_path: "setting_note".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.setting_note())
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

