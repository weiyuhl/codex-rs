use std::collections::BTreeMap;
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum MetricsExporter {
    Otlp(()),
    InMemory(()),
}

#[derive(Clone, Debug)]
pub struct MetricsConfig {
    pub(crate) environment: String,
    pub(crate) service_name: String,
    pub(crate) service_version: String,
    pub(crate) exporter: MetricsExporter,
    pub(crate) export_interval: Option<Duration>,
    pub(crate) runtime_reader: bool,
    pub(crate) default_tags: BTreeMap<String, String>,
}

impl MetricsConfig {
    pub fn otlp(
        environment: impl Into<String>,
        service_name: impl Into<String>,
        service_version: impl Into<String>,
        _exporter: (),
    ) -> Self {
        Self {
            environment: environment.into(),
            service_name: service_name.into(),
            service_version: service_version.into(),
            exporter: MetricsExporter::Otlp(()),
            export_interval: None,
            runtime_reader: false,
            default_tags: BTreeMap::new(),
        }
    }

    pub fn with_export_interval(mut self, interval: Duration) -> Self {
        self.export_interval = Some(interval);
        self
    }

    pub fn with_runtime_reader(mut self) -> Self {
        self.runtime_reader = true;
        self
    }
}
