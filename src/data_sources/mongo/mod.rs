use async_graphql::dynamic::{FieldValue, ValueAccessor};
use bson::{oid::ObjectId, to_document, Document};
use log::{debug, info};
use mongodb::{options::ClientOptions, Client, Database};
use std::str::FromStr;

use crate::{
    configuration::subgraph::{
        data_sources::mongo::MongoDataSourceConfig, entities::ServiceEntity,
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
    pub async fn init_mongo(mongo_data_source_config: &MongoDataSourceConfig) -> DataSource {
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

    pub fn convert_object_id_string_to_object_id(mut filter: Document) -> Document {
        info!("Converting String, `_id`, In Filter to Object ID");
        let object_id_string = filter.get_str("_id").unwrap();
        let object_id = ObjectId::from_str(object_id_string).unwrap();
        filter.insert("_id", object_id);
        filter
    }

    pub fn finalize_filter(filter: Document) -> Document {
        info!("Finalizing Filter");

        let mut filter = to_document(&filter).unwrap();

        if filter.contains_key("_id") {
            info!("Found `_id` In Filter");
            filter = MongoDataSource::convert_object_id_string_to_object_id(filter);
        }

        info!("Filter Finalized");
        debug!("{:?}", filter);

        filter
    }

    pub async fn execute_operation<'a>(
        data_source: &DataSource,
        input: &ValueAccessor<'_>,
        entity: ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<FieldValue<'a>, async_graphql::Error> {
        info!("Executing Mongo Data Source Operation");

        let mut filter = input.deserialize::<Document>().unwrap();

        info!("Found Filter");
        debug!("{:?}", filter);

        filter = MongoDataSource::finalize_filter(filter);

        let db = match data_source {
            DataSource::Mongo(ds) => ds.db.clone(),
            _ => unreachable!(),
        };

        info!("Found DB");
        debug!("{:?}", db);

        let entity_collection_name = entity.data_source.unwrap().collection;
        let collection_name = if entity_collection_name.is_some() {
            entity_collection_name.unwrap()
        } else {
            entity.name
        };

        info!("Found Collection Name");
        debug!("{:?}", collection_name);

        match resolver_type {
            ResolverType::FindOne => {
                let result = services::Services::find_one(db, filter, collection_name).await?;
                Ok(FieldValue::owned_any(result))
            }
            ResolverType::FindMany => {
                let results = services::Services::find_many(db, filter, collection_name).await?;
                Ok(FieldValue::list(
                    results.into_iter().map(|doc| FieldValue::owned_any(doc)),
                ))
            }
            ResolverType::CreateOne => {
                let result = services::Services::create_one(db, filter, collection_name).await?;
                Ok(FieldValue::owned_any(result))
            }
        }
    }
}
