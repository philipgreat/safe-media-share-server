
//! # Media Guard Axum Web Workspace
//!
//! IP-whitelist protected media server (Range/resume) with one-shot invite links.
//! Domain access goes through generated Q/E APIs only.

pub use media_guard_service_core::{teaql_core, E, Q};
pub use teaql_web_integration_axum::{AxumTeaError, ContextProvider, TeaContext, WebResponse};

pub mod entity_ref;
pub mod handlers;
pub mod proxy;
pub mod range;
pub mod service;
pub mod settings;

pub fn generated_domain_crate() -> &'static str {
    "media-guard-service-core"
}

/// Build a request-scoped UserContext with the same wiring as
/// `service_runtime_from_pool` (id generator + sqlite provider + executor).
pub fn build_request_context(
    pool: &media_guard_service_core::DataServicePool,
) -> teaql_runtime::UserContext {
    use media_guard_service_core::{
        DataServiceIdGenerator, DataServiceMutationExecutor, ServiceRuntimeExecutor,
    };
    use teaql_provider_sqlite::SqliteProviderExt as _;

    let mut context = media_guard_service_core::module_with_behaviors_and_checkers().into_context();
    let mutation_executor = DataServiceMutationExecutor::new(pool.clone());
    let id_generator = DataServiceIdGenerator::from_executor(mutation_executor.clone());
    context.set_internal_id_generator(id_generator);
    context.use_sqlite_provider(mutation_executor.clone());
    let executor = ServiceRuntimeExecutor::new(mutation_executor);
    context.register_executor(executor.clone());
    context.insert_resource(executor);
    context.set_entity_reference_codec(crate::entity_ref::shared_codec());
    context
}
