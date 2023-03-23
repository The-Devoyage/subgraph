use async_graphql::{Error, ErrorExtensions, Result};
use bson::{doc, to_document, Document};
use log::{debug, info};
use mongodb::Database;

use super::Services;

impl Services {
    pub async fn create_one(
        db: Database,
        new_struct: Document,
        collection: String,
    ) -> Result<Document, async_graphql::Error> {
        info!("Executing Create One");

        let coll = db.collection::<Document>(&collection);

        info!("Found Collection");
        debug!("{:?}", coll);

        let document = to_document(&new_struct).unwrap();

        info!("Converted New Document");
        debug!("{:?}", document);

        let insert_one_result = coll
            .insert_one(document, None)
            .await
            .expect("Failed to create document.");

        info!("Document Inserted");
        debug!("{:?}", insert_one_result);

        let document = coll
            .find_one(doc! {"_id": insert_one_result.inserted_id }, None)
            .await;

        info!("Found Newly Inserted Document");
        debug!("{:?}", document);

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
