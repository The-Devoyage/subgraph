use serde_json::Value;

pub mod field_filter;
pub mod filter_config;
pub mod stats;

pub struct RequestFilterLanguage {
    pub json: Value,
}

impl RequestFilterLanguage {
    pub fn new(json: Value) -> Self {
        Self { json }
    }
}
