mod config_layer_source;
pub mod config_toml;
mod constraint;
mod diagnostics;
mod fingerprint;
mod hook_config;
mod host_name;
mod key_aliases;
pub mod loader;
mod marketplace_edit;
mod mcp_edit;
mod mcp_requirements;
mod mcp_types;
mod merge;
mod overrides;
pub mod permissions_toml;
mod plugin_edit;
pub mod profile_toml;
mod project_root_markers;
pub mod schema;
mod shell_environment_policy;
mod skills_config;
mod state;
mod strict_config;
mod thread_config;
pub mod types;
pub use types::ResidencyRequirement;

pub const CONFIG_TOML_FILE: &str = "config.toml";

pub use constraint::RequirementSource;
pub use codex_protocol::config_types::ProfileV2Name;
pub use codex_protocol::config_types::ProfileV2NameParseError;
pub use codex_utils_absolute_path::AbsolutePathBuf;
pub use config_layer_source::ConfigLayer;
pub use config_layer_source::ConfigLayerMetadata;
pub use config_layer_source::ConfigLayerSource;
pub use config_layer_source::format_config_layer_source;
pub use constraint::Constrained;
pub use constraint::ConstraintError;
pub use constraint::ConstraintResult;
pub use diagnostics::ConfigError;
pub use diagnostics::ConfigLoadError;
pub use diagnostics::TextPosition;
pub use diagnostics::TextRange;
pub use diagnostics::config_error_from_toml;
pub use diagnostics::config_error_from_typed_toml;
pub use diagnostics::first_layer_config_error;
pub use diagnostics::first_layer_config_error_from_entries;
pub use diagnostics::format_config_error;
pub use diagnostics::format_config_error_with_source;
pub use diagnostics::io_error_from_config_error;
pub use fingerprint::version_for_toml;
pub use hook_config::HookEventsToml;
pub use hook_config::HookHandlerConfig;
pub use hook_config::HookStateToml;
pub use hook_config::HooksFile;
pub use hook_config::HooksToml;
pub use hook_config::ManagedHooksRequirementsToml;
pub use hook_config::MatcherGroup;
pub use host_name::host_name;
pub use marketplace_edit::MarketplaceConfigUpdate;
pub use marketplace_edit::RemoveMarketplaceConfigOutcome;
pub use marketplace_edit::record_user_marketplace;
pub use marketplace_edit::remove_user_marketplace;
pub use marketplace_edit::remove_user_marketplace_config;
pub use mcp_edit::ConfigEditsBuilder;
pub use mcp_edit::load_global_mcp_servers;
pub use mcp_requirements::McpServerCommandMatcher;
pub use mcp_requirements::McpServerIdentity;
pub use mcp_requirements::McpServerRequirement;
pub use mcp_requirements::McpServerValueMatcher;
pub use mcp_types::AppToolApproval;
pub use mcp_types::DEFAULT_MCP_SERVER_ENVIRONMENT_ID;
pub use mcp_types::McpServerAuth;
pub use mcp_types::McpServerConfig;
pub use mcp_types::McpServerDisabledReason;
pub use mcp_types::McpServerEnvVar;
pub use mcp_types::McpServerOAuthConfig;
pub use mcp_types::McpServerToolConfig;
pub use mcp_types::McpServerTransportConfig;
pub use mcp_types::RawMcpServerConfig;
pub use merge::ShellEnvironmentPolicyFilterRepresentation;
pub use merge::merge_toml_values;
pub use merge::shell_environment_filter_entry;
pub use overrides::build_cli_overrides_layer;
pub use plugin_edit::PluginConfigEdit;
pub use plugin_edit::apply_user_plugin_config_edits;
pub use plugin_edit::clear_user_plugin;
pub use plugin_edit::set_user_plugin_enabled;
pub use project_root_markers::default_project_root_markers;
pub use project_root_markers::project_root_markers_from_config;
pub use shell_environment_policy::validate_shell_environment_policy_filter_config;
pub use skills_config::BundledSkillsConfig;
pub use skills_config::SkillConfig;
pub use skills_config::SkillsConfig;
pub use state::ConfigLayerEntry;
pub use state::ConfigLayerStack;
pub use state::ConfigLayerStackOrdering;
pub use state::ConfigLoadOptions;
pub use state::LoaderOverrides;
pub use strict_config::config_error_from_ignored_toml_fields;
pub use thread_config::NoopThreadConfigLoader;
pub use thread_config::SessionThreadConfig;
pub use thread_config::StaticThreadConfigLoader;
pub use thread_config::ThreadConfigContext;
pub use thread_config::ThreadConfigLoadError;
pub use thread_config::ThreadConfigLoadErrorCode;
pub use thread_config::ThreadConfigLoader;
pub use thread_config::ThreadConfigLoaderFuture;
pub use thread_config::ThreadConfigSource;
pub use thread_config::UserThreadConfig;
pub use toml::Value as TomlValue;

pub use state::ConfigRequirements;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketplaceAllowedSourceKind { Direct, Remote, Git, HostPattern, Local }

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MarketplaceAllowedSourceToml {
    pub source: Option<MarketplaceAllowedSourceKind>,
    pub source_type: Option<String>,
    pub url: Option<String>,
    pub ref_name: Option<String>,
    pub host_pattern: Option<String>,
    pub path: Option<std::path::PathBuf>,
}

