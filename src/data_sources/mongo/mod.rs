use async_graphql::dynamic::FieldValue;
use bson::{oid::ObjectId, to_document, Document};
use log::{debug, error, info};
use mongodb::{options::ClientOptions, Client, Database};
use std::str::FromStr;

use crate::{
    configuration::subgraph::{
        data_sources::mongo::MongoDataSourceConfig, entities::ServiceEntityConfig,
    },
    graphql::schema::ResolverType,
};

use super::DataSource;

pub mod services;

#[derive(Debug, Clone)]
pub struct MongoDataSource {
    pub client: Client,
    pub db: Database,
    pub config: MongoDataSourceConfig,
}

impl MongoDataSource {
    pub async fn init(mongo_data_source_config: &MongoDataSourceConfig) -> DataSource {
        info!("Initializing Mongo");
        let client_options = ClientOptions::parse(&mongo_data_source_config.uri)
            .await
            .expect("Failed to parse mongo client options.");

        let client = Client::with_options(client_options).expect("Failed to create client");
        let db = client.database(&mongo_data_source_config.db);

        info!("Created Mongo Data Source");
        debug!("{:?}", client);
        debug!("{:?}", db);

        DataSource::Mongo(MongoDataSource {
            client,
            db,
            config: mongo_data_source_config.clone(),
        })
    }

    /// If filter contains a string `_id`, convert it to an object id.
    /// Returns the filter with the converted `_id`, if it exists.
    pub fn convert_object_id_string_to_object_id_from_doc(
        mut filter: Document,
    ) -> Result<Document, async_graphql::Error> {
        info!("Converting String, `_id`, In Filter to Object ID");

        if filter.contains_key("_id") {
            let object_id_string = match filter.get("_id") {
                Some(object_id) => match object_id {
                    bson::Bson::String(object_id_string) => object_id_string.clone(),
                    bson::Bson::ObjectId(_) => {
                        return Ok(filter);
                    }
                    _ => {
                        error!("`_id` is not a string or object id");
                        return Err(async_graphql::Error::new(
                            "`_id` is not a string or object id",
                        ));
                    }
                },
                None => {
                    error!("`_id` does not exist in filter");
                    return Err(async_graphql::Error::new("`_id` does not exist in filter"));
                }
            };

            let object_id = ObjectId::from_str(&object_id_string).map_err(|e| {
                error!(
                    "Failed to convert `_id` from string to object id. Error: {}",
                    e
                );
                async_graphql::Error::new(format!(
                    "Failed to convert `_id` from string to object id. Error: {}",
                    e
                ))
            })?;

            filter.insert("_id", object_id);
        }

        Ok(filter)
    }

    pub fn finalize_filter(filter: Document) -> Result<Document, async_graphql::Error> {
        info!("Finalizing Filter");

        let mut filter = to_document(&filter).unwrap();
        filter = MongoDataSource::convert_object_id_string_to_object_id_from_doc(filter)?;

        info!("Filter Finalized");
        debug!("{:?}", filter);

        Ok(filter)
    }

    pub async fn execute_operation<'a>(
        data_source: &DataSource,
        mut input: Document,
        entity: ServiceEntityConfig,
        resolver_type: ResolverType,
    ) -> Result<FieldValue<'a>, async_graphql::Error> {
        debug!("Executing Operation - Mongo Data Source");

        input = MongoDataSource::finalize_filter(input)?;

        let db = match data_source {
            DataSource::Mongo(ds) => ds.db.clone(),
            _ => unreachable!(),
        };

        debug!("Database Found");

        let collection_name = ServiceEntityConfig::get_mongo_collection_name(&entity);

        info!("Found Collection Name");
        debug!("{:?}", collection_name);

        match resolver_type {
            ResolverType::FindOne => {
                let result = services::Services::find_one(db, input, collection_name).await?;
                Ok(FieldValue::owned_any(result))
            }
            ResolverType::FindMany => {
                let results = services::Services::find_many(db, input, collection_name).await?;
                Ok(FieldValue::list(
                    results.into_iter().map(|doc| FieldValue::owned_any(doc)),
                ))
            }
            ResolverType::CreateOne => {
                let result = services::Services::create_one(db, input, collection_name).await?;
                Ok(FieldValue::owned_any(result))
            }
            ResolverType::UpdateOne => {
                let result = services::Services::update_one(db, input, collection_name).await?;
                Ok(FieldValue::owned_any(result))
            }
            ResolverType::UpdateMany => {
                let results = services::Services::update_many(db, input, collection_name).await?;
                Ok(FieldValue::list(
                    results.into_iter().map(|doc| FieldValue::owned_any(doc)),
                ))
            }
            _ => panic!("Invalid resolver type"),
        }
    }
}
