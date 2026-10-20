
// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/media_library
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
#[teaql(reverse_relation(name = "media_asset_list", target = "MediaAsset", json_type = "crate::MediaAsset", local_key = "id", foreign_key = "media_library_id", many))]
#[teaql(reverse_relation(name = "grant_status_list", target = "GrantStatus", json_type = "crate::GrantStatus", local_key = "id", foreign_key = "media_library_id", many))]
#[teaql(reverse_relation(name = "ip_access_grant_list", target = "IpAccessGrant", json_type = "crate::IpAccessGrant", local_key = "id", foreign_key = "media_library_id", many))]
#[teaql(reverse_relation(name = "invite_status_list", target = "InviteStatus", json_type = "crate::InviteStatus", local_key = "id", foreign_key = "media_library_id", many))]
#[teaql(reverse_relation(name = "invite_code_list", target = "InviteCode", json_type = "crate::InviteCode", local_key = "id", foreign_key = "media_library_id", many))]
#[teaql(reverse_relation(name = "app_setting_list", target = "AppSetting", json_type = "crate::AppSetting", local_key = "id", foreign_key = "media_library_id", many))]
#[teaql(reverse_relation(name = "proxy_trust_rule_list", target = "ProxyTrustRule", json_type = "crate::ProxyTrustRule", local_key = "id", foreign_key = "media_library_id", many))]
#[teaql(reverse_relation(name = "share_handle_list", target = "ShareHandle", json_type = "crate::ShareHandle", local_key = "id", foreign_key = "media_library_id", many))]
#[teaql(entity = "MediaLibrary", table = "media_library_data", data_service = "sqlite", audit_mask_fields = "")]
pub struct MediaLibrary {
#[teaql(id)]
    id: u64,

// @source model.xml:14
#[teaql(max_length = 100)]
    library_name: String,

// @source model.xml:14
#[teaql(max_length = 100)]
    root_path: String,

// @source model.xml:14
    create_time: teaql_core::time::Timestamp,

// @source model.xml:14
    update_time: teaql_core::time::Timestamp,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl MediaLibrary {
    pub const ENTITY_NAME: &'static str = "Media Library";

    // Framework metadata: complete fixed layout, including overflow slots.
    pub const __TEAQL_FIELD_LAYOUT_REVISION: &'static str = "f70f90dc8fc329b94c45296dcfd12d3230636a8aa8ee2a9516a309acdec8d260";
    pub const __TEAQL_FIXED_FIELD_INDEXES: &'static [(&'static str, usize)] = &[
        ("id", 0),
        ("library_name", 1),
        ("root_path", 2),
        ("create_time", 3),
        ("update_time", 4),
        ("version", 5),
    ];
    pub const __TEAQL_FIXED_FIELD_MAPPINGS: &'static [(&'static str, &'static str, &'static str)] = &[
        ("id", "id", "id"),
        ("library_name", "library_name", "library_name"),
        ("root_path", "root_path", "root_path"),
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
            library_name: String::new(),
            root_path: String::new(),
            create_time: teaql_core::time::Timestamp::now(),
            update_time: teaql_core::time::Timestamp::now(),
            version: 0_i64,
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


    pub fn library_name(&self) -> String {
        self.changed_library_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.library_name.clone())
    }

    pub fn update_library_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.library_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.library_name.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "library_name", value);
        self.__load_state.mark_loaded("library_name").expect("generated field layout");
        self
    }

    pub fn changed_library_name(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "library_name")
    }

    pub fn eval_library_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("library_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "library_name".to_string(), attempted_path: "library_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.library_name())
                }}


    pub fn root_path(&self) -> String {
        self.changed_root_path().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.root_path.clone())
    }

    pub fn update_root_path(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.root_path = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.root_path.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "root_path", value);
        self.__load_state.mark_loaded("root_path").expect("generated field layout");
        self
    }

    pub fn changed_root_path(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "root_path")
    }

    pub fn eval_root_path(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("root_path") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "root_path".to_string(), attempted_path: "root_path".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.root_path())
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

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn media_asset_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::MediaAsset>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "media_asset_list",
        )
    }

    pub fn eval_media_asset_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::MediaAsset>> {
        let relation = self.media_asset_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "media_asset_list".to_string(), attempted_path: "media_asset_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn grant_status_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::GrantStatus>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "grant_status_list",
        )
    }

    pub fn eval_grant_status_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::GrantStatus>> {
        let relation = self.grant_status_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "grant_status_list".to_string(), attempted_path: "grant_status_list".to_string() },
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
    pub fn invite_status_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::InviteStatus>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "invite_status_list",
        )
    }

    pub fn eval_invite_status_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::InviteStatus>> {
        let relation = self.invite_status_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "invite_status_list".to_string(), attempted_path: "invite_status_list".to_string() },
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
    pub fn app_setting_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::AppSetting>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "app_setting_list",
        )
    }

    pub fn eval_app_setting_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::AppSetting>> {
        let relation = self.app_setting_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "app_setting_list".to_string(), attempted_path: "app_setting_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn proxy_trust_rule_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::ProxyTrustRule>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "proxy_trust_rule_list",
        )
    }

    pub fn eval_proxy_trust_rule_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::ProxyTrustRule>> {
        let relation = self.proxy_trust_rule_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "proxy_trust_rule_list".to_string(), attempted_path: "proxy_trust_rule_list".to_string() },
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

