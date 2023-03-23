use bson::{doc, to_document, Document};
use log::{debug, info};
use mongodb::{
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Database,
};

use crate::data_sources::mongo::MongoDataSource;

use super::Services;

impl Services {
    pub async fn update_one(
        db: Database,
        mut input: Document,
        collection: String,
    ) -> Result<Document, async_graphql::Error> {
        info!("Executing Update One");

        let coll = db.collection::<Document>(&collection);

        debug!("Found Collection: {:?}", collection);

        let mut filter = to_document(input.get("query").unwrap())?;

        filter = MongoDataSource::convert_object_id_string_to_object_id(filter);

        debug!("Filter: {:?}", filter);

        input.remove("query");

        debug!("Input: {:?}", input);

        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .upsert(true)
            .build();

        let document = coll
            .find_one_and_update(filter, doc! {"$set": input }, options)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        debug!("Update One Result: {:?}", document);

        match document {
            Some(document) => Ok(document),
            None => Err(async_graphql::Error::new("No Document Found")),
        }
    }
}
