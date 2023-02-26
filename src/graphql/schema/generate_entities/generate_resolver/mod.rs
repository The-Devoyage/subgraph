use async_graphql::dynamic::{Field, FieldFuture, TypeRef};
use log::{debug, info};

use crate::{
    configuration::subgraph::entities::ServiceEntity, data_sources::DataSources,
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

                            let data_sources = ctx.data_unchecked::<DataSources>().clone();

                            info!("Found Data Sources");
                            debug!("{:?}", data_sources);

                            let input =
                                ctx.args.try_get(&format!("{}_input", ctx.field().name()))?;

                            info!("Found Input");

                            let result = DataSources::execute(
                                data_sources,
                                &input,
                                cloned_entity,
                                resolver_type,
                            )
                            .await;

                            info!("Found Results");

                            Ok(Some(result))
                        }
                        ResolverType::FindMany => {
                            info!("Executing Find Many");

                            let data_sources = ctx.data_unchecked::<DataSources>().clone();

                            info!("Found Data Sources");
                            debug!("{:?}", data_sources);

                            let input =
                                ctx.args.try_get(&format!("{}_input", ctx.field().name()))?;

                            let results = DataSources::execute(
                                data_sources,
                                &input,
                                cloned_entity,
                                resolver_type,
                            )
                            .await;

                            Ok(Some(results))
                        }
                        ResolverType::CreateOne => {
                            info!("Executing Create One");

                            let data_sources = ctx.data_unchecked::<DataSources>().clone();

                            info!("Found Data Sources");
                            debug!("{:?}", data_sources);

                            let input =
                                ctx.args.try_get(&format!("{}_input", ctx.field().name()))?;

                            let result = DataSources::execute(
                                data_sources,
                                &input,
                                cloned_entity,
                                resolver_type,
                            )
                            .await;

                            Ok(Some(result))
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
