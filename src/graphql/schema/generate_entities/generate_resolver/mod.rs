use async_graphql::dynamic::{Field, FieldFuture, FieldValue, TypeRef};
use bson::{to_document, Document};
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

                            info!("Query Object Found");
                            debug!("{:?}", query);

                            let filter = to_document(&query)?;

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

                            info!("Query Object Found.");
                            debug!("{:?}", query);

                            let filter = to_document(&query)?;

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

                            info!("Extracting Args");
                            let new_entity = ctx
                                .args
                                .try_get(&format!("{}_input", ctx.field().name()))?
                                .deserialize::<Document>()?;
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
