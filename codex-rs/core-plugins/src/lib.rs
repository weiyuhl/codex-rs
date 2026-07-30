mod app_mcp_routing;
mod discoverable;
mod http_client_selector;
pub mod installed_marketplaces;
pub mod loader;
mod manager;
pub mod manifest;
pub mod marketplace;
pub mod marketplace_add;
mod marketplace_policy;
pub mod marketplace_remove;
pub mod marketplace_upgrade;
mod plugin_bundle_archive;
mod provider;
pub mod remote;
pub mod remote_bundle;
pub mod remote_legacy;
pub mod startup_sync;
pub mod store;
#[cfg(test)]
mod test_support;
pub mod toggles;
mod tool_suggest_metadata;

pub const OPENAI_CURATED_MARKETPLACE_NAME: &str = "openai-curated";
pub const OPENAI_API_CURATED_MARKETPLACE_NAME: &str = "openai-api-curated";
pub const OPENAI_BUNDLED_MARKETPLACE_NAME: &str = "openai-bundled";
pub(crate) const OPENAI_BUNDLED_ALPHA_MARKETPLACE_NAME: &str = "openai-bundled-alpha";
pub(crate) const OPENAI_PRIMARY_RUNTIME_MARKETPLACE_NAME: &str = "openai-primary-runtime";

pub fn is_openai_curated_marketplace_name(marketplace_name: &str) -> bool {
    marketplace_name == OPENAI_CURATED_MARKETPLACE_NAME
        || marketplace_name == OPENAI_API_CURATED_MARKETPLACE_NAME
}

pub mod codex_plugin {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    pub struct PluginId {
        pub marketplace_name: String,
        pub plugin_name: String,
    }
    impl PluginId {
        pub fn as_key(&self) -> String { format!("{}/{}", self.marketplace_name, self.plugin_name) }
        pub fn new(plugin_name: String, marketplace_name: String) -> Result<Self, PluginIdError> {
            Ok(Self { marketplace_name, plugin_name })
        }
        pub fn parse(s: &str) -> Result<Self, PluginIdError> {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() == 2 {
                Ok(Self { marketplace_name: parts[0].to_string(), plugin_name: parts[1].to_string() })
            } else {
                Ok(Self { marketplace_name: String::new(), plugin_name: s.to_string() })
            }
        }
    }
    impl std::fmt::Display for PluginId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}/{}", self.marketplace_name, self.plugin_name)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct PluginSkillRoot {
        pub path: std::path::PathBuf,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct AppConnectorId(pub String);
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct AppDeclaration {
        pub app_id: String,
        pub name: String,
        pub connector_id: AppConnectorId,
        pub category: Option<String>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct PluginCapabilitySummary {
        pub config_name: String,
        pub display_name: String,
        pub description: Option<String>,
        pub has_skills: bool,
        pub mcp_server_names: Vec<String>,
        pub app_connector_ids: Vec<AppConnectorId>,
    }
    pub use codex_utils_plugins::PluginSkillRoot;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct PluginTelemetryMetadata {
        pub plugin_id: Option<PluginId>,
        pub remote_plugin_id: Option<String>,
        pub capability_summary: Option<PluginCapabilitySummary>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    pub enum PluginIdError {
        #[error("Invalid: {0}")]
        Invalid(String),
    }

    pub fn app_connector_ids_from_declarations(_: impl std::fmt::Debug) -> Vec<AppConnectorId> { Vec::new() }
    pub fn prompt_safe_plugin_description(_: impl std::fmt::Debug) -> Option<String> { None }
    pub fn validate_plugin_segment(_: &str, _: &str) -> Result<(), String> { Ok(()) }

    pub use codex_hooks::declarations::PluginHookSource;

    #[derive(Debug, Clone, Default)]
    pub struct LoadedPlugin<T> {
        pub config_name: String,
        pub plugin_id: String,
        pub active: bool,
        pub enabled: bool,
        pub apps: Vec<AppDeclaration>,
        pub mcp_servers: std::collections::HashMap<String, T>,
        pub manifest_name: Option<String>,
        pub root: std::path::PathBuf,
        pub error: Option<String>,
        pub plugin_namespace: Option<String>,
        pub manifest_description: Option<String>,
        pub skill_roots: Vec<PluginSkillRoot>,
        pub disabled_skill_paths: Vec<std::path::PathBuf>,
        pub has_enabled_skills: bool,
        pub hook_sources: Vec<PluginHookSource>,
        pub hook_load_warnings: Vec<String>,
    }
    impl<T> LoadedPlugin<T> {
        pub fn is_active(&self) -> bool { self.active }
    }

    #[derive(Debug, Clone)]
    pub struct PluginLoadOutcome<T> {
        pub loaded: Vec<LoadedPlugin<T>>,
        pub errors: Vec<String>,
    }
    impl<T> Default for PluginLoadOutcome<T> {
        fn default() -> Self {
            Self { loaded: Vec::new(), errors: Vec::new() }
        }
    }
    #[derive(Debug, Clone, Default)]
    pub struct CapabilitySummary {
        pub plugin_id: String,
        pub connector_id: String,
        pub app_name: String,
        pub app_connector_ids: Vec<AppConnectorId>,
    }


    impl<T> PluginLoadOutcome<T> {
        pub fn from_plugins(plugins: Vec<LoadedPlugin<T>>) -> Self {
            Self { loaded: plugins, errors: Vec::new() }
        }
        pub fn plugins(&self) -> &[LoadedPlugin<T>] { &self.loaded }
        pub fn effective_plugin_skill_roots(&self) -> Vec<PluginSkillRoot> { Vec::new() }
        pub fn capability_summaries(&self) -> Vec<CapabilitySummary> { Vec::new() }
    }

    #[derive(Debug, Clone, Default)]
    pub struct ResolvedPlugin;
    impl ResolvedPlugin {
        pub fn from_environment(_1: impl std::fmt::Debug, _2: impl std::fmt::Debug, _3: impl std::fmt::Debug, _4: impl std::fmt::Debug, _5: impl std::fmt::Debug) -> Result<Self, ResolvedPluginError> { Ok(Self) }
    }

    #[derive(Debug, Clone, Default, thiserror::Error)]
    #[error("Resolved plugin error")]
    pub struct ResolvedPluginError;

    pub trait PluginProvider {
        type Error: std::error::Error + Send + Sync + 'static;
        fn resolve(&self, root: &codex_protocol::capabilities::SelectedCapabilityRoot) -> impl std::future::Future<Output = Result<Option<ResolvedPlugin>, Self::Error>> + Send;
    }

    pub mod manifest {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct PluginManifest<T = std::path::PathBuf> {
            pub name: String,
            pub version: Option<String>,
            pub description: Option<String>,
            pub keywords: Vec<String>,
            pub interface: Option<PluginManifestInterface<T>>,
            pub paths: PluginManifestPaths<T>,
        }
        impl<T> Default for PluginManifest<T> {
            fn default() -> Self {
                Self {
                    name: String::new(),
                    version: None,
                    description: None,
                    keywords: Vec::new(),
                    interface: None,
                    paths: PluginManifestPaths::default(),
                }
            }
        }
        impl<T> PluginManifest<T> {
            pub fn display_name(&self) -> &str {
                self.interface.as_ref().and_then(|i| i.display_name.as_deref()).unwrap_or(self.name.as_str())
            }
            pub fn try_map_resources<U, E>(self, _f: impl Fn(T) -> Result<U, E>) -> Result<PluginManifest<U>, E> {
                Ok(PluginManifest {
                    name: self.name,
                    version: self.version,
                    description: self.description,
                    keywords: self.keywords,
                    interface: None,
                    paths: PluginManifestPaths::default(),
                })
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct PluginManifestInterface<T = std::path::PathBuf> {
            pub display_name: Option<String>,
            pub icon: Option<T>,
            pub logo: Option<T>,
            pub logo_dark: Option<T>,
            pub composer_icon: Option<T>,
            pub short_description: Option<String>,
            pub long_description: Option<String>,
            pub developer_name: Option<String>,
            pub category: Option<String>,
            pub capabilities: Vec<String>,
            pub website_url: Option<String>,
            pub privacy_policy_url: Option<String>,
            pub terms_of_service_url: Option<String>,
            pub default_prompt: Option<Vec<String>>,
            pub brand_color: Option<String>,
            pub screenshots: Vec<T>,
        }
        impl<T> Default for PluginManifestInterface<T> {
            fn default() -> Self {
                Self {
                    display_name: None,
                    icon: None,
                    logo: None,
                    logo_dark: None,
                    composer_icon: None,
                    short_description: None,
                    long_description: None,
                    developer_name: None,
                    category: None,
                    capabilities: Vec::new(),
                    website_url: None,
                    privacy_policy_url: None,
                    terms_of_service_url: None,
                    default_prompt: None,
                    brand_color: None,
                    screenshots: Vec::new(),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct PluginManifestPaths<T = std::path::PathBuf> {
            pub skills: Vec<T>,
            pub apps: Option<T>,
            pub hooks: Option<PluginManifestHooks<T>>,
            pub mcp_servers: Option<PluginManifestMcpServers<T>>,
        }
        impl<T> Default for PluginManifestPaths<T> {
            fn default() -> Self {
                Self {
                    skills: Vec::new(),
                    apps: None,
                    hooks: None,
                    mcp_servers: None,
                }
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum PluginManifestHooks<T = std::path::PathBuf> {
            Paths(Vec<T>),
            Inline(Vec<codex_config::HooksFile>),
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum PluginManifestMcpServers<T = std::path::PathBuf> {
            Path(T),
            Object(String),
        }
    }
}

pub type LoadedPlugin = codex_plugin::LoadedPlugin<codex_config::McpServerConfig>;
pub type PluginLoadOutcome = codex_plugin::PluginLoadOutcome<codex_config::McpServerConfig>;

pub use app_mcp_routing::apps_route_available;
pub use discoverable::ToolSuggestDiscoverablePlugin;
pub use discoverable::ToolSuggestPluginDiscoveryInput;
pub use loader::PluginHookLoadOutcome;
pub use manager::ConfiguredMarketplace;
pub use manager::ConfiguredMarketplaceListOutcome;
pub use manager::ConfiguredMarketplacePlugin;
pub use manager::EffectivePluginsChange;
pub use manager::PluginDetail;
pub use manager::PluginDetailsUnavailableReason;
pub use manager::PluginInstallError;
pub use manager::PluginInstallOutcome;
pub use manager::PluginInstallRequest;
pub use manager::PluginListBackgroundTaskOptions;
pub use manager::PluginReadOutcome;
pub use manager::PluginReadRequest;
pub use manager::PluginUninstallError;
pub use manager::PluginsConfigInput;
pub use manager::PluginsManager;
pub use manager::RecommendedPluginCandidatesInput;
pub use marketplace_policy::allowed_configured_marketplace_names;
pub use marketplace_upgrade::ConfiguredMarketplaceUpgradeError as PluginMarketplaceUpgradeError;
pub use marketplace_upgrade::ConfiguredMarketplaceUpgradeOutcome as PluginMarketplaceUpgradeOutcome;
pub use provider::ExecutorPluginProvider;
pub use provider::ExecutorPluginProviderError;
pub use provider::ResolvedExecutorPlugin;
pub use remote::RecommendedPlugin;
pub use remote::RecommendedPluginsMode;

pub mod command_migration {
    pub fn migrated_command_skills_root(_: impl std::fmt::Debug) -> Option<std::path::PathBuf> { None }
    pub fn migrate_plugin_commands(_: impl std::fmt::Debug) -> Result<(), String> { Ok(()) }
}
pub mod npm_source {
    pub fn materialize_npm_plugin_source(_: impl std::fmt::Debug, _: impl std::fmt::Debug, _: impl std::fmt::Debug, _: impl std::fmt::Debug) -> Result<(std::path::PathBuf, tempfile::TempDir), String> { Err("npm disabled".into()) }
}
