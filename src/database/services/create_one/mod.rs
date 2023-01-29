use async_graphql::{futures_util::StreamExt, Error, ErrorExtensions, Result};
use bson::{doc, to_document, Document};
use log::{debug, info};
use mongodb::Database;

use super::Services;

impl Services {
    pub async fn create_one(
        db: Database,
        new_struct: Document,
    ) -> Result<Document, async_graphql::Error> {
        let coll = db.collection::<Document>("users");

        let document = to_document(&new_struct).unwrap();

        let insert_many_result = coll
            .insert_one(document, None)
            .await
            .expect("Failed to create document.");

        let document = coll
            .find_one(doc! {"_id": insert_many_result.inserted_id }, None)
            .await;

        if let Ok(doc_exists) = document {
            if let Some(user_document) = doc_exists {
                Ok(user_document)
            } else {
                Err(Error::new("User not found")
                    .extend_with(|err, e| e.set("details", err.message.as_str())))
            }
        } else {
            info!("Dastabase Error");
            debug!("{:?}", document);
            Err(Error::new("Database Error"))
        }
    }
}
