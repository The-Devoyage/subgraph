use async_graphql::{Error, ErrorExtensions};
use log::debug;
use mongodb::{bson::Document, Database};

use super::Services;

impl Services {
    pub async fn find_one(
        db: Database,
        filter: Document,
        collection: String,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Executing Find One - Mongo Data Source: {:?}", collection);

        let collection = db.collection(&collection);

        let filter = Services::create_nested_find_filter(&filter);

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
