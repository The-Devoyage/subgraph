use async_graphql::dynamic::FieldValue;
use bson::{oid::ObjectId, Document};
use log::{debug, error, info};
use mongodb::{options::ClientOptions, Client, Database};
use std::str::FromStr;

use crate::{
    configuration::subgraph::{
        data_sources::mongo::MongoDataSourceConfig,
        entities::{ScalarOptions, ServiceEntityConfig},
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
        debug!("Initializing Mongo");
        let client_options = ClientOptions::parse(&mongo_data_source_config.uri)
            .await
            .expect("Failed to parse mongo client options.");

        let client = Client::with_options(client_options).expect("Failed to create client");
        let db = client.database(&mongo_data_source_config.db);

        debug!("Created Mongo Data Source");
        debug!("{:?}", client);
        debug!("{:?}", db);

        DataSource::Mongo(MongoDataSource {
            client,
            db,
            config: mongo_data_source_config.clone(),
        })
    }

    /// Recursively convert all string object ids to object ids.
    /// Uses field definitions to determine if a field is an object id.
    pub fn convert_object_id_string_to_object_id_from_doc(
        filter: Document,
        entity: &ServiceEntityConfig,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Serialize String Object IDs to Object IDs");

        let mut converted = filter.clone();

        let mut key = "".to_string();
        for (k, value) in filter.iter() {
            debug!("Key: {}, Value: {}", k, value);
            if k == "query" || k == "values" || k == "OR" || k == "AND" {
                let document = match value.as_document() {
                    Some(document) => document,
                    None => {
                        error!("Failed to get document from value");
                        return Err(async_graphql::Error::from(
                            "Failed to get document from value",
                        ));
                    }
                };
                let nested_converted =
                    match MongoDataSource::convert_object_id_string_to_object_id_from_doc(
                        document.clone(),
                        entity,
                    ) {
                        Ok(nested) => nested,
                        Err(e) => {
                            error!(
                                "Failed to convert object id string to object id. Error: {:?}",
                                e
                            );
                            return Err(e);
                        }
                    };
                converted.insert(k.clone(), nested_converted);
                continue;
            }

            let fields = match ServiceEntityConfig::get_fields_recursive(entity, &k) {
                Ok(fields) => fields,
                Err(_) => {
                    continue;
                }
            };

            // if the last field is a scalar of object id, convert the value to an object id.
            if let Some(last_element) = fields.last() {
                match last_element.scalar {
                    ScalarOptions::ObjectID => {
                        // if the value is a string, convert it to an object id.
                        if let bson::Bson::String(object_id_string) = value {
                            let object_id = ObjectId::from_str(&object_id_string).map_err(|e| {
                                error!("Failed to convert string to object id. Error: {:?}", e);
                                async_graphql::Error::new(format!(
                                    "Failed to convert string to object id. Error: {:?}",
                                    e
                                ))
                            })?;

                            //update the cooresponding value in converted
                            converted.insert(k.clone(), object_id);
                        }
                    }
                    ScalarOptions::Object => {
                        let separator = if key.is_empty() { "" } else { "." };
                        let separated = format!("{}{}", separator, k);
                        key.push_str(&separated);
                        let nested_converted =
                            match MongoDataSource::convert_object_id_string_to_object_id_from_doc(
                                value.as_document().unwrap().clone(),
                                entity,
                            ) {
                                Ok(nested) => nested,
                                Err(e) => {
                                    error!(
                                        "Failed to convert object id string to object id. Error: {:?}",
                                        e
                                    );
                                    return Err(e);
                                }
                            };
                        converted.insert(key.clone(), nested_converted);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        Ok(converted)
    }

    pub fn finalize_input(
        filter: Document,
        entity: &ServiceEntityConfig,
    ) -> Result<Document, async_graphql::Error> {
        info!("Finalizing Input Filters");

        let mut finalized = filter.clone();

        for (key, value) in filter.iter() {
            if key == "query" {
                let query = value.as_document().unwrap();
                let query_finalized = MongoDataSource::finalize_input(query.clone(), entity)?;
                finalized.insert(key.clone(), query_finalized);
            }

            // Values is an object without filters, so we can just return it.
            if key == "values" {
                finalized.insert(key.clone(), value.clone());
            }

            if key == "AND" || key == "OR" {
                debug!("AND/OR Filter");
                let mut and_or_filters = Vec::new();
                let filters = value.as_array().unwrap();
                for filter in filters {
                    let filter = filter.as_document().unwrap();
                    let filter_finalized = MongoDataSource::finalize_input(filter.clone(), entity)?;
                    and_or_filters.push(filter_finalized);
                }
                finalized.remove(key);
                let key = if key == "AND" { "$and" } else { "$or" };
                debug!("Key: {}", key);
                finalized.insert(key, and_or_filters);
                debug!("Finalized: {:?}", finalized);
            }
        }

        finalized =
            MongoDataSource::convert_object_id_string_to_object_id_from_doc(finalized, entity)?;

        info!("Filter Finalized");
        debug!("{:?}", finalized);

        Ok(finalized)
    }

    pub async fn execute_operation<'a>(
        data_source: &DataSource,
        mut input: Document,
        entity: ServiceEntityConfig,
        resolver_type: ResolverType,
    ) -> Result<FieldValue<'a>, async_graphql::Error> {
        debug!("Executing Operation - Mongo Data Source");

        input = MongoDataSource::finalize_input(input, &entity)?;

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
