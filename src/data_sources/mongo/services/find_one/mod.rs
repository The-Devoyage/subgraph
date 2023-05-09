use async_graphql::{Error, ErrorExtensions};
use log::{debug, info};
use mongodb::{bson::Document, Database};

use super::Services;

impl Services {
    pub async fn find_one(
        db: Database,
        filter: Document,
        collection: String,
    ) -> Result<Document, async_graphql::Error> {
        info!("Executing Find One - Mongo Data Source");

        let collection = db.collection(&collection);

        debug!("Created Collection: {:?}", collection);

        let filter = Services::create_nested_fields(&filter);

        debug!("Created Filter: {:?}", filter);

        let document = collection.find_one(filter, None).await?;

        if let Some(user_document) = document {
            Ok(user_document)
        } else {
            Err(Error::new("Document not found")
                .extend_with(|err, e| e.set("details", err.message.as_str())))
        }
    }
}
