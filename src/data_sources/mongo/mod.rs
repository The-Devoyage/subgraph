use async_graphql::dynamic::FieldValue;
use bson::{doc, oid::ObjectId, Document};
use log::{debug, error, trace};
use mongodb::{options::ClientOptions, Client, Database};
use std::str::FromStr;

use crate::{
    configuration::subgraph::{
        data_sources::mongo::MongoDataSourceConfig,
        entities::{
            service_entity_field::ServiceEntityFieldConfig, ScalarOptions, ServiceEntityConfig,
        },
        SubGraphConfig,
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

#[derive(Debug, Clone)]
pub struct EagerLoadOptions {
    pub from: String,
    pub local_field: String,
    pub foreign_field: String,
    pub as_field: String,
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
        subgraph_config: &SubGraphConfig,
    ) -> Result<(Document, Vec<EagerLoadOptions>), async_graphql::Error> {
        debug!("Serialize String Object IDs to Object IDs");

        let mut converted = filter.clone();
        let mut combined_eager_options = vec![];

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
                let (nested_converted, nested_eager_load_options) =
                    match MongoDataSource::convert_object_id_string_to_object_id_from_doc(
                        document.clone(),
                        entity,
                        subgraph_config,
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
                combined_eager_options.extend(nested_eager_load_options);
                continue;
            }

            let fields = match ServiceEntityConfig::get_fields_recursive(entity, &k) {
                Ok(fields) => fields,
                Err(_) => {
                    continue;
                }
            };

            // Since searching by a single key above, the last field is guaranteed to be the field we are looking for.
            if let Some(field) = fields.last() {
                match field.scalar {
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
                        let (nested_converted, nested_eager_load_options) =
                            match MongoDataSource::convert_object_id_string_to_object_id_from_doc(
                                value.as_document().unwrap().clone(),
                                entity,
                                subgraph_config,
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
                        combined_eager_options.extend(nested_eager_load_options);
                    }
                    _ => (),
                }

                // Handle eager loaded fields
                let eager_load_options =
                    match MongoDataSource::handle_eager_fields(field, entity, subgraph_config) {
                        Ok(eager_load_options) => eager_load_options,
                        Err(e) => {
                            error!("Failed to handle eager fields. Error: {:?}", e);
                            return Err(e);
                        }
                    };
                if let Some(eager_load_option) = eager_load_options {
                    // replace the key with the eager load key.
                    converted.remove(&k);
                    converted.insert(eager_load_option.as_field.clone(), value);
                    combined_eager_options.push(eager_load_option);
                }
            }
        }

        trace!("Converted: {:?}", converted);
        Ok((converted, combined_eager_options))
    }

    pub fn handle_eager_fields(
        field: &ServiceEntityFieldConfig,
        entity: &ServiceEntityConfig,
        subgraph_config: &SubGraphConfig,
    ) -> Result<Option<EagerLoadOptions>, async_graphql::Error> {
        debug!("Handle eager fields");
        // Since searching by a single key above, the last field is guaranteed to be the field we are looking for.
        if field.eager.is_none() {
            return Ok(None);
        }

        // Get the name of the field to eager load - this can help to get the correct collection name.
        // the join_on value will be the name of the parent entity
        let join_on = if let Some(join_on) = field.join_on.clone() {
            join_on
        } else {
            return Err(async_graphql::Error::new(format!(
                "Eager load failed. Failed to get join_on for field: {}",
                field.name
            )));
        };

        let as_type = if let Some(as_type) = field.as_type.clone() {
            as_type
        } else {
            return Err(async_graphql::Error::new(format!(
                "Eager load failed. Failed to get `as_type` for field: {}",
                field.name
            )));
        };

        // Get the entity to reference the correct collection name.
        let eager_entity = match subgraph_config.clone().get_entity(&as_type) {
            Some(entity) => entity,
            None => {
                return Err(async_graphql::Error::new(format!(
                    "Eager load failed. Failed to get entity for key: {}",
                    join_on
                )));
            }
        };

        // Check if it has a collection name.
        let collection_name = if let Some(ds) = eager_entity.data_source {
            if ds.collection.is_some() {
                ds.collection.unwrap().clone()
            } else {
                eager_entity.name.clone()
            }
        } else {
            eager_entity.name.clone()
        };

        let join_from = if let Some(join_from) = field.join_from.clone() {
            join_from
        } else {
            field.name.clone()
        };

        //replace the key with the eager loaded key.
        let eager_key = format!("{}_{}_{}", entity.name, field.name, join_on);

        let eager_load_options = EagerLoadOptions {
            from: collection_name,
            local_field: join_from,
            foreign_field: join_on,
            as_field: eager_key.clone(),
        };

        trace!("Eager load options: {:?}", eager_load_options);
        Ok(Some(eager_load_options))
    }

    pub fn finalize_input(
        filter: Document,
        entity: &ServiceEntityConfig,
        subgraph_config: &SubGraphConfig,
    ) -> Result<(Document, Vec<EagerLoadOptions>), async_graphql::Error> {
        debug!("Finalizing Input Filters");

        let mut finalized = filter.clone();
        let mut eager_filters = Vec::new();

        for (key, value) in filter.iter() {
            if key == "query" {
                let query = value.as_document().unwrap();
                let (query_finalized, eager_opts) =
                    MongoDataSource::finalize_input(query.clone(), entity, subgraph_config)?;
                finalized.insert(key.clone(), query_finalized);
                eager_filters.extend(eager_opts);
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
                    let (filter_finalized, eager_opts) =
                        MongoDataSource::finalize_input(filter.clone(), entity, subgraph_config)?;
                    and_or_filters.push(filter_finalized);
                    eager_filters.extend(eager_opts);
                }
                finalized.remove(key);
                let key = if key == "AND" { "$and" } else { "$or" };
                trace!("Key: {}", key);
                finalized.insert(key, and_or_filters);
                trace!("Finalized: {:?}", finalized);
            }
        }

        let eager_load_options;
        (finalized, eager_load_options) =
            MongoDataSource::convert_object_id_string_to_object_id_from_doc(
                finalized,
                entity,
                subgraph_config,
            )?;
        eager_filters.extend(eager_load_options);

        trace!("Filter Finalized");
        trace!("Finalized: {:?}", finalized);
        trace!("Total Eager Load Options: {:?}", eager_filters);

        Ok((finalized, eager_filters))
    }

    pub fn create_aggregation(
        query_doc: &Document,
        eager_load_options: Vec<EagerLoadOptions>,
    ) -> Result<Vec<Document>, async_graphql::Error> {
        debug!("Creating Aggregation");
        let mut pipeline = Vec::new();
        for eager_load_option in eager_load_options {
            let lookup = doc! {
                "$lookup": {
                    "from": eager_load_option.from,
                    "localField": eager_load_option.local_field,
                    "foreignField": eager_load_option.foreign_field,
                    "as": eager_load_option.as_field.clone(),
                }
            };
            pipeline.push(lookup);
            let unwind = doc! {
                "$unwind": {
                    "path": format!("${}", eager_load_option.as_field),
                    "preserveNullAndEmptyArrays": true,
                }
            };
            pipeline.push(unwind);
        }

        let match_doc = doc! {
            "$match": query_doc
        };

        pipeline.push(match_doc);

        trace!("Pipeline: {:?}", pipeline);
        Ok(pipeline)
    }

    pub async fn execute_operation<'a>(
        data_source: &DataSource,
        mut input: Document,
        entity: ServiceEntityConfig,
        resolver_type: ResolverType,
        subgraph_config: &SubGraphConfig,
    ) -> Result<Option<FieldValue<'a>>, async_graphql::Error> {
        debug!("Executing Operation - Mongo Data Source");

        let eager_load_options;
        (input, eager_load_options) =
            MongoDataSource::finalize_input(input, &entity, subgraph_config)?;

        let db = match data_source {
            DataSource::Mongo(ds) => ds.db.clone(),
            _ => unreachable!(),
        };

        debug!("Database Found");

        let collection_name = ServiceEntityConfig::get_mongo_collection_name(&entity);

        debug!("Found Collection Name");
        trace!("{:?}", collection_name);

        match resolver_type {
            ResolverType::FindOne => {
                let result =
                    services::Services::find_one(db, input, collection_name, eager_load_options)
                        .await?;
                if result.is_none() {
                    return Ok(FieldValue::NONE);
                }
                Ok(Some(FieldValue::owned_any(result)))
            }
            ResolverType::FindMany => {
                let results = services::Services::find_many(db, input, collection_name).await?;
                Ok(Some(FieldValue::list(
                    results.into_iter().map(|doc| FieldValue::owned_any(doc)),
                )))
            }
            ResolverType::CreateOne => {
                let result = services::Services::create_one(db, input, collection_name).await?;
                if result.is_none() {
                    return Ok(FieldValue::NONE);
                }
                Ok(Some(FieldValue::owned_any(result)))
            }
            ResolverType::UpdateOne => {
                let result = services::Services::update_one(db, input, collection_name).await?;
                if result.is_none() {
                    return Ok(FieldValue::NONE);
                }
                Ok(Some(FieldValue::owned_any(result)))
            }
            ResolverType::UpdateMany => {
                let results = services::Services::update_many(db, input, collection_name).await?;
                Ok(Some(FieldValue::list(
                    results.into_iter().map(|doc| FieldValue::owned_any(doc)),
                )))
            }
            _ => panic!("Invalid resolver type"),
        }
    }
}
