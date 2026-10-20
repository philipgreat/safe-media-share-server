
use crate::*;
use teaql_core::TeaqlEntity;

use teaql_provider_sqlite::SqliteProviderExt as _;

pub type DataServiceDialect = teaql_provider_sqlite::SqliteDialect;
pub type DataServiceMutationExecutor = teaql_provider_sqlite::SqliteMutationExecutor;
pub type DataServiceMutationError = teaql_provider_sqlite::MutationExecutorError;
pub type DataServiceIdGenerator = teaql_provider_sqlite::SqliteIdSpaceGenerator;
pub type DataServicePool = std::sync::Arc<std::sync::Mutex<rusqlite::Connection>>;
pub type DataServiceExecutor = ServiceRuntimeExecutor;
pub type ServiceRuntime = teaql_runtime::UserContext;

pub const DATABASE_URL_ENV: &str = "MEDIA_GUARD_SERVICE_CORE_DATABASE_URL";
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceRuntimeConfig {
    pub database_url: String,
}

impl ServiceRuntimeConfig {
    pub fn from_env() -> Result<Self, ServiceRuntimeError> {
        Ok(Self {
            database_url: env_value(DATABASE_URL_ENV)?,
        })
    }
}

#[derive(Debug)]
pub enum ServiceRuntimeError {
    MissingEnv {
        name: &'static str,
        source: std::env::VarError,
    },
    ConnectionError(String),
    Configuration(String),
    Rusqlite(rusqlite::Error),
    Runtime(teaql_runtime::RuntimeError),
}

impl std::fmt::Display for ServiceRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceRuntimeError::MissingEnv { name, source } => {
                write!(f, "missing environment variable {name}: {source}")
            }
            ServiceRuntimeError::ConnectionError(err) => write!(f, "connection error: {err}"),
            ServiceRuntimeError::Configuration(err) => write!(f, "configuration error: {err}"),
            ServiceRuntimeError::Rusqlite(err) => write!(f, "rusqlite error: {err}"),
            ServiceRuntimeError::Runtime(err) => write!(f, "runtime error: {err}"),
        }
    }
}

impl std::error::Error for ServiceRuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceRuntimeError::MissingEnv { source, .. } => Some(source),
            ServiceRuntimeError::ConnectionError(_) => None,
            ServiceRuntimeError::Configuration(_) => None,
            ServiceRuntimeError::Rusqlite(err) => Some(err),
            ServiceRuntimeError::Runtime(err) => Some(err),
        }
    }
}

impl From<rusqlite::Error> for ServiceRuntimeError {
    fn from(err: rusqlite::Error) -> Self {
        ServiceRuntimeError::Rusqlite(err)
    }
}
impl From<teaql_runtime::RuntimeError> for ServiceRuntimeError {
    fn from(err: teaql_runtime::RuntimeError) -> Self {
        ServiceRuntimeError::Runtime(err)
    }
}

#[derive(Clone)]
pub struct LocalSchemaProvider;

impl teaql_data_service::SchemaProvider for LocalSchemaProvider {
    fn get_entity(&self, name: &str) -> Option<std::sync::Arc<teaql_core::EntityDescriptor>> {
        match name {
            "MediaLibrary" => Some(std::sync::Arc::new(crate::MediaLibrary::entity_descriptor())),
            "MediaAsset" => Some(std::sync::Arc::new(crate::MediaAsset::entity_descriptor())),
            "GrantStatus" => Some(std::sync::Arc::new(crate::GrantStatus::entity_descriptor())),
            "IpAccessGrant" => Some(std::sync::Arc::new(crate::IpAccessGrant::entity_descriptor())),
            "InviteStatus" => Some(std::sync::Arc::new(crate::InviteStatus::entity_descriptor())),
            "InviteCode" => Some(std::sync::Arc::new(crate::InviteCode::entity_descriptor())),
            "AppSetting" => Some(std::sync::Arc::new(crate::AppSetting::entity_descriptor())),
            "ProxyTrustRule" => Some(std::sync::Arc::new(crate::ProxyTrustRule::entity_descriptor())),
            "ShareHandle" => Some(std::sync::Arc::new(crate::ShareHandle::entity_descriptor())),
            "UserSession" => Some(std::sync::Arc::new(crate::UserSession::entity_descriptor())),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct ServiceRuntimeExecutor {
    inner: teaql_sql::SqlDataServiceExecutor<
        DataServiceDialect,
        DataServiceMutationExecutor,
        LocalSchemaProvider
    >,
    query_metadata_observer: Option<std::sync::Arc<dyn Fn(&teaql_data_service::ExecutionMetadata) + Send + Sync>>,
}

impl ServiceRuntimeExecutor {
    pub fn new(inner: DataServiceMutationExecutor) -> Self {
        Self {
            inner: teaql_sql::SqlDataServiceExecutor::new(
                DataServiceDialect::default(),
                inner,
                LocalSchemaProvider
            ),
            query_metadata_observer: None,
        }
    }

    /// Trusted runtime diagnostic SPI; it never changes query results or intent.
    #[doc(hidden)]
    pub fn with_query_metadata_observer(mut self,
        observer: std::sync::Arc<dyn Fn(&teaql_data_service::ExecutionMetadata) + Send + Sync>,
    ) -> Self {
        self.query_metadata_observer = Some(observer);
        self
    }

    fn notify_query_metadata(&self, metadata: &teaql_data_service::ExecutionMetadata) {
        if let Some(observer) = &self.query_metadata_observer {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| observer(metadata)));
        }
    }

    fn query_metadata_observer<'a>(
        &self, upstream: Option<teaql_data_service::ExecutionObserver<'a>>,
    ) -> Option<teaql_data_service::ExecutionObserver<'a>> {
        match &self.query_metadata_observer {
            Some(callback) => {
                let callback = callback.clone();
                Some(std::sync::Arc::new(move |metadata| {
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(&metadata)));
                    if let Some(upstream) = &upstream { upstream(metadata); }
                }))
            }
            None => upstream,
        }
    }

}

impl teaql_data_service::DataServiceExecutor for ServiceRuntimeExecutor {
    type Error = teaql_sql::SqlExecutorError<DataServiceMutationError>;
    fn capabilities(&self) -> teaql_data_service::DataServiceCapabilities {
        teaql_data_service::DataServiceExecutor::capabilities(&self.inner)
    }
}

impl teaql_data_service::QueryExecutor for ServiceRuntimeExecutor {
    fn query_log_intent(&self, query: &teaql_core::SelectQuery) -> teaql_data_service::SqlIntentRedactions {
        teaql_data_service::QueryExecutor::query_log_intent(&self.inner, query)
    }

    fn dynamic_field_store(&self) -> Option<&dyn teaql_data_service::dynamic_fields::DynamicFieldStore> {
        teaql_data_service::QueryExecutor::dynamic_field_store(&self.inner)
    }

    async fn query(&self, request: teaql_data_service::QueryRequest) -> Result<teaql_data_service::QueryResult, Self::Error> {
        teaql_data_service::QueryExecutor::query_observed(self, request, None).await
    }

    fn query_observed<'a>(
        &'a self,
        request: teaql_data_service::QueryRequest,
        observer: Option<teaql_data_service::ExecutionObserver<'a>>,
    ) -> impl std::future::Future<Output = Result<teaql_data_service::QueryResult, Self::Error>> + Send + 'a {
        async move {
        let observer = self.query_metadata_observer(observer);
        let result = teaql_data_service::QueryExecutor::query_observed(&self.inner, request, observer).await?;
        self.notify_query_metadata(&result.metadata);
        Ok(result)
        }
    }
}

impl teaql_data_service::StreamQueryExecutor for ServiceRuntimeExecutor {
    fn query_stream_with_dynamic_fields<'a>(
        &'a self,
        request: teaql_data_service::QueryRequest,
        chunk_size: usize,
        plan: teaql_data_service::dynamic_fields::DynamicFieldStreamPlan,
        observer: Option<teaql_data_service::ExecutionObserver<'a>>,
    ) -> Result<teaql_data_service::QueryStream<'a, Self::Error>, teaql_core::dynamic_fields::DynamicFieldError> {
        let observer = self.query_metadata_observer(observer);
        teaql_data_service::StreamQueryExecutor::query_stream_with_dynamic_fields(
            &self.inner, request, chunk_size, plan, observer,
        )
    }

    fn query_stream_observed<'a>(
        &'a self,
        request: teaql_data_service::QueryRequest,
        chunk_size: usize,
        observer: teaql_data_service::ExecutionObserver<'a>,
    ) -> teaql_data_service::QueryStream<'a, Self::Error> {
        let observer = self.query_metadata_observer(Some(observer)).expect("caller supplied stream observer");
        teaql_data_service::StreamQueryExecutor::query_stream_observed(&self.inner, request, chunk_size, observer)
    }

    fn query_stream(&self, request: teaql_data_service::QueryRequest, chunk_size: usize) -> teaql_data_service::QueryStream<'_, Self::Error> {
        if let Some(observer) = self.query_metadata_observer(None) {
            teaql_data_service::StreamQueryExecutor::query_stream_observed(&self.inner, request, chunk_size, observer)
        } else {
            teaql_data_service::StreamQueryExecutor::query_stream(&self.inner, request, chunk_size)
        }
    }
}

impl teaql_data_service::MutationExecutor for ServiceRuntimeExecutor {
    async fn mutate(&self, request: teaql_data_service::MutationRequest) -> Result<teaql_data_service::MutationResult, Self::Error> {
        teaql_data_service::MutationExecutor::mutate(&self.inner, request).await
    }
}

impl teaql_data_service::TransactionExecutor for ServiceRuntimeExecutor {
    type Tx<'a> = teaql_sql::SqlDataServiceTransaction<'a, DataServiceDialect, <DataServiceMutationExecutor as teaql_sql::SqlTransactionTransport>::Tx<'a>, LocalSchemaProvider> where Self: 'a;

    async fn begin(&self) -> Result<Self::Tx<'_ >, Self::Error> {
        teaql_data_service::TransactionExecutor::begin(&self.inner).await
    }
}

pub async fn service_runtime_from_env() -> Result<ServiceRuntime, ServiceRuntimeError> {
    service_runtime(ServiceRuntimeConfig::from_env()?).await
}

pub async fn service_runtime(config: ServiceRuntimeConfig) -> Result<ServiceRuntime, ServiceRuntimeError> {
    let pool = connect_data_service_pool(&config).await?;
    service_runtime_from_pool(pool).await
}

pub async fn service_runtime_from_pool(pool: DataServicePool) -> Result<ServiceRuntime, ServiceRuntimeError> {
    let mutation_executor = DataServiceMutationExecutor::new(pool);
    let id_generator = DataServiceIdGenerator::from_executor(mutation_executor.clone());let mut context = module_with_behaviors_and_checkers().into_context();
    context.set_internal_id_generator(id_generator);
    context.use_sqlite_provider(mutation_executor.clone());
    let executor = ServiceRuntimeExecutor::new(mutation_executor);
    context.register_executor(executor.clone());
    context.insert_resource(executor);

    // SQL logging is owned by teaql-runtime. The legacy teaql-tool-core
    // environment whitelist rejects runtime logging variables and must not
    // gate generated application startup. Schema remains explicit.

    Ok(context)
}



fn env_value(name: &'static str) -> Result<String, ServiceRuntimeError> {
    std::env::var(name).map_err(|source| ServiceRuntimeError::MissingEnv { name, source })
}

async fn connect_data_service_pool(config: &ServiceRuntimeConfig) -> Result<DataServicePool, ServiceRuntimeError> {
    let url = &config.database_url;
    let sanitized_url = if url.starts_with("sqlite:") { url.strip_prefix("sqlite:").unwrap().trim_start_matches("//") } else { url };
    let pure_file_path = sanitized_url.split('?').next().unwrap_or(sanitized_url);
    let path = std::path::Path::new(pure_file_path);
    if let Some(parent) = path.parent() { if !parent.as_os_str().is_empty() { std::fs::create_dir_all(parent).map_err(|e| ServiceRuntimeError::ConnectionError(e.to_string()))?; } }
    Ok(std::sync::Arc::new(std::sync::Mutex::new(rusqlite::Connection::open(pure_file_path).map_err(|e| ServiceRuntimeError::ConnectionError(e.to_string()))?)))
}

pub fn repository_registry() -> teaql_runtime::InMemoryEntityRegistry {
    teaql_runtime::InMemoryEntityRegistry::new()
        .with_entity("MediaLibrary")
        .with_entity("MediaAsset")
        .with_entity("GrantStatus")
        .with_entity("IpAccessGrant")
        .with_entity("InviteStatus")
        .with_entity("InviteCode")
        .with_entity("AppSetting")
        .with_entity("ProxyTrustRule")
        .with_entity("ShareHandle")
        .with_entity("UserSession")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry {
    teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry::new()
        .with_behavior("MediaLibrary", MediaLibraryBehavior::default())
        .with_behavior("MediaAsset", MediaAssetBehavior::default())
        .with_behavior("GrantStatus", GrantStatusBehavior::default())
        .with_behavior("IpAccessGrant", IpAccessGrantBehavior::default())
        .with_behavior("InviteStatus", InviteStatusBehavior::default())
        .with_behavior("InviteCode", InviteCodeBehavior::default())
        .with_behavior("AppSetting", AppSettingBehavior::default())
        .with_behavior("ProxyTrustRule", ProxyTrustRuleBehavior::default())
        .with_behavior("ShareHandle", ShareHandleBehavior::default())
        .with_behavior("UserSession", UserSessionBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<MediaLibrary, _>::new(MediaLibraryChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MediaAsset, _>::new(MediaAssetChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<GrantStatus, _>::new(GrantStatusChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<IpAccessGrant, _>::new(IpAccessGrantChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<InviteStatus, _>::new(InviteStatusChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<InviteCode, _>::new(InviteCodeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<AppSetting, _>::new(AppSettingChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ProxyTrustRule, _>::new(ProxyTrustRuleChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ShareHandle, _>::new(ShareHandleChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<UserSession, _>::new(UserSessionChecker::default()))
}

fn is_generated_bootstrap_retryable_conflict(error: &teaql_runtime::RuntimeError) -> bool {
    if matches!(error, teaql_runtime::RuntimeError::OptimisticLockConflict { .. }) { return true }
    let teaql_runtime::RuntimeError::Graph(message) = error else { return false };
    (message.contains("UNIQUE constraint failed:") && message.contains("_data.id"))
        || (message.contains("duplicate key value violates unique constraint") && message.contains("_data_pkey"))
        || (message.contains("Duplicate entry") && message.contains("PRIMARY"))
}

fn ensure_generated_bootstrap<'a>(context: &'a teaql_runtime::UserContext) -> teaql_runtime::GeneratedSchemaBootstrapFuture<'a> {
    Box::pin(async move {
        for attempt in 0..5 {
            match ensure_generated_bootstrap_once(context).await {
                Ok(()) => return Ok(()),
                Err(error) if attempt < 4 && is_generated_bootstrap_retryable_conflict(&error) => tokio::task::yield_now().await,
                Err(error) => return Err(error),
            }
        }
        unreachable!("bootstrap attempts always return or fail")
    })
}

async fn ensure_generated_bootstrap_once(context: &teaql_runtime::UserContext) -> Result<(), teaql_runtime::RuntimeError> {
        use teaql_core::Entity as _;
        let root_rows = crate::Q::media_libraries().select_self_fields().with_id_is(1_u64).comment("what: locate generated Domain Root").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        let domain_root = if let Some(entity) = root_rows.data.into_iter().next() { entity } else {
            let mut entity = MediaLibrary::runtime_new(context.entity_runtime_state());
            entity.update_id(1_u64);
            context.initialize_generated_bootstrap_entity(&mut entity, MediaLibrary::ENTITY_NAME, 1_u64)?;
            entity.update_library_name("Home Video Library");
            entity.update_root_path("/srv/media");
            teaql_runtime::AuditedSaveExt::save(entity.audit_as("create generated Domain Root MediaLibrary"), context).await?
        };
        context.set_generated_bootstrap_active_root(MediaLibrary::ENTITY_NAME, domain_root.id())?;
        let rows_constant_grant_status_1001 = crate::Q::grant_statuses().select_self_fields().with_id_is(1001_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_grant_status_1001) = rows_constant_grant_status_1001.data.into_iter().next() {
            let mut changed = false;
            if constant_grant_status_1001.media_library_id() != 1_u64 { constant_grant_status_1001.update_media_library_id(1_u64); changed = true; }
            if constant_grant_status_1001.name() != "Active" { constant_grant_status_1001.update_name("Active"); changed = true; }
            if constant_grant_status_1001.code() != "ACTIVE" { constant_grant_status_1001.update_code("ACTIVE"); changed = true; }
            if constant_grant_status_1001.display_order() != rust_decimal::Decimal::from_str_exact("1").unwrap() { constant_grant_status_1001.update_display_order(rust_decimal::Decimal::from_str_exact("1").unwrap()); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_grant_status_1001.audit_as("reconcile model constant GrantStatus(1001)"), context).await?; }
        } else {
            let mut constant_grant_status_1001 = GrantStatus::runtime_new(context.entity_runtime_state());
            constant_grant_status_1001.update_id(1001_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_grant_status_1001, GrantStatus::ENTITY_NAME, 1001_u64)?;
            constant_grant_status_1001.update_media_library_id(1_u64);
            constant_grant_status_1001.update_name("Active");
            constant_grant_status_1001.update_code("ACTIVE");
            constant_grant_status_1001.update_display_order(rust_decimal::Decimal::from_str_exact("1").unwrap());
            let _ = teaql_runtime::AuditedSaveExt::save(constant_grant_status_1001.audit_as("create model constant GrantStatus(1001)"), context).await?;
        }
        let rows_constant_grant_status_1002 = crate::Q::grant_statuses().select_self_fields().with_id_is(1002_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_grant_status_1002) = rows_constant_grant_status_1002.data.into_iter().next() {
            let mut changed = false;
            if constant_grant_status_1002.media_library_id() != 1_u64 { constant_grant_status_1002.update_media_library_id(1_u64); changed = true; }
            if constant_grant_status_1002.name() != "Expired" { constant_grant_status_1002.update_name("Expired"); changed = true; }
            if constant_grant_status_1002.code() != "EXPIRED" { constant_grant_status_1002.update_code("EXPIRED"); changed = true; }
            if constant_grant_status_1002.display_order() != rust_decimal::Decimal::from_str_exact("2").unwrap() { constant_grant_status_1002.update_display_order(rust_decimal::Decimal::from_str_exact("2").unwrap()); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_grant_status_1002.audit_as("reconcile model constant GrantStatus(1002)"), context).await?; }
        } else {
            let mut constant_grant_status_1002 = GrantStatus::runtime_new(context.entity_runtime_state());
            constant_grant_status_1002.update_id(1002_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_grant_status_1002, GrantStatus::ENTITY_NAME, 1002_u64)?;
            constant_grant_status_1002.update_media_library_id(1_u64);
            constant_grant_status_1002.update_name("Expired");
            constant_grant_status_1002.update_code("EXPIRED");
            constant_grant_status_1002.update_display_order(rust_decimal::Decimal::from_str_exact("2").unwrap());
            let _ = teaql_runtime::AuditedSaveExt::save(constant_grant_status_1002.audit_as("create model constant GrantStatus(1002)"), context).await?;
        }
        let rows_constant_grant_status_1003 = crate::Q::grant_statuses().select_self_fields().with_id_is(1003_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_grant_status_1003) = rows_constant_grant_status_1003.data.into_iter().next() {
            let mut changed = false;
            if constant_grant_status_1003.media_library_id() != 1_u64 { constant_grant_status_1003.update_media_library_id(1_u64); changed = true; }
            if constant_grant_status_1003.name() != "Revoked" { constant_grant_status_1003.update_name("Revoked"); changed = true; }
            if constant_grant_status_1003.code() != "REVOKED" { constant_grant_status_1003.update_code("REVOKED"); changed = true; }
            if constant_grant_status_1003.display_order() != rust_decimal::Decimal::from_str_exact("3").unwrap() { constant_grant_status_1003.update_display_order(rust_decimal::Decimal::from_str_exact("3").unwrap()); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_grant_status_1003.audit_as("reconcile model constant GrantStatus(1003)"), context).await?; }
        } else {
            let mut constant_grant_status_1003 = GrantStatus::runtime_new(context.entity_runtime_state());
            constant_grant_status_1003.update_id(1003_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_grant_status_1003, GrantStatus::ENTITY_NAME, 1003_u64)?;
            constant_grant_status_1003.update_media_library_id(1_u64);
            constant_grant_status_1003.update_name("Revoked");
            constant_grant_status_1003.update_code("REVOKED");
            constant_grant_status_1003.update_display_order(rust_decimal::Decimal::from_str_exact("3").unwrap());
            let _ = teaql_runtime::AuditedSaveExt::save(constant_grant_status_1003.audit_as("create model constant GrantStatus(1003)"), context).await?;
        }
        let rows_constant_invite_status_1001 = crate::Q::invite_statuses().select_self_fields().with_id_is(1001_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_invite_status_1001) = rows_constant_invite_status_1001.data.into_iter().next() {
            let mut changed = false;
            if constant_invite_status_1001.media_library_id() != 1_u64 { constant_invite_status_1001.update_media_library_id(1_u64); changed = true; }
            if constant_invite_status_1001.name() != "Fresh" { constant_invite_status_1001.update_name("Fresh"); changed = true; }
            if constant_invite_status_1001.code() != "FRESH" { constant_invite_status_1001.update_code("FRESH"); changed = true; }
            if constant_invite_status_1001.display_order() != rust_decimal::Decimal::from_str_exact("1").unwrap() { constant_invite_status_1001.update_display_order(rust_decimal::Decimal::from_str_exact("1").unwrap()); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_invite_status_1001.audit_as("reconcile model constant InviteStatus(1001)"), context).await?; }
        } else {
            let mut constant_invite_status_1001 = InviteStatus::runtime_new(context.entity_runtime_state());
            constant_invite_status_1001.update_id(1001_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_invite_status_1001, InviteStatus::ENTITY_NAME, 1001_u64)?;
            constant_invite_status_1001.update_media_library_id(1_u64);
            constant_invite_status_1001.update_name("Fresh");
            constant_invite_status_1001.update_code("FRESH");
            constant_invite_status_1001.update_display_order(rust_decimal::Decimal::from_str_exact("1").unwrap());
            let _ = teaql_runtime::AuditedSaveExt::save(constant_invite_status_1001.audit_as("create model constant InviteStatus(1001)"), context).await?;
        }
        let rows_constant_invite_status_1002 = crate::Q::invite_statuses().select_self_fields().with_id_is(1002_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_invite_status_1002) = rows_constant_invite_status_1002.data.into_iter().next() {
            let mut changed = false;
            if constant_invite_status_1002.media_library_id() != 1_u64 { constant_invite_status_1002.update_media_library_id(1_u64); changed = true; }
            if constant_invite_status_1002.name() != "Claimed" { constant_invite_status_1002.update_name("Claimed"); changed = true; }
            if constant_invite_status_1002.code() != "CLAIMED" { constant_invite_status_1002.update_code("CLAIMED"); changed = true; }
            if constant_invite_status_1002.display_order() != rust_decimal::Decimal::from_str_exact("2").unwrap() { constant_invite_status_1002.update_display_order(rust_decimal::Decimal::from_str_exact("2").unwrap()); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_invite_status_1002.audit_as("reconcile model constant InviteStatus(1002)"), context).await?; }
        } else {
            let mut constant_invite_status_1002 = InviteStatus::runtime_new(context.entity_runtime_state());
            constant_invite_status_1002.update_id(1002_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_invite_status_1002, InviteStatus::ENTITY_NAME, 1002_u64)?;
            constant_invite_status_1002.update_media_library_id(1_u64);
            constant_invite_status_1002.update_name("Claimed");
            constant_invite_status_1002.update_code("CLAIMED");
            constant_invite_status_1002.update_display_order(rust_decimal::Decimal::from_str_exact("2").unwrap());
            let _ = teaql_runtime::AuditedSaveExt::save(constant_invite_status_1002.audit_as("create model constant InviteStatus(1002)"), context).await?;
        }
        let rows_constant_invite_status_1003 = crate::Q::invite_statuses().select_self_fields().with_id_is(1003_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_invite_status_1003) = rows_constant_invite_status_1003.data.into_iter().next() {
            let mut changed = false;
            if constant_invite_status_1003.media_library_id() != 1_u64 { constant_invite_status_1003.update_media_library_id(1_u64); changed = true; }
            if constant_invite_status_1003.name() != "Spent" { constant_invite_status_1003.update_name("Spent"); changed = true; }
            if constant_invite_status_1003.code() != "SPENT" { constant_invite_status_1003.update_code("SPENT"); changed = true; }
            if constant_invite_status_1003.display_order() != rust_decimal::Decimal::from_str_exact("3").unwrap() { constant_invite_status_1003.update_display_order(rust_decimal::Decimal::from_str_exact("3").unwrap()); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_invite_status_1003.audit_as("reconcile model constant InviteStatus(1003)"), context).await?; }
        } else {
            let mut constant_invite_status_1003 = InviteStatus::runtime_new(context.entity_runtime_state());
            constant_invite_status_1003.update_id(1003_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_invite_status_1003, InviteStatus::ENTITY_NAME, 1003_u64)?;
            constant_invite_status_1003.update_media_library_id(1_u64);
            constant_invite_status_1003.update_name("Spent");
            constant_invite_status_1003.update_code("SPENT");
            constant_invite_status_1003.update_display_order(rust_decimal::Decimal::from_str_exact("3").unwrap());
            let _ = teaql_runtime::AuditedSaveExt::save(constant_invite_status_1003.audit_as("create model constant InviteStatus(1003)"), context).await?;
        }
        Ok(())
}


/// Canonical KSML field to selected JSON wire name, consumed by HTTP/TFP adapters.
pub fn generated_wire_field_mappings() -> std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>> {
    std::collections::BTreeMap::from([
        ("MediaLibrary".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("library_name".to_owned(), "libraryName".to_owned()),
            ("root_path".to_owned(), "rootPath".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("MediaAsset".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("media_library".to_owned(), "mediaLibrary".to_owned()),
            ("display_name".to_owned(), "displayName".to_owned()),
            ("file_path".to_owned(), "filePath".to_owned()),
            ("content_type".to_owned(), "contentType".to_owned()),
            ("file_size".to_owned(), "fileSize".to_owned()),
            ("duration_seconds".to_owned(), "durationSeconds".to_owned()),
            ("active".to_owned(), "active".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("GrantStatus".to_owned(), std::collections::BTreeMap::from([
            ("media_library".to_owned(), "mediaLibrary".to_owned()),
            ("id".to_owned(), "id".to_owned()),
            ("name".to_owned(), "name".to_owned()),
            ("code".to_owned(), "code".to_owned()),
            ("display_order".to_owned(), "displayOrder".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("IpAccessGrant".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("media_library".to_owned(), "mediaLibrary".to_owned()),
            ("media_asset".to_owned(), "mediaAsset".to_owned()),
            ("grant_status".to_owned(), "grantStatus".to_owned()),
            ("client_ip".to_owned(), "clientIp".to_owned()),
            ("expire_at".to_owned(), "expireAt".to_owned()),
            ("grant_note".to_owned(), "grantNote".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("InviteStatus".to_owned(), std::collections::BTreeMap::from([
            ("media_library".to_owned(), "mediaLibrary".to_owned()),
            ("id".to_owned(), "id".to_owned()),
            ("name".to_owned(), "name".to_owned()),
            ("code".to_owned(), "code".to_owned()),
            ("display_order".to_owned(), "displayOrder".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("InviteCode".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("media_library".to_owned(), "mediaLibrary".to_owned()),
            ("media_asset".to_owned(), "mediaAsset".to_owned()),
            ("invite_status".to_owned(), "inviteStatus".to_owned()),
            ("invite_token".to_owned(), "inviteToken".to_owned()),
            ("first_client_ip".to_owned(), "firstClientIp".to_owned()),
            ("first_click_at".to_owned(), "firstClickAt".to_owned()),
            ("second_client_ip".to_owned(), "secondClientIp".to_owned()),
            ("second_click_at".to_owned(), "secondClickAt".to_owned()),
            ("grant_note".to_owned(), "grantNote".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("AppSetting".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("media_library".to_owned(), "mediaLibrary".to_owned()),
            ("setting_key".to_owned(), "settingKey".to_owned()),
            ("setting_value".to_owned(), "settingValue".to_owned()),
            ("setting_name".to_owned(), "settingName".to_owned()),
            ("setting_note".to_owned(), "settingNote".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("ProxyTrustRule".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("media_library".to_owned(), "mediaLibrary".to_owned()),
            ("header_name".to_owned(), "headerName".to_owned()),
            ("trusted_proxy_cidrs".to_owned(), "trustedProxyCidrs".to_owned()),
            ("enabled".to_owned(), "enabled".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("ShareHandle".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("media_library".to_owned(), "mediaLibrary".to_owned()),
            ("media_asset".to_owned(), "mediaAsset".to_owned()),
            ("handle_token".to_owned(), "handleToken".to_owned()),
            ("active".to_owned(), "active".to_owned()),
            ("handle_note".to_owned(), "handleNote".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("UserSession".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("media_library".to_owned(), "mediaLibrary".to_owned()),
            ("session_fp".to_owned(), "sessionFp".to_owned()),
            ("active".to_owned(), "active".to_owned()),
            ("last_seen_at".to_owned(), "lastSeenAt".to_owned()),
            ("expire_at".to_owned(), "expireAt".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ]))
    ])
}

/// Accepted legacy aliases; empty until explicitly declared by the model.
pub fn generated_wire_field_aliases() -> std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>> {
    std::collections::BTreeMap::new()
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<MediaLibrary>()
        .entity::<MediaAsset>()
        .entity::<GrantStatus>()
        .entity::<IpAccessGrant>()
        .entity::<InviteStatus>()
        .entity::<InviteCode>()
        .entity::<AppSetting>()
        .entity::<ProxyTrustRule>()
        .entity::<ShareHandle>()
        .entity::<UserSession>()
        .generated_schema_bootstrap(ensure_generated_bootstrap)
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    let mut module = teaql_runtime::RuntimeModule::new();
    module = module.entity::<MediaLibrary>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<MediaLibrary, _>::new(MediaLibraryChecker::default()));
    module = module.entity::<MediaAsset>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<MediaAsset, _>::new(MediaAssetChecker::default()));
    module = module.entity::<GrantStatus>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<GrantStatus, _>::new(GrantStatusChecker::default()));
    module = module.entity::<IpAccessGrant>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<IpAccessGrant, _>::new(IpAccessGrantChecker::default()));
    module = module.entity::<InviteStatus>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<InviteStatus, _>::new(InviteStatusChecker::default()));
    module = module.entity::<InviteCode>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<InviteCode, _>::new(InviteCodeChecker::default()));
    module = module.entity::<AppSetting>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<AppSetting, _>::new(AppSettingChecker::default()));
    module = module.entity::<ProxyTrustRule>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<ProxyTrustRule, _>::new(ProxyTrustRuleChecker::default()));
    module = module.entity::<ShareHandle>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<ShareHandle, _>::new(ShareHandleChecker::default()));
    module = module.entity::<UserSession>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<UserSession, _>::new(UserSessionChecker::default()));
    module = module.generated_schema_bootstrap(ensure_generated_bootstrap);
    module
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    let mut module = teaql_runtime::RuntimeModule::new();
    module = module.entity_with_behavior::<MediaLibrary, _>(MediaLibraryBehavior::default());
    module = module.entity_with_behavior::<MediaAsset, _>(MediaAssetBehavior::default());
    module = module.entity_with_behavior::<GrantStatus, _>(GrantStatusBehavior::default());
    module = module.entity_with_behavior::<IpAccessGrant, _>(IpAccessGrantBehavior::default());
    module = module.entity_with_behavior::<InviteStatus, _>(InviteStatusBehavior::default());
    module = module.entity_with_behavior::<InviteCode, _>(InviteCodeBehavior::default());
    module = module.entity_with_behavior::<AppSetting, _>(AppSettingBehavior::default());
    module = module.entity_with_behavior::<ProxyTrustRule, _>(ProxyTrustRuleBehavior::default());
    module = module.entity_with_behavior::<ShareHandle, _>(ShareHandleBehavior::default());
    module = module.entity_with_behavior::<UserSession, _>(UserSessionBehavior::default());
    module = module.generated_schema_bootstrap(ensure_generated_bootstrap);
    module
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    let mut module = teaql_runtime::RuntimeModule::new();
    module = module.entity_with_behavior::<MediaLibrary, _>(MediaLibraryBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<MediaLibrary, _>::new(MediaLibraryChecker::default()));
    module = module.entity_with_behavior::<MediaAsset, _>(MediaAssetBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<MediaAsset, _>::new(MediaAssetChecker::default()));
    module = module.entity_with_behavior::<GrantStatus, _>(GrantStatusBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<GrantStatus, _>::new(GrantStatusChecker::default()));
    module = module.entity_with_behavior::<IpAccessGrant, _>(IpAccessGrantBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<IpAccessGrant, _>::new(IpAccessGrantChecker::default()));
    module = module.entity_with_behavior::<InviteStatus, _>(InviteStatusBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<InviteStatus, _>::new(InviteStatusChecker::default()));
    module = module.entity_with_behavior::<InviteCode, _>(InviteCodeBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<InviteCode, _>::new(InviteCodeChecker::default()));
    module = module.entity_with_behavior::<AppSetting, _>(AppSettingBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<AppSetting, _>::new(AppSettingChecker::default()));
    module = module.entity_with_behavior::<ProxyTrustRule, _>(ProxyTrustRuleBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<ProxyTrustRule, _>::new(ProxyTrustRuleChecker::default()));
    module = module.entity_with_behavior::<ShareHandle, _>(ShareHandleBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<ShareHandle, _>::new(ShareHandleChecker::default()));
    module = module.entity_with_behavior::<UserSession, _>(UserSessionBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<UserSession, _>::new(UserSessionChecker::default()));
    module = module.generated_schema_bootstrap(ensure_generated_bootstrap);
    module
}