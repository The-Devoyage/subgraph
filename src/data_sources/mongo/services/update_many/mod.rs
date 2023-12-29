use bson::{doc, to_document, Document};
use log::debug;
use mongodb::{options::UpdateOptions, Database};

use super::Services;

impl Services {
    pub async fn update_many(
        db: Database,
        input: Document,
        collection: String,
    ) -> Result<Vec<Option<Document>>, async_graphql::Error> {
        debug!("Executing Update Many");

        let coll = db.collection::<Document>(&collection);

        let filter = to_document(input.get("query").unwrap()).unwrap();
        let values = to_document(input.get("values").unwrap()).unwrap();

        let options = UpdateOptions::builder().upsert(true).build();

        let update_doc = Services::create_nested_fields(&values);

        coll.update_many(filter, doc! {"$set": update_doc}, options)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        // Find the new docs that match the new values
        let input = doc! {"query": input.get("values").unwrap()};
        let documents = Services::find_many(db, input, collection, vec![]).await;

        documents
    }
}
