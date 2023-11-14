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

        let query_filter = match filter.get("query") {
            Some(query_filter) => query_filter,
            None => return Err(async_graphql::Error::new("Query filter not found")),
        };

        let query_document = match query_filter.as_document() {
            Some(query_document) => query_document,
            None => return Err(async_graphql::Error::new("Query filter not found")),
        };

        let filter = Services::create_nested_find_filter(query_document);

        let document = collection.find_one(filter, None).await?;

        if let Some(user_document) = document {
            Ok(user_document)
        } else {
            Ok(None)
        }
    }
}
