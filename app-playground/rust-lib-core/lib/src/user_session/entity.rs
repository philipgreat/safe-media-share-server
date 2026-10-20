
// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/user_session
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
#[teaql(entity = "UserSession", table = "user_session_data", data_service = "sqlite", audit_mask_fields = "")]
pub struct UserSession {
#[teaql(id)]
    id: u64,

// @source model.xml:129
#[teaql(max_length = 100)]
    session_fp: String,

// @source model.xml:129
    active: bool,

// @source model.xml:129
#[teaql(max_length = 100)]
    last_seen_at: String,

// @source model.xml:129
#[teaql(max_length = 100)]
    expire_at: String,

// @source model.xml:129
    create_time: teaql_core::time::Timestamp,

// @source model.xml:129
    update_time: teaql_core::time::Timestamp,
#[teaql(version)]
    version: i64,
// @source model.xml:129
#[teaql(column = "media_library")]
    media_library_id: u64,
// @source model.xml:129
#[teaql(relation(target = "MediaLibrary", local_key = "media_library_id", foreign_key = "id"))]
    media_library: Option<Box<crate::MediaLibrary>>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl UserSession {
    pub const ENTITY_NAME: &'static str = "User Session";

    // Framework metadata: complete fixed layout, including overflow slots.
    pub const __TEAQL_FIELD_LAYOUT_REVISION: &'static str = "48a7c6009d637df33af31836696c5601d6896853cf6c36cfdf4b2344044d99e8";
    pub const __TEAQL_FIXED_FIELD_INDEXES: &'static [(&'static str, usize)] = &[
        ("id", 0),
        ("media_library", 1),
        ("session_fp", 2),
        ("active", 3),
        ("last_seen_at", 4),
        ("expire_at", 5),
        ("create_time", 6),
        ("update_time", 7),
        ("version", 8),
    ];
    pub const __TEAQL_FIXED_FIELD_MAPPINGS: &'static [(&'static str, &'static str, &'static str)] = &[
        ("id", "id", "id"),
        ("media_library", "media_library_id", "media_library"),
        ("session_fp", "session_fp", "session_fp"),
        ("active", "active", "active"),
        ("last_seen_at", "last_seen_at", "last_seen_at"),
        ("expire_at", "expire_at", "expire_at"),
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
            session_fp: String::new(),
            active: false,
            last_seen_at: String::new(),
            expire_at: String::new(),
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


    pub fn session_fp(&self) -> String {
        self.changed_session_fp().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.session_fp.clone())
    }

    pub fn update_session_fp(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.session_fp = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.session_fp.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "session_fp", value);
        self.__load_state.mark_loaded("session_fp").expect("generated field layout");
        self
    }

    pub fn changed_session_fp(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "session_fp")
    }

    pub fn eval_session_fp(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("session_fp") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "session_fp".to_string(), attempted_path: "session_fp".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.session_fp())
                }}


    pub fn active(&self) -> bool {
        self.changed_active().and_then(|value| value.try_bool()).unwrap_or(self.active)
    }

    pub fn update_active(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.active = value.try_bool().unwrap_or(self.active.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "active", value);
        self.__load_state.mark_loaded("active").expect("generated field layout");
        self
    }

    pub fn changed_active(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "active")
    }

    pub fn eval_active(&self) -> teaql_core::eval::EvalResult<bool> {
        if !self.is_loaded("active") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "active".to_string(), attempted_path: "active".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.active())
                }}


    pub fn last_seen_at(&self) -> String {
        self.changed_last_seen_at().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.last_seen_at.clone())
    }

    pub fn update_last_seen_at(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.last_seen_at = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.last_seen_at.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "last_seen_at", value);
        self.__load_state.mark_loaded("last_seen_at").expect("generated field layout");
        self
    }

    pub fn changed_last_seen_at(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "last_seen_at")
    }

    pub fn eval_last_seen_at(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("last_seen_at") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "last_seen_at".to_string(), attempted_path: "last_seen_at".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.last_seen_at())
                }}


    pub fn expire_at(&self) -> String {
        self.changed_expire_at().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.expire_at.clone())
    }

    pub fn update_expire_at(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.expire_at = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.expire_at.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "expire_at", value);
        self.__load_state.mark_loaded("expire_at").expect("generated field layout");
        self
    }

    pub fn changed_expire_at(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "expire_at")
    }

    pub fn eval_expire_at(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("expire_at") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "expire_at".to_string(), attempted_path: "expire_at".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.expire_at())
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

