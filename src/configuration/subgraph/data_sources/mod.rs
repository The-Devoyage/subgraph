use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceDataSourceConfig {
    MongoDataSourceConfig(MongoDataSourceConfig),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataSourceType {
    MONGO,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoDataSourceConfig {
    pub name: String,
    pub source_type: DataSourceType,
    pub uri: String,
    pub db: String,
}
