use async_graphql::{Error, ErrorExtensions};
use log::{debug, info};
use mongodb::{bson::Document, Database};

use super::Services;

impl Services {
    pub async fn find_one(
        db: Database,
        filter: Document,
        collection: String,
    ) -> Result<Document, async_graphql::Error> {
        info!("Executing Find One");

        let collection = db.collection(&collection);

        info!("Created Collection");
        debug!("{:?}", collection);

        let document = collection.find_one(filter, None).await;

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
