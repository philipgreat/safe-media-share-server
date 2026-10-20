
// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/ip_access_grant
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
#[teaql(entity = "IpAccessGrant", table = "ip_access_grant_data", data_service = "sqlite", audit_mask_fields = "")]
pub struct IpAccessGrant {
#[teaql(id)]
    id: u64,

// @source model.xml:54
#[teaql(max_length = 100)]
    client_ip: String,

// @source model.xml:54
#[teaql(max_length = 100)]
    expire_at: String,

// @source model.xml:54
#[teaql(max_length = 100)]
    grant_note: String,

// @source model.xml:54
    create_time: teaql_core::time::Timestamp,

// @source model.xml:54
    update_time: teaql_core::time::Timestamp,
#[teaql(version)]
    version: i64,
// @source model.xml:54
#[teaql(column = "media_library")]
    media_library_id: u64,

// @source model.xml:54
#[teaql(column = "media_asset")]
    media_asset_id: u64,

// @source model.xml:54
#[teaql(column = "grant_status")]
    grant_status_id: u64,
// @source model.xml:54
#[teaql(relation(target = "MediaLibrary", local_key = "media_library_id", foreign_key = "id"))]
    media_library: Option<Box<crate::MediaLibrary>>,

// @source model.xml:54
#[teaql(relation(target = "MediaAsset", local_key = "media_asset_id", foreign_key = "id"))]
    media_asset: Option<Box<crate::MediaAsset>>,

// @source model.xml:54
#[teaql(relation(target = "GrantStatus", local_key = "grant_status_id", foreign_key = "id"))]
    grant_status: Option<Box<crate::GrantStatus>>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl IpAccessGrant {
    pub const ENTITY_NAME: &'static str = "IP Access Grant";

    // Framework metadata: complete fixed layout, including overflow slots.
    pub const __TEAQL_FIELD_LAYOUT_REVISION: &'static str = "1e85ff087cde634da32c787848230ed61a8af91196745c8e090aea391a61802a";
    pub const __TEAQL_FIXED_FIELD_INDEXES: &'static [(&'static str, usize)] = &[
        ("id", 0),
        ("media_library", 1),
        ("media_asset", 2),
        ("grant_status", 3),
        ("client_ip", 4),
        ("expire_at", 5),
        ("grant_note", 6),
        ("create_time", 7),
        ("update_time", 8),
        ("version", 9),
    ];
    pub const __TEAQL_FIXED_FIELD_MAPPINGS: &'static [(&'static str, &'static str, &'static str)] = &[
        ("id", "id", "id"),
        ("media_library", "media_library_id", "media_library"),
        ("media_asset", "media_asset_id", "media_asset"),
        ("grant_status", "grant_status_id", "grant_status"),
        ("client_ip", "client_ip", "client_ip"),
        ("expire_at", "expire_at", "expire_at"),
        ("grant_note", "grant_note", "grant_note"),
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
            client_ip: String::new(),
            expire_at: String::new(),
            grant_note: String::new(),
            create_time: teaql_core::time::Timestamp::now(),
            update_time: teaql_core::time::Timestamp::now(),
            version: 0_i64,
            media_library_id: 0_u64,
            media_asset_id: 0_u64,
            grant_status_id: 0_u64,
            media_library: None,
            media_asset: None,
            grant_status: None,
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
        if let Some(entity) = &mut self.media_asset {
            entity.attach_runtime_state_recursive(root.clone());
        }
        if let Some(entity) = &mut self.grant_status {
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


    pub fn client_ip(&self) -> String {
        self.changed_client_ip().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.client_ip.clone())
    }

    pub fn update_client_ip(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.client_ip = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.client_ip.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "client_ip", value);
        self.__load_state.mark_loaded("client_ip").expect("generated field layout");
        self
    }

    pub fn changed_client_ip(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "client_ip")
    }

    pub fn eval_client_ip(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("client_ip") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "client_ip".to_string(), attempted_path: "client_ip".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.client_ip())
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


    pub fn grant_note(&self) -> String {
        self.changed_grant_note().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.grant_note.clone())
    }

    pub fn update_grant_note(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.grant_note = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.grant_note.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "grant_note", value);
        self.__load_state.mark_loaded("grant_note").expect("generated field layout");
        self
    }

    pub fn changed_grant_note(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "grant_note")
    }

    pub fn eval_grant_note(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("grant_note") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "grant_note".to_string(), attempted_path: "grant_note".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.grant_note())
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

    pub fn media_asset_id(&self) -> u64 {
        self.changed_media_asset_id().and_then(|value| value.try_u64()).unwrap_or(self.media_asset_id)
    }

    pub fn update_media_asset_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.media_asset_id = value.try_u64().unwrap_or(self.media_asset_id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "media_asset_id", value);
        self.__load_state.mark_loaded("media_asset_id").expect("generated field layout");
        self
    }

    pub fn changed_media_asset_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "media_asset_id")
    }

    pub fn eval_media_asset_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("media_asset_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "media_asset_id".to_string(), attempted_path: "media_asset_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.media_asset_id())
                }}

    pub fn grant_status_id(&self) -> u64 {
        self.changed_grant_status_id().and_then(|value| value.try_u64()).unwrap_or(self.grant_status_id)
    }

    pub(crate) fn update_grant_status_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.grant_status_id = value.try_u64().unwrap_or(self.grant_status_id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "grant_status_id", value);
        self.__load_state.mark_loaded("grant_status_id").expect("generated field layout");
        self
    }

    pub fn changed_grant_status_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "grant_status_id")
    }

    pub fn eval_grant_status_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("grant_status_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "grant_status_id".to_string(), attempted_path: "grant_status_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.grant_status_id())
                }}
    pub fn update_grant_status_to_active(&mut self) -> &mut Self {
        self.update_grant_status_id(1001_u64)
    }

    pub fn grant_status_is_active(&self) -> bool {
        self.grant_status_id() == 1001_u64
    }
    pub fn update_grant_status_to_expired(&mut self) -> &mut Self {
        self.update_grant_status_id(1002_u64)
    }

    pub fn grant_status_is_expired(&self) -> bool {
        self.grant_status_id() == 1002_u64
    }
    pub fn update_grant_status_to_revoked(&mut self) -> &mut Self {
        self.update_grant_status_id(1003_u64)
    }

    pub fn grant_status_is_revoked(&self) -> bool {
        self.grant_status_id() == 1003_u64
    }
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

    pub fn media_asset(&self) -> Option<&crate::MediaAsset> {
        if self.changed_media_asset_id().is_some()
            || !self.is_loaded("media_asset") {
            return None;
        }
        if let Some(edge) = self.__teaql_runtime_state().resolve_relation_option::<crate::MediaAsset>(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME, self.id(), "media_asset",
        ) {
            return edge.as_ref();
        }
        self.media_asset.as_deref().or_else(|| {
            self.__teaql_runtime_state().resolve_entity(self.media_asset_id())})
    }

    pub fn eval_media_asset(&self) -> teaql_core::eval::EvalResult<&crate::MediaAsset> {
        match self.media_asset() {
            Some(v) => teaql_core::eval::EvalResult::Value(v),
            None => teaql_core::eval::EvalResult::NotLoaded { failed_node: "media_asset".to_string(), attempted_path: "media_asset".to_string() },
        }
    }

    pub fn grant_status(&self) -> Option<&crate::GrantStatus> {
        if self.changed_grant_status_id().is_some()
            || !self.is_loaded("grant_status") {
            return None;
        }
        if let Some(edge) = self.__teaql_runtime_state().resolve_relation_option::<crate::GrantStatus>(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME, self.id(), "grant_status",
        ) {
            return edge.as_ref();
        }
        self.grant_status.as_deref().or_else(|| {
            self.__teaql_runtime_state().resolve_entity(self.grant_status_id())})
    }

    pub fn eval_grant_status(&self) -> teaql_core::eval::EvalResult<&crate::GrantStatus> {
        match self.grant_status() {
            Some(v) => teaql_core::eval::EvalResult::Value(v),
            None => teaql_core::eval::EvalResult::NotLoaded { failed_node: "grant_status".to_string(), attempted_path: "grant_status".to_string() },
        }
    }

}

