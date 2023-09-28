use log::debug;
use mongodb::{bson::Document, Database};

use super::Services;

impl Services {
    pub async fn find_one(
        db: Database,
        filter: Document,
        collection: String,
    ) -> Result<Option<Document>, async_graphql::Error> {
        debug!("Executing Find One - Mongo Data Source: {:?}", collection);

        let collection = db.collection(&collection);

        let filter = Services::create_nested_find_filter(&filter);

        debug!("Created Filter: {:?}", filter);

        let document = collection.find_one(filter, None).await?;

        if let Some(user_document) = document {
            Ok(user_document)
        } else {
            Ok(None)
        }
    }
}
