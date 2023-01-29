use mongodb::{options::ClientOptions, Client, Database};

use crate::configuration::subgraph::SubGraphConfig;

#[derive(Debug)]
pub struct DataSource {
    pub client: Client,
    pub db: Database,
}

impl DataSource {
    pub async fn init(subgraph_config: &SubGraphConfig) -> DataSource {
        let client_options = ClientOptions::parse(
            subgraph_config
                .service
                .database_config
                .as_ref()
                .unwrap()
                .mongo_uri
                .as_ref()
                .unwrap(),
        )
        .await
        .expect("Failed to parse mongo client options.");

        let client = Client::with_options(client_options).expect("Failed to create client");

        let db = client.database(
            &subgraph_config
                .service
                .database_config
                .as_ref()
                .unwrap()
                .mongo_db
                .as_ref()
                .unwrap(),
        );

        DataSource { client, db }
    }
}
