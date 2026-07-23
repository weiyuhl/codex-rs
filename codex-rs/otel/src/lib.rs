mod events;
pub(crate) mod metrics;
pub(crate) mod provider;

use codex_protocol::auth::AuthMode;
use serde::Serialize;
use strum_macros::Display;
pub use crate::metrics::runtime_metrics::RuntimeMetricTotals;
pub use crate::metrics::runtime_metrics::RuntimeMetricsSummary;
pub use crate::metrics::*;
pub use crate::provider::OtelProvider;
pub use codex_utils_string::sanitize_metric_tag_value;

#[derive(Debug, Clone, Serialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ToolDecisionSource {
    AutomatedReviewer,
    Config,
    User,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum TelemetryAuthMode {
    ApiKey,
    Chatgpt,
}

impl From<AuthMode> for TelemetryAuthMode {
    fn from(mode: AuthMode) -> Self {
        match mode {
            AuthMode::ApiKey | AuthMode::BedrockApiKey => Self::ApiKey,
            AuthMode::Chatgpt
            | AuthMode::ChatgptAuthTokens
            | AuthMode::Headers
            | AuthMode::AgentIdentity
            | AuthMode::PersonalAccessToken => Self::Chatgpt,
        }
    }
}

pub type OtelSettings = ();
pub type OtelExporter = ();
pub type OtelHttpProtocol = ();
pub type OtelTlsConfig = ();

pub fn record_process_start_once(_metrics: &MetricsClient, _originator: &str) {}
pub fn global_statsig_metrics_settings() -> Option<StatsigMetricsSettings> {
    None
}
