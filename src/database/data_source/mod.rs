use mongodb::{options::ClientOptions, Client, Database};

#[derive(Debug)]
pub struct DataSource {
    pub client: Client,
    pub db: Database,
}

impl DataSource {
    pub async fn init() -> DataSource {
        let client_options = ClientOptions::parse("mongodb://sun:sun@127.0.0.1:27017/sun")
            .await
            .expect("Failed to parse mongo client options.");
        let client = Client::with_options(client_options).expect("Failed to create client");
        let db = client.database("sun");

        DataSource { client, db }
    }
}
