use mongodb::{options::ClientOptions, Client, Database};

use crate::configuration::subgraph::data_sources::MongoDataSourceConfig;

use super::DataSource;

pub mod services;

pub struct MongoDataSource {
    pub client: Client,
    pub db: Database,
    pub config: MongoDataSourceConfig,
}

impl MongoDataSource {
    pub async fn init_mongo(mongo_data_source_config: &MongoDataSourceConfig) -> DataSource {
        let client_options = ClientOptions::parse(&mongo_data_source_config.uri)
            .await
            .expect("Failed to parse mongo client options.");
        let client = Client::with_options(client_options).expect("Failed to create client");
        let db = client.database(&mongo_data_source_config.db);

        DataSource::Mongo(MongoDataSource {
            client,
            db,
            config: mongo_data_source_config.clone(),
        })
    }
}
