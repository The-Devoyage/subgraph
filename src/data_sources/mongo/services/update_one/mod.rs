use async_graphql::futures_util::StreamExt;
use bson::{doc, to_document, Document};
use log::{debug, error};
use mongodb::{
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Database,
};

use crate::configuration::subgraph::entities::ServiceEntityConfig;

use super::Services;

impl Services {
    pub async fn update_one(
        db: Database,
        input: Document,
        collection: String,
        entity: &ServiceEntityConfig,
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

        let mut cursor = coll.find(filter.clone(), None).await.map_err(|e| {
            async_graphql::Error::new(format!(
                "Error finding document to update: {}",
                e.to_string()
            ))
        })?;

        let mut primary_keys = Vec::new();

        let primary_key_field = ServiceEntityConfig::get_primary_key_field(&entity)?;
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let primary_key = document.get(primary_key_field.name.clone()).unwrap();
                    primary_keys.push(primary_key.clone());
                }
                Err(e) => {
                    return Err(async_graphql::Error::new(e.to_string()));
                }
            }
        }

        if primary_keys.len() > 1 {
            error!("Multiple documents found for update");
            return Err(async_graphql::Error::new(
                "Multiple documents found for update",
            ));
        }

        if primary_keys.len() == 0 {
            error!("No documents found for update");
            return Err(async_graphql::Error::new("No documents found for update"));
        }

        let primary_key = primary_keys.get(0).unwrap();
        let filter = doc! {primary_key_field.name: primary_key};

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
