use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const CONNECTORS_CACHE_TTL: Duration = Duration::from_secs(300);

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppBranding;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppMetadata;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorDirectoryCacheContext;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConnectorDirectoryCacheKey;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppToolPolicy;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppToolPolicyEvaluator;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppToolPolicyInput;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorSnapshot;

impl ConnectorSnapshot {
    pub fn from_plugin_capability_summaries(_summaries: &[()]) -> Self {
        Self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginConnectorSource;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorMetadata;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorMetadataStore;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorToolSummary;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DirectoryListResponse;

pub mod accessible {
    use super::*;

    #[derive(Debug, Clone, Default)]
    pub struct AccessibleConnectorTool;

    pub fn collect_accessible_connectors<T>(_tools: T) -> Vec<AccessibleConnectorTool> {
        Vec::new()
    }
}

pub mod metadata {
    pub fn connector_install_url(_connector: &str) -> String {
        String::new()
    }
    pub fn connector_display_label(_connector: &str) -> String {
        String::new()
    }
    pub fn connector_mention_slug(_connector: &str) -> String {
        String::new()
    }
    pub fn sanitize_name(name: &str) -> String {
        name.to_string()
    }
}

pub mod merge {
    use super::*;

    pub fn merge_connectors() -> Vec<AppInfo> {
        Vec::new()
    }
    pub fn merge_plugin_connectors(_a: &[()], _b: &[()]) -> Vec<AppInfo> {
        Vec::new()
    }
    pub fn merge_plugin_connectors_with_accessible(_a: &[()], _b: &[()]) -> Vec<AppInfo> {
        Vec::new()
    }
    pub fn plugin_connector_to_app_info(_p: &str) -> AppInfo {
        AppInfo::default()
    }
}

pub mod filter {
    use super::*;

    pub fn filter_tool_suggest_discoverable_connectors(connectors: Vec<AppInfo>) -> Vec<AppInfo> {
        connectors
    }
}

pub fn cached_directory_connectors(_ctx: &ConnectorDirectoryCacheContext) -> Option<Vec<AppInfo>> {
    Some(Vec::new())
}

pub fn connector_runtime_context_key(_auth: Option<&()>) -> String {
    String::new()
}

pub fn apps_config_from_layer_stack() -> Option<()> {
    None
}

pub fn app_is_enabled(_app_id: &str) -> bool {
    false
}
