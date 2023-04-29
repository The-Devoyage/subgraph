use serde::{Deserialize, Serialize};

pub mod http;
pub mod mongo;
pub mod sql;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceDataSourceConfig {
    Mongo(mongo::MongoDataSourceConfig),
    HTTP(http::HttpDataSourceConfig),
    SQL(sql::SqlDataSourceConfig),
}
