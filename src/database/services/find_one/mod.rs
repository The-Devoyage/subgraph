use async_graphql::{Error, ErrorExtensions};
use log::{debug, info};
use mongodb::{bson::Document, Database};

use super::Services;

impl Services {
    pub async fn find_one(
        db: Database,
        filter: Document,
    ) -> Result<Document, async_graphql::Error> {
        let collection = db.collection("users");

        let document = collection.find_one(filter, None).await;

        if let Ok(user_exists) = document {
            if let Some(user_document) = user_exists {
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
