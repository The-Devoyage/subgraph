use async_graphql::dynamic::{Field, FieldFuture, FieldValue, TypeRef, ValueAccessor};
use log::{debug, info};

use crate::{configuration::subgraph::entities::ServiceEntity, data_sources::DataSources};

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
        };

        debug!("Resolver Type: {:?}", resolver_type);

        resolver_type
    }

    pub async fn resolve_find_one<'a>(
        data_sources: &DataSources,
        input: &ValueAccessor<'_>,
        entity: ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<Option<FieldValue<'a>>, async_graphql::Error> {
        info!("Resolving Find One");
        let result = DataSources::execute(data_sources, &input, entity, resolver_type).await?;

        Ok(Some(result))
    }

    pub async fn resolve_find_many<'a>(
        data_sources: &DataSources,
        input: &ValueAccessor<'_>,
        entity: ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<Option<FieldValue<'a>>, async_graphql::Error> {
        info!("Resolving Find Many");

        let results = DataSources::execute(data_sources, &input, entity, resolver_type).await?;

        Ok(Some(results))
    }

    pub async fn resolve_create_one<'a>(
        data_sources: &DataSources,
        input: &ValueAccessor<'_>,
        entity: ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<Option<FieldValue<'a>>, async_graphql::Error> {
        info!("Resolving Create One");

        let result = DataSources::execute(data_sources, &input, entity, resolver_type).await?;

        Ok(Some(result))
    }

    pub async fn resolve_update_one<'a>(
        data_sources: &DataSources,
        input: &ValueAccessor<'_>,
        entity: ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<Option<FieldValue<'a>>, async_graphql::Error> {
        info!("Resolving Update One");

        let result = DataSources::execute(data_sources, &input, entity, resolver_type).await?;

        Ok(Some(result))
    }

    pub async fn handle_resolve<'a>(
        data_sources: &DataSources,
        input: &ValueAccessor<'_>,
        entity: ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<Option<FieldValue<'a>>, async_graphql::Error> {
        debug!("Resolving Entity");
        match resolver_type {
            ResolverType::FindOne => {
                ServiceSchemaBuilder::resolve_find_one(data_sources, &input, entity, resolver_type)
                    .await
            }
            ResolverType::FindMany => {
                ServiceSchemaBuilder::resolve_find_many(data_sources, &input, entity, resolver_type)
                    .await
            }
            ResolverType::CreateOne => {
                ServiceSchemaBuilder::resolve_create_one(
                    data_sources,
                    &input,
                    entity,
                    resolver_type,
                )
                .await
            }
            ResolverType::UpdateOne => {
                ServiceSchemaBuilder::resolve_update_one(
                    data_sources,
                    &input,
                    entity,
                    resolver_type,
                )
                .await
            }
        }
    }

    pub fn create_resolver(mut self, entity: &ServiceEntity, resolver_type: ResolverType) -> Self {
        info!("Creating Resolver");

        let resolver_config = ServiceSchemaBuilder::create_resolver_config(entity, resolver_type);
        let cloned_entity = entity.clone();

        let resolver = Field::new(
            resolver_config.resolver_name,
            resolver_config.return_type,
            move |ctx| {
                let cloned_entity = cloned_entity.clone();
                FieldFuture::new(async move {
                    let data_sources = ctx.data_unchecked::<DataSources>().clone();
                    let input = ctx.args.try_get(&format!("{}_input", ctx.field().name()))?;

                    ServiceSchemaBuilder::handle_resolve(
                        &data_sources,
                        &input,
                        cloned_entity,
                        resolver_type,
                    )
                    .await
                })
            },
        );

        debug!("Resolver: {:?}", resolver);

        self = self.create_resolver_input_value(&entity, resolver, &resolver_type);
        self
    }
}
