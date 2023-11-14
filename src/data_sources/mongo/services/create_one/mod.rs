use async_graphql::{Error, ErrorExtensions, Result};
use bson::{doc, to_document, Document};
use log::{debug, error, info};
use mongodb::Database;

use super::Services;

impl Services {
    pub async fn create_one(
        db: Database,
        input: Document,
        collection: String,
    ) -> Result<Option<Document>, async_graphql::Error> {
        info!("Executing Create One");

        let coll = db.collection::<Document>(&collection);

        let values_input = match input.get("values") {
            Some(values_input) => values_input,
            None => return Err(Error::new("Values input not found")),
        };

        let values_input_doc = match values_input.as_document() {
            Some(values_input_doc) => values_input_doc,
            None => return Err(Error::new("Values input not found")),
        };

        let insert_one_result = coll
            .insert_one(values_input_doc, None)
            .await
            .expect("Failed to create document.");

        let document = coll
            .find_one(doc! {"_id": insert_one_result.inserted_id }, None)
            .await;

        if let Ok(doc_exists) = document {
            if let Some(user_document) = doc_exists {
                Ok(Some(user_document))
            } else {
                Err(Error::new("Document not found")
                    .extend_with(|err, e| e.set("details", err.message.as_str())))
            }
        } else {
            info!("Dastabase Error");
            debug!("{:?}", document);
            Err(Error::new("Database Error"))
        }
    }
}
