use async_graphql::futures_util::StreamExt;
use log::{debug, trace};
use mongodb::{
    bson::{doc, Document},
    Database,
};

use crate::{
    data_sources::mongo::{EagerLoadOptions, MongoDataSource},
    graphql::schema::create_options_input::OptionsInput,
};

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

        if let Some(opts_doc) = filter.get("opts") {
            let opts = match opts_doc.as_document() {
                Some(opts) => opts.clone(),
                None => {
                    let default_opts = doc! {
                        "per_page": 10,
                        "page": 1,
                        "sort": [
                            {
                                "field": "_id",
                                "direction": "Asc"
                            }
                        ]
                    };
                    default_opts
                }
            };
            // Serialize the opts document to a OptionsInput
            let opts: OptionsInput = bson::from_bson(bson::Bson::Document(opts))?;

            let aggregation =
                MongoDataSource::create_aggregation(&filter, eager_load_options, Some(opts))?;

            // let document = collection.find_one(filter, None).await?;
            let mut cursor = collection.aggregate(aggregation, None).await?;

            while let Some(document) = cursor.next().await {
                if let Ok(document) = document {
                    return Ok(Some(document));
                }
            }
        } else {
            let document = collection.find_one(filter, None).await?;

            return Ok(document);
        }

        Ok(None)
    }
}
