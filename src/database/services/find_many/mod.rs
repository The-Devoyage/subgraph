use async_graphql::{futures_util::StreamExt, Error, ErrorExtensions};
use bson::Document;
use mongodb::Database;

use super::Services;

impl Services {
    pub async fn find_many(
        db: Database,
        filter: Document,
        collection: String,
    ) -> Result<Vec<Document>, async_graphql::Error> {
        let coll = db.collection::<Document>(&collection);

        let mut cursor = coll.find(filter, None).await?;

        let mut documents = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => documents.push(document),
                Err(error) => Err(Error::new("Can't find results.")
                    .extend_with(|err, e| e.set("details", err.message.as_str())))?,
            }
        }

        Ok(documents)
    }
}
