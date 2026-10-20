
// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/media_asset
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
#[teaql(reverse_relation(name = "ip_access_grant_list", target = "IpAccessGrant", json_type = "crate::IpAccessGrant", local_key = "id", foreign_key = "media_asset_id", many))]
#[teaql(reverse_relation(name = "invite_code_list", target = "InviteCode", json_type = "crate::InviteCode", local_key = "id", foreign_key = "media_asset_id", many))]
#[teaql(reverse_relation(name = "share_handle_list", target = "ShareHandle", json_type = "crate::ShareHandle", local_key = "id", foreign_key = "media_asset_id", many))]
#[teaql(entity = "MediaAsset", table = "media_asset_data", data_service = "sqlite", audit_mask_fields = "")]
pub struct MediaAsset {
#[teaql(id)]
    id: u64,

// @source model.xml:27
#[teaql(max_length = 100)]
    display_name: String,

// @source model.xml:27
#[teaql(max_length = 100)]
    file_path: String,

// @source model.xml:27
#[teaql(max_length = 100)]
    content_type: String,

// @source model.xml:27
    file_size: i64,

// @source model.xml:27
    duration_seconds: i64,

// @source model.xml:27
    active: bool,

// @source model.xml:27
    create_time: teaql_core::time::Timestamp,

// @source model.xml:27
    update_time: teaql_core::time::Timestamp,
#[teaql(version)]
    version: i64,
// @source model.xml:27
#[teaql(column = "media_library")]
    media_library_id: u64,
// @source model.xml:27
#[teaql(relation(target = "MediaLibrary", local_key = "media_library_id", foreign_key = "id"))]
    media_library: Option<Box<crate::MediaLibrary>>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl MediaAsset {
    pub const ENTITY_NAME: &'static str = "Media Asset";

    // Framework metadata: complete fixed layout, including overflow slots.
    pub const __TEAQL_FIELD_LAYOUT_REVISION: &'static str = "4d29fcb627ee4ae2b57ace2c874d5aac34eca9f44d4d53cf283de37042a1af4d";
    pub const __TEAQL_FIXED_FIELD_INDEXES: &'static [(&'static str, usize)] = &[
        ("id", 0),
        ("media_library", 1),
        ("display_name", 2),
        ("file_path", 3),
        ("content_type", 4),
        ("file_size", 5),
        ("duration_seconds", 6),
        ("active", 7),
        ("create_time", 8),
        ("update_time", 9),
        ("version", 10),
    ];
    pub const __TEAQL_FIXED_FIELD_MAPPINGS: &'static [(&'static str, &'static str, &'static str)] = &[
        ("id", "id", "id"),
        ("media_library", "media_library_id", "media_library"),
        ("display_name", "display_name", "display_name"),
        ("file_path", "file_path", "file_path"),
        ("content_type", "content_type", "content_type"),
        ("file_size", "file_size", "file_size"),
        ("duration_seconds", "duration_seconds", "duration_seconds"),
        ("active", "active", "active"),
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
            display_name: String::new(),
            file_path: String::new(),
            content_type: String::new(),
            file_size: 0_i64,
            duration_seconds: 0_i64,
            active: false,
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


    pub fn display_name(&self) -> String {
        self.changed_display_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.display_name.clone())
    }

    pub fn update_display_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.display_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.display_name.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "display_name", value);
        self.__load_state.mark_loaded("display_name").expect("generated field layout");
        self
    }

    pub fn changed_display_name(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "display_name")
    }

    pub fn eval_display_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("display_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "display_name".to_string(), attempted_path: "display_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.display_name())
                }}


    pub fn file_path(&self) -> String {
        self.changed_file_path().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.file_path.clone())
    }

    pub fn update_file_path(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.file_path = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.file_path.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "file_path", value);
        self.__load_state.mark_loaded("file_path").expect("generated field layout");
        self
    }

    pub fn changed_file_path(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "file_path")
    }

    pub fn eval_file_path(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("file_path") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "file_path".to_string(), attempted_path: "file_path".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.file_path())
                }}


    pub fn content_type(&self) -> String {
        self.changed_content_type().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.content_type.clone())
    }

    pub fn update_content_type(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.content_type = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.content_type.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "content_type", value);
        self.__load_state.mark_loaded("content_type").expect("generated field layout");
        self
    }

    pub fn changed_content_type(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "content_type")
    }

    pub fn eval_content_type(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("content_type") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "content_type".to_string(), attempted_path: "content_type".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.content_type())
                }}


    pub fn file_size(&self) -> i64 {
        self.changed_file_size().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.file_size)
    }

    pub fn update_file_size(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.file_size = value.try_i64().map(|value| value as i64).unwrap_or(self.file_size.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "file_size", value);
        self.__load_state.mark_loaded("file_size").expect("generated field layout");
        self
    }

    pub fn changed_file_size(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "file_size")
    }

    pub fn eval_file_size(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("file_size") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "file_size".to_string(), attempted_path: "file_size".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.file_size())
                }}


    pub fn duration_seconds(&self) -> i64 {
        self.changed_duration_seconds().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.duration_seconds)
    }

    pub fn update_duration_seconds(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.duration_seconds = value.try_i64().map(|value| value as i64).unwrap_or(self.duration_seconds.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "duration_seconds", value);
        self.__load_state.mark_loaded("duration_seconds").expect("generated field layout");
        self
    }

    pub fn changed_duration_seconds(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "duration_seconds")
    }

    pub fn eval_duration_seconds(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("duration_seconds") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "duration_seconds".to_string(), attempted_path: "duration_seconds".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.duration_seconds())
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
    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn ip_access_grant_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::IpAccessGrant>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "ip_access_grant_list",
        )
    }

    pub fn eval_ip_access_grant_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::IpAccessGrant>> {
        let relation = self.ip_access_grant_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "ip_access_grant_list".to_string(), attempted_path: "ip_access_grant_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn invite_code_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::InviteCode>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "invite_code_list",
        )
    }

    pub fn eval_invite_code_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::InviteCode>> {
        let relation = self.invite_code_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "invite_code_list".to_string(), attempted_path: "invite_code_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn share_handle_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::ShareHandle>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "share_handle_list",
        )
    }

    pub fn eval_share_handle_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::ShareHandle>> {
        let relation = self.share_handle_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "share_handle_list".to_string(), attempted_path: "share_handle_list".to_string() },
        }
    }

}

