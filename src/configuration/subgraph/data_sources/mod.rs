use serde::{Deserialize, Serialize};

pub mod http;
pub mod mongo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceDataSourceConfig {
    Mongo(mongo::MongoDataSourceConfig),
    HTTP(http::HttpDataSourceConfig),
}
