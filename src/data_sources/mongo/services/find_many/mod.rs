use async_graphql::{futures_util::StreamExt, Error, ErrorExtensions};
use bson::Document;
use log::debug;
use mongodb::Database;

use crate::data_sources::mongo::{EagerLoadOptions, MongoDataSource};

use super::Services;

impl Services {
    pub async fn find_many(
        db: Database,
        filter: Document,
        collection: String,
        eager_load_options: Vec<EagerLoadOptions>,
    ) -> Result<Vec<Option<Document>>, async_graphql::Error> {
        let coll = db.collection::<Document>(&collection);

        debug!("Find Many: {:?}", filter);

        let query = match filter.get("query") {
            Some(query) => query,
            None => return Err(Error::new("Query filter not found")),
        };

        let query_doc = match query.as_document() {
            Some(query_doc) => query_doc,
            None => return Err(Error::new("Failed to convert query filter to document.")),
        };

        let filter = Services::create_nested_find_filter(&query_doc);

        let aggregation = MongoDataSource::create_aggregation(&filter, eager_load_options)?;

        let mut cursor = coll.aggregate(aggregation, None).await?;

        let mut documents = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => documents.push(Some(document)),
                Err(_error) => Err(Error::new("Can't find results.")
                    .extend_with(|err, e| e.set("details", err.message.as_str())))?,
            }
        }

        Ok(documents)
    }
}
