use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}
