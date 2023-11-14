use bson::{doc, to_document, Document};
use log::debug;
use mongodb::{
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Database,
};

use super::Services;

impl Services {
    pub async fn update_one(
        db: Database,
        input: Document,
        collection: String,
    ) -> Result<Option<Document>, async_graphql::Error> {
        debug!("Executing Update One");

        let coll = db.collection::<Document>(&collection);

        let filter = to_document(input.get("query").unwrap())?;

        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .upsert(true)
            .build();

        let values = to_document(input.get("values").unwrap())?;

        let update_doc = Services::create_nested_fields(&values);

        let document = coll
            .find_one_and_update(filter, doc! {"$set": update_doc}, options)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        debug!("Update One Result: {:?}", document);

        match document {
            Some(document) => Ok(Some(document)),
            None => Err(async_graphql::Error::new("No Document Found")),
        }
    }
}
