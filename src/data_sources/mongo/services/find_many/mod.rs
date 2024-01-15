use async_graphql::{futures_util::StreamExt, Error, ErrorExtensions};
use bson::{doc, Document};
use log::{debug, error, trace};
use mongodb::Database;

use crate::{
    data_sources::mongo::{EagerLoadOptions, MongoDataSource},
    graphql::schema::create_options_input::OptionsInput,
};

use super::Services;

impl Services {
    pub async fn find_many(
        db: Database,
        filter: Document,
        collection: String,
        eager_load_options: Vec<EagerLoadOptions>,
    ) -> Result<(Vec<Option<Document>>, i64), async_graphql::Error> {
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

        let nested_find_filter = Services::create_nested_find_filter(&query_doc);

        let opts_bson = match filter.get("opts") {
            Some(opts_doc) => opts_doc.clone(),
            None => {
                trace!("No opts found, using default opts.");
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
                bson::to_bson(&default_opts).unwrap()
            }
        };

        // Serialize the opts document to a OptionsInput
        let opts: Option<OptionsInput> = match bson::from_bson(opts_bson) {
            Ok(opts) => opts,
            Err(error) => {
                error!("Failed to convert opts to OptionsInput: {:?}", error);
                return Err(Error::new("Failed to convert opts to OptionsInput."));
            }
        };

        let aggregation =
            MongoDataSource::create_aggregation(&nested_find_filter, eager_load_options, opts)?;

        let mut cursor = coll.aggregate(aggregation, None).await?;

        trace!("Find Many Cursor: {:?}", cursor);

        let mut result_doc = None;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    result_doc = Some(document);
                    break;
                }
                Err(_error) => Err(Error::new("Can't find results.")
                    .extend_with(|err, e| e.set("details", err.message.as_str())))?,
            }
        }

        trace!("Find Many Result: {:?}", result_doc);

        if result_doc.is_none() {
            return Err(Error::new("Can't find matched results."));
        }
        let results = match result_doc.as_ref().unwrap().get("documents") {
            Some(documents) => {
                let documents = match documents.as_array() {
                    Some(documents) => documents.clone(),
                    None => Vec::<bson::Bson>::new(),
                };
                let documents = documents
                    .clone()
                    .into_iter()
                    .map(|doc| Some(doc.as_document().unwrap().clone()))
                    .collect::<Vec<Option<Document>>>();
                documents
            }
            None => Vec::<Option<Document>>::new(),
        };

        let total_count = if let Some(total_count_docs) = result_doc
            .as_ref()
            .unwrap()
            .get("total_count")
            .unwrap()
            .as_array()
        {
            if total_count_docs.len() > 0 {
                let total_count_doc = total_count_docs.get(0).unwrap().as_document().unwrap();
                let total_count = total_count_doc
                    .get("total_count")
                    .unwrap_or(&bson::Bson::Int32(0))
                    .as_i32()
                    .unwrap();
                total_count
            } else {
                0
            }
        } else {
            0
        };

        Ok((results, total_count as i64))
    }
}
