use async_graphql::futures_util::StreamExt;
use log::{debug, trace};
use mongodb::{bson::Document, Database};

use crate::data_sources::mongo::{EagerLoadOptions, MongoDataSource};

use super::Services;

impl Services {
    pub async fn find_one(
        db: Database,
        filter: Document,
        collection: String,
        eager_load_options: Vec<EagerLoadOptions>,
    ) -> Result<Option<Document>, async_graphql::Error> {
        debug!("Executing Find One - Mongo Data Source: {:?}", collection);
        trace!("Filter: {:?}", filter);

        let collection = db.collection::<Document>(&collection);

        let query_filter = match filter.get("query") {
            Some(query_filter) => query_filter,
            None => return Err(async_graphql::Error::new("Query filter not found")),
        };

        let query_document = match query_filter.as_document() {
            Some(query_document) => query_document,
            None => return Err(async_graphql::Error::new("Query filter not found")),
        };

        let filter = Services::create_nested_find_filter(query_document);

        let aggregation = MongoDataSource::create_aggregation(&filter, eager_load_options)?;

        // let document = collection.find_one(filter, None).await?;
        let mut cursor = collection.aggregate(aggregation, None).await?;

        while let Some(document) = cursor.next().await {
            if let Ok(document) = document {
                return Ok(Some(document));
            }
        }

        Ok(None)
    }
}
