use async_graphql::{futures_util::StreamExt, Error, ErrorExtensions, Result};
use bson::{doc, to_document, Document};
use mongodb::Database;

use super::Services;

impl Services {
    pub async fn create(
        db: Database,
        new_structs: Vec<Document>,
    ) -> Result<Vec<Document>, async_graphql::Error> {
        let coll = db.collection::<Document>("users");

        let mut documents = vec![];

        for s in new_structs {
            documents.push(to_document(&s).unwrap())
        }

        let insert_many_result = coll
            .insert_many(documents, None)
            .await
            .expect("Failed to create documents.");

        let mut cursor = coll
            .find(
                doc! {"_id": Vec::from_iter(insert_many_result.inserted_ids.values())},
                None,
            )
            .await?;

        let mut documents = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    documents.push(document);
                }
                Err(error) => Err(Error::new("Can't find new documents.")
                    .extend_with(|err, e| e.set("details", err.message.as_str())))?,
            }
        }

        Ok(documents)
    }
}
