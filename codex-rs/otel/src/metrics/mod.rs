mod config;
mod error;
pub(crate) mod names;
pub(crate) mod runtime_metrics;
pub(crate) mod tags;

pub use names::*;
pub use tags::ORIGINATOR_TAG;
pub use tags::SessionMetricTagValues;
pub use tags::bounded_originator_tag_value;

#[derive(Clone, Debug, Default)]
pub struct MetricsClient;

impl MetricsClient {
    pub fn shutdown(&self) {}
    pub fn start_timer(&self, _name: &str, _tags: &[(&str, &str)]) -> Result<Timer, crate::metrics::error::MetricsError> {
        Err(crate::metrics::error::MetricsError::ExporterDisabled)
    }
    pub fn counter(&self, _name: &str, _val: i64, _tags: &[(&str, &str)]) -> Result<(), crate::metrics::error::MetricsError> {
        Ok(())
    }
    pub fn record_duration(&self, _name: &str, _duration: std::time::Duration, _tags: &[(&str, &str)]) -> Result<(), crate::metrics::error::MetricsError> {
        Ok(())
    }
    pub fn histogram<V>(&self, _name: &str, _val: V, _tags: &[(&str, &str)]) -> Result<(), crate::metrics::error::MetricsError> {
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct StatsigMetricsSettings;

#[derive(Clone, Debug, Default)]
pub struct Timer;

impl Timer {
    pub fn record(&self, _tags: &[(&str, &str)]) -> Result<(), crate::metrics::error::MetricsError> {
        Ok(())
    }
}

pub fn global() -> Option<MetricsClient> {
    None
}

pub fn start_global_timer(_name: &str, _tags: &[(&str, &str)]) -> Result<Timer, crate::metrics::error::MetricsError> {
    Err(crate::metrics::error::MetricsError::ExporterDisabled)
}

pub fn global_statsig_settings() -> Option<StatsigMetricsSettings> {
    None
}
