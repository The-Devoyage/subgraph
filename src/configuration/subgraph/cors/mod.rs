use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MethodOption {
    #[serde(with = "http_serde::method")]
    pub method: Method,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CorsConfigOptions {
    pub allow_methods: Option<Vec<MethodOption>>,
    pub allow_headers: Option<Vec<String>>,
    pub allow_origins: Option<Vec<String>>,
    pub allow_any_origin: Option<bool>,
}
