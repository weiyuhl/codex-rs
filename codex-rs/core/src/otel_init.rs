use crate::config::Config;
use codex_config::types::OtelExporterKind as Kind;
use codex_config::types::OtelHttpProtocol as Protocol;
use codex_features::Feature;
use codex_login::default_client::originator;
use codex_otel::OtelExporter;
use codex_otel::OtelHttpProtocol;
use codex_otel::OtelProvider;
use codex_otel::OtelSettings;
use codex_otel::OtelTlsConfig as OtelTlsSettings;
use std::error::Error;

/// Build an OpenTelemetry provider from the app Config.
///
/// Returns `None` when OTEL export is disabled.
pub fn build_provider(
    config: &Config,
    service_version: &str,
    service_name_override: Option<&str>,
    default_analytics_enabled: bool,
) -> Result<Option<OtelProvider>, Box<dyn Error>> {
    let _ = (config, service_version, service_name_override, default_analytics_enabled);
    // OpenTelemetry exporter is paused for Android/Private Agent mode.
    Ok(None)
}

pub fn record_process_start(otel: Option<&OtelProvider>, originator: &str) {
    let Some(metrics) = otel.and_then(OtelProvider::metrics) else {
        return;
    };
    let _ = codex_otel::record_process_start_once(metrics, originator);
}

pub fn install_sqlite_telemetry(otel: Option<&OtelProvider>, originator: &str) {
    let Some(metrics) = otel.and_then(OtelProvider::metrics) else {
        return;
    };
    let telemetry = codex_rollout::sqlite_telemetry_recorder(metrics.clone(), originator);
    let _ = codex_state::install_process_db_telemetry(telemetry);
}
