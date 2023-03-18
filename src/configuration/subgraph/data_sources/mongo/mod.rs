use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoDataSourceConfig {
    pub name: String,
    pub uri: String,
    pub db: String,
}
