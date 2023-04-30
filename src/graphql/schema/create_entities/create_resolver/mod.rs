use async_graphql::dynamic::{Field, FieldFuture, TypeRef};
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

        let resolver = Field::new(
            resolver_config.resolver_name,
            resolver_config.return_type,
            move |ctx| {
                let cloned_entity = cloned_entity.clone();
                FieldFuture::new(async move {
                    let data_sources = ctx.data_unchecked::<DataSources>().clone();
                    let input = ctx.args.try_get(&format!("{}_input", ctx.field().name()))?;

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
