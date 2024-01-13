use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DialectEnum {
    POSTGRES,
    MYSQL,
    SQLITE,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SqlDataSourceConfig {
    pub name: String,
    pub uri: String,
    pub dialect: DialectEnum,
    pub sqlite_extensions: Option<Vec<String>>,
    pub migrations_path: Option<String>,
}
