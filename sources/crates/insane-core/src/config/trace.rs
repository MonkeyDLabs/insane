use crate::traces::TraceFormat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceConfig {
    format: TraceFormat,
    pub filter: String,
    pub pretty_backtrace: bool,
}

impl TraceConfig {
    pub fn format(&self) -> &TraceFormat {
        &self.format
    }

    pub fn filter(&self) -> &str {
        self.filter.as_str()
    }
}

impl Default for TraceConfig {
    fn default() -> Self {
        TraceConfig {
            format: Default::default(),
            filter: "info".to_string(),
            pretty_backtrace: false,
        }
    }
}