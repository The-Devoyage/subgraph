use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DefaultHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HttpDataSourceConfig {
    pub name: String,
    pub url: String,
    pub default_headers: Option<Vec<DefaultHeader>>,
}
