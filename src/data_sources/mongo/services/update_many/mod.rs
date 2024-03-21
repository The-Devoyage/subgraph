use async_graphql::futures_util::StreamExt;
use bson::{doc, to_document, Document};
use log::debug;
use mongodb::Database;

use crate::configuration::subgraph::entities::ServiceEntityConfig;

use super::Services;

impl Services {
    pub async fn update_many(
        db: Database,
        input: Document,
        collection: String,
        entity: &ServiceEntityConfig,
    ) -> Result<Vec<Option<Document>>, async_graphql::Error> {
        debug!("Executing Update Many");

        let coll = db.collection::<Document>(&collection);

        let filter = to_document(input.get("query").unwrap()).unwrap();
        let values = to_document(input.get("values").unwrap()).unwrap();

        let update_doc = Services::create_nested_fields(&values);

        let mut cursor = coll
            .find(filter.clone(), None)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut primary_keys = vec![];

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let primary_key_field = ServiceEntityConfig::get_primary_key_field(&entity)?;
                    let primary_key = document.get(primary_key_field.name).unwrap();
                    primary_keys.push(primary_key.clone());
                }
                Err(e) => {
                    return Err(async_graphql::Error::new(e.to_string()));
                }
            }
        }

        let ids_doc = doc! {"_id": {"$in": primary_keys}};

        coll.update_many(ids_doc.clone(), doc! {"$set": update_doc}, None)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut cursor = coll
            .find(ids_doc.clone(), None)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut documents = vec![];
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    documents.push(Some(document));
                }
                Err(e) => {
                    return Err(async_graphql::Error::new(e.to_string()));
                }
            }
        }

        Ok(documents)
    }
}
