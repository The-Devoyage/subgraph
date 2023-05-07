use async_graphql::dynamic::{Field, FieldFuture, TypeRef};
use bson::Document;
use http::HeaderMap;
use log::{debug, info};

use crate::{
    configuration::subgraph::{entities::ServiceEntity, guard::Guard},
    data_sources::DataSources,
};

use super::{ResolverType, ServiceSchemaBuilder};

mod create_resolver_input_value;

#[derive(Debug)]
pub struct ResolverConfig {
    resolver_name: String,
    return_type: TypeRef,
}

impl ServiceSchemaBuilder {
    pub fn create_resolver_config(
        entity: &ServiceEntity,
        resolver_type: ResolverType,
    ) -> ResolverConfig {
        info!("Creating Resolver Config");

        let resolver_type = match resolver_type {
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
            ResolverType::UpdateOne => ResolverConfig {
                resolver_name: format!("update_{}", &entity.name.to_lowercase()),
                return_type: TypeRef::named_nn(&entity.name),
            },
            ResolverType::UpdateMany => ResolverConfig {
                resolver_name: format!("update_{}s", &entity.name.to_lowercase()),
                return_type: TypeRef::named_nn_list_nn(&entity.name),
            },
        };

        debug!("Resolver Type: {:?}", resolver_type);

        resolver_type
    }

    pub fn create_resolver(mut self, entity: &ServiceEntity, resolver_type: ResolverType) -> Self {
        info!("Creating Resolver");

        let resolver_config = ServiceSchemaBuilder::create_resolver_config(entity, resolver_type);
        let cloned_entity = entity.clone();
        let service_guards = self.subgraph_config.service.guards.clone();
        let entity_guards = entity.guards.clone();
        let resolver = ServiceEntity::get_resolver(&entity, resolver_type);
        let resolver_guards = if resolver.is_some() {
            resolver.unwrap().guards
        } else {
            None
        };
        let field_guards = entity
            .fields
            .iter()
            .flat_map(|field| {
                if field.guards.is_some() {
                    field.guards.clone().unwrap().clone()
                } else {
                    Vec::new()
                }
            })
            .collect::<Vec<Guard>>();

        let resolver = Field::new(
            resolver_config.resolver_name,
            resolver_config.return_type,
            move |ctx| {
                let cloned_entity = cloned_entity.clone();
                let service_guards = service_guards.clone();
                let entity_guards = entity_guards.clone();
                let resolver_guards = resolver_guards.clone();
                let field_guards = field_guards.clone();

                FieldFuture::new(async move {
                    let data_sources = ctx.data_unchecked::<DataSources>().clone();
                    let input = ctx.args.try_get(&format!("{}_input", ctx.field().name()))?;
                    let headers = ctx.data_unchecked::<HeaderMap>().clone();
                    let input_document = &input.deserialize::<Document>().unwrap();
                    let guard_context = Guard::create_guard_context(
                        headers,
                        input_document.clone(),
                        cloned_entity.clone(),
                    )?;

                    if service_guards.is_some() {
                        Guard::check(&service_guards.unwrap(), &guard_context)?;
                    }

                    if resolver_guards.is_some() {
                        Guard::check(&resolver_guards.unwrap(), &guard_context)?;
                    }

                    if entity_guards.is_some() {
                        Guard::check(&entity_guards.unwrap(), &guard_context)?;
                    }

                    if field_guards.len() > 0 {
                        Guard::check(&field_guards, &guard_context)?;
                    }

                    let results =
                        DataSources::execute(&data_sources, &input, cloned_entity, resolver_type)
                            .await?;

                    Ok(Some(results))
                })
            },
        );

        debug!("Resolver: {:?}", resolver);

        self = self.create_resolver_input_value(&entity, resolver, &resolver_type);
        self
    }
}
