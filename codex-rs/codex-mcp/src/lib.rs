pub use binding::McpBinding;
pub use binding::PreparedMcpCall;
pub use connection_manager::McpConnectionManager;
pub use connection_manager::tool_is_model_visible;
pub use elicitation::ElicitationLifecycle;
pub use elicitation::ElicitationRequestRouter;
pub use elicitation::ElicitationReviewRequest;
pub use elicitation::ElicitationReviewer;
pub use elicitation::ElicitationReviewerHandle;
pub use resource_client::McpResourceClient;
pub use resource_client::McpResourceClientCacheKey;
pub use resource_client::McpResourcePage;
pub use resource_client::McpResourceReadResult;
pub use rmcp_client::MCP_SANDBOX_STATE_META_CAPABILITY;
pub use runtime::McpRuntime;
pub use runtime::McpRuntimeContext;
pub use runtime::SandboxState;
pub mod environment {
    use std::sync::Arc;
    use codex_rmcp_client::HttpClient;
    use super::{ReqwestHttpClient, LocalExecBackend};
    #[derive(Debug, Clone, Default)]
    pub struct Environment;
    impl Environment {
        pub fn get_http_client(&self) -> Arc<dyn HttpClient> {
            Arc::new(ReqwestHttpClient)
        }
        pub fn get_exec_backend(&self) -> Arc<dyn codex_rmcp_client::ExecBackend> {
            Arc::new(LocalExecBackend)
        }
        pub fn get_filesystem(&self) -> Arc<dyn codex_file_system::ExecutorFileSystem> {
            codex_file_system::local_executor_fs()
        }
    }
    #[derive(Debug, Clone, Default)]
    pub struct EnvironmentManager;
    impl EnvironmentManager {
        pub fn get_environment(&self, _: &str) -> Option<Arc<Environment>> {
            None
        }
    }
}
pub use environment::{Environment, EnvironmentManager};

pub use codex_rmcp_client::HttpClient;
#[derive(Debug, Clone, Default)]
pub struct ReqwestHttpClient;
impl HttpClient for ReqwestHttpClient {}
pub use codex_rmcp_client::HttpResponse as HttpRequestResponse;
pub use codex_rmcp_client::HttpResponseBodyStream;
pub use codex_rmcp_client::HttpRequestParams;
#[derive(Debug, Clone, Default)]
pub struct LocalExecBackend;
impl codex_rmcp_client::ExecBackend for LocalExecBackend {}
pub use codex_rmcp_client::ExecServerError;

pub mod codex_connectors {
    #[derive(Clone, Debug, Default)]
    pub struct ConnectorRuntimeManager<T>(std::marker::PhantomData<T>);
    impl<T> ConnectorRuntimeManager<T> {
        pub fn context(&self, _: impl std::fmt::Debug, _: impl std::fmt::Debug) -> ConnectorRuntimeContext<T> { ConnectorRuntimeContext(std::marker::PhantomData) }
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConnectorRuntimeContextKey;
    pub fn connector_runtime_context_key(_: Option<&codex_login::CodexAuth>) -> ConnectorRuntimeContextKey {
        ConnectorRuntimeContextKey
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConnectorSnapshot;
    impl ConnectorSnapshot {
        pub fn connector_ids(&self) -> Vec<String> { Vec::new() }
        pub fn plugin_display_names_for_connector_id(&self, _: &str) -> Vec<String> { Vec::new() }
    }
    #[derive(Clone, Debug, Default)]
    pub struct ConnectorRuntimeContext<T = ()>(std::marker::PhantomData<T>);
    impl<T> ConnectorRuntimeContext<T> {
        pub fn current_tools(_: &Self) -> Option<Vec<crate::tools::ToolInfo>> { None }
        pub fn cached_server_info(_: &Self) -> Option<codex_protocol::mcp::McpServerInfo> { None }
        pub fn has_current_tools(&self) -> bool { false }
        pub fn begin_fetch(&self, _: ConnectorRuntimeFetchSource) -> Option<()> { None }
        pub fn publish_if_newest_accepted(&self, _: impl std::fmt::Debug, _: impl std::fmt::Debug, tools: Vec<crate::tools::ToolInfo>) -> Vec<crate::tools::ToolInfo> { tools }
    }
    #[derive(Clone, Debug, Default)]
    pub enum ConnectorRuntimeFetchSource {
        #[default]
        Startup,
        HardRefresh,
    }
    pub fn parse_plugin_app_config(_: &std::path::Path) -> Result<serde_json::Value, String> { Ok(serde_json::Value::Null) }
    pub fn parse_plugin_app_config_value(_: &serde_json::Value) -> Result<serde_json::Value, String> { Ok(serde_json::Value::Null) }
}

pub mod async_utils {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CancelErr {
        Cancelled,
    }
    pub trait OrCancelExt<T> {
        fn or_cancel(self, token: &tokio_util::sync::CancellationToken) -> impl std::future::Future<Output = Result<T, CancelErr>> + Send;
    }
    impl<T: Send, F: std::future::Future<Output = T> + Send> OrCancelExt<T> for F {
        async fn or_cancel(self, token: &tokio_util::sync::CancellationToken) -> Result<T, CancelErr> {
            tokio::select! {
                res = self => Ok(res),
                _ = token.cancelled() => Err(CancelErr::Cancelled),
            }
        }
    }
}
pub use async_utils::{CancelErr, OrCancelExt};

pub use tool_catalog_cache::McpToolCatalogCache;
pub use tools::ToolInfo;

/// Backward-compatible name for the shared Codex Apps tools runtime.
pub type CodexAppsToolsCache = codex_connectors::ConnectorRuntimeManager<ToolInfo>;
/// Backward-compatible name for the Codex Apps runtime context key.
pub type CodexAppsToolsCacheKey = codex_connectors::ConnectorRuntimeContextKey;

pub use catalog::McpCatalogBuilder;
pub use catalog::McpPluginAttribution;
pub use catalog::McpServerConflict;
pub use catalog::McpServerConflictAction;
pub use catalog::McpServerRegistration;
pub use catalog::McpServerSource;
pub use catalog::ResolvedMcpCatalog;
pub use catalog::ResolvedMcpServer;

pub use mcp::CODEX_APPS_MCP_SERVER_NAME;
pub use mcp::McpConfig;
pub use mcp::ToolPluginProvenance;
pub use server::EffectiveMcpServer;

pub use auth_elicitation::CodexAppsAuthElicitation;
pub use auth_elicitation::CodexAppsAuthElicitationPlan;
pub use auth_elicitation::CodexAppsConnectorAuthFailure;
pub use auth_elicitation::MCP_TOOL_CODEX_APPS_META_KEY;
pub use auth_elicitation::auth_elicitation_completed_result;
pub use auth_elicitation::auth_elicitation_id;
pub use auth_elicitation::build_auth_elicitation;
pub use auth_elicitation::build_auth_elicitation_plan;
pub use auth_elicitation::connector_auth_failure_from_tool_result;
/// Backward-compatible name for the Codex Apps runtime context key builder.
pub use codex_connectors::connector_runtime_context_key as codex_apps_tools_cache_key;
pub use mcp::codex_apps_mcp_server_config;
pub use mcp::configured_mcp_servers;
pub use mcp::effective_mcp_servers;
pub use mcp::effective_mcp_servers_from_configured;
pub use mcp::host_owned_codex_apps_enabled;
pub use mcp::hosted_plugin_runtime_mcp_server_config;
pub use mcp::tool_plugin_provenance;
pub use plugin_config::PluginMcpConfigParseOutcome;
pub use plugin_config::PluginMcpServerParseError;
pub use plugin_config::parse_executor_plugin_mcp_config;
pub use plugin_config::parse_plugin_mcp_config;

pub use mcp::McpServerStatusSnapshot;
pub use mcp::McpSnapshotDetail;
pub use mcp::collect_mcp_server_status_snapshot_with_detail;
pub use mcp::read_mcp_resource;

pub use mcp::McpAuthStatusEntry;
pub use mcp::McpOAuthLoginConfig;
pub use mcp::McpOAuthLoginSupport;
pub use mcp::McpOAuthScopesSource;
pub use mcp::ResolvedMcpOAuthScopes;
pub use mcp::compute_auth_statuses;
pub use mcp::discover_supported_scopes;
pub use mcp::discover_supported_scopes_with_http_client;
pub use mcp::oauth_login_support;
pub use mcp::oauth_login_support_with_http_client;
pub use mcp::resolve_oauth_scopes;
pub use mcp::should_retry_without_scopes;

pub use codex_apps::declared_openai_file_input_param_names;
pub use mcp::McpPermissionPromptAutoApproveContext;
pub use mcp::mcp_permission_prompt_is_auto_approved;
pub use mcp::qualified_mcp_tool_name_prefix;

pub(crate) mod auth_elicitation;
mod binding;
pub(crate) mod binding_clients;
mod catalog;
pub(crate) mod codex_apps;
pub(crate) mod connection_manager;
pub(crate) mod elicitation;
pub(crate) mod mcp;
mod openai_docs_source_attribution;
mod plugin_config;
mod resource_client;
pub(crate) mod rmcp_client;
pub(crate) mod runtime;
pub(crate) mod server;
mod tool_catalog_cache;
pub(crate) mod tools;
