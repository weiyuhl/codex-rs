use std::error::Error;

#[derive(Clone, Debug, Default)]
pub struct OtelProvider;

impl OtelProvider {
    pub fn shutdown(&self) {}

    pub fn from<T>(_settings: &T) -> Result<Option<Self>, Box<dyn Error>> {
        Ok(None)
    }

    pub fn metrics(&self) -> Option<&()> {
        None
    }
}
