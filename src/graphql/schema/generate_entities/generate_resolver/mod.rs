use std::str::FromStr;

use async_graphql::dynamic::{Field, FieldFuture, FieldValue, TypeRef};
use bson::{oid::ObjectId, to_document, Document};
use log::{debug, info};

use crate::{
    configuration::subgraph::ServiceEntity,
    database::{data_source::DataSource, services::Services},
    graphql::schema::ResolverConfig,
};

use super::{ResolverType, ServiceSchema};

mod add_entity_type;
mod generate_resolver_input_value;

impl ServiceSchema {
    pub fn convert_object_id_string_to_object_id(mut filter: Document) -> Document {
        let object_id_string = filter.get_str("_id").unwrap();
        let object_id = ObjectId::from_str(object_id_string).unwrap();
        filter.insert("_id", object_id);
        filter
    }

    pub fn add_resolver(mut self, entity: &ServiceEntity, resolver_type: ResolverType) -> Self {
        let resolver_config = match resolver_type {
            ResolverType::FindOne => ResolverConfig {
                resolver_name: format!("get_{}", &entity.name.to_lowercase()),
                return_type: TypeRef::named_nn(&entity.name),
            },
            ResolverType::CreateOne => ResolverConfig {
                resolver_name: format!("create_{}", &entity.name.to_lowercase()),
                return_type: TypeRef::named_nn(&entity.name),
            },
            ResolverType::FindMany => ResolverConfig {
                resolver_name: format!("get_{}s", &entity.name.to_lowercase()),
                return_type: TypeRef::named_nn_list_nn(&entity.name),
            },
        };

        self = self.add_entity_type(&entity);

        info!("Creating Resolver, {}.", resolver_config.resolver_name);
        debug!("{:?}", resolver_config);

        let cloned_entity = entity.clone();

        let field = Field::new(
            resolver_config.resolver_name,
            resolver_config.return_type,
            move |ctx| {
                let cloned_entity = cloned_entity.clone();
                FieldFuture::new(async move {
                    match resolver_type {
                        ResolverType::FindOne => {
                            info!("Executing Find One");
                            let db = ctx.data_unchecked::<DataSource>().db.clone();
                            info!("Getting `query` Input");
                            let query = ctx
                                .args
                                .try_get(&format!("{}_input", ctx.field().name()))?
                                .deserialize::<Document>()?;

                            info!("Find One - Query Object Found");
                            debug!("{:?}", query);

                            let mut filter = to_document(&query)?;

                            if filter.contains_key("_id") {
                                info!("Converting `_id` To Object Id");
                                filter =
                                    ServiceSchema::convert_object_id_string_to_object_id(filter);
                            }

                            info!("Found Filter");
                            debug!("{:?}", filter);

                            let collection_name =
                                cloned_entity.database_config.unwrap().mongo_collection;

                            let document = Services::find_one(
                                db,
                                filter,
                                if collection_name.is_some() {
                                    collection_name.unwrap()
                                } else {
                                    cloned_entity.name
                                },
                            )
                            .await
                            .unwrap();

                            info!("Found Document");
                            debug!("{:?}", document);

                            info!("Returning Result Found");
                            Ok(Some(FieldValue::owned_any(document)))
                        }
                        ResolverType::FindMany => {
                            let db = ctx.data_unchecked::<DataSource>().db.clone();

                            let query = ctx
                                .args
                                .try_get(&format!("{}_input", ctx.field().name()))?
                                .deserialize::<Document>()?;

                            info!("Find Many - Query Object Found.");
                            debug!("{:?}", query);

                            let mut filter = to_document(&query)?;

                            if filter.contains_key("_id") {
                                info!("Converting `_id` To Object Id");
                                filter =
                                    ServiceSchema::convert_object_id_string_to_object_id(filter);
                            }

                            debug!("{:?}", filter);

                            let collection_name =
                                cloned_entity.database_config.unwrap().mongo_collection;

                            let documents = Services::find_many(
                                db,
                                filter,
                                if collection_name.is_some() {
                                    collection_name.unwrap()
                                } else {
                                    cloned_entity.name
                                },
                            )
                            .await;

                            info!("Found Documents");
                            debug!("{:?}", documents);

                            info!("Returning Results Found");
                            Ok(Some(FieldValue::list(
                                documents
                                    .unwrap()
                                    .into_iter()
                                    .map(|doc| FieldValue::owned_any(doc)),
                            )))
                        }
                        ResolverType::CreateOne => {
                            let db = ctx.data_unchecked::<DataSource>().db.clone();

                            let new_entity = ctx
                                .args
                                .try_get(&format!("{}_input", ctx.field().name()))?
                                .deserialize::<Document>()?;

                            info!("Found Args");
                            debug!("{:?}", new_entity);

                            let collection_name =
                                cloned_entity.database_config.unwrap().mongo_collection;

                            let document = to_document(&new_entity)?;

                            let result = Services::create_one(
                                db,
                                document,
                                if collection_name.is_some() {
                                    collection_name.unwrap()
                                } else {
                                    cloned_entity.name
                                },
                            )
                            .await?;

                            info!("Returning Result Found");
                            Ok(Some(FieldValue::owned_any(result)))
                        }
                    }
                })
            },
        );

        info!("Field Created");
        debug!("{:?}", field);

        self = self.generate_resolver_input_value(&entity, field, &resolver_type);

        self
    }
}
