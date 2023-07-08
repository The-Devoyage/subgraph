use async_graphql::dynamic::{FieldFuture, ResolverContext};
use log::debug;

use crate::data_sources::DataSources;

use super::ServiceResolver;

mod get_operation_type;
mod get_resolver_input;
mod guard_resolver;

impl ServiceResolver {
    pub fn create_resolver_function(
        &self,
    ) -> Box<(dyn for<'a> Fn(ResolverContext<'a>) -> FieldFuture<'a> + Send + Sync)> {
        debug!("Creating Resolver Function");
        let entity = self.entity.clone();
        let as_field = self.as_field.clone();
        let resolver_type = self.resolver_type.clone();
        let service_guards = self.subgraph_config.service.guards.clone();

        Box::new(move |ctx: ResolverContext| {
            debug!("Resolving Field: {}", ctx.field().name());
            let entity = entity.clone();
            let as_field = as_field.clone();
            let resolver_type = resolver_type.clone();
            let service_guards = service_guards.clone();

            FieldFuture::new(async move {
                let data_sources = ctx.data_unchecked::<DataSources>().clone();
                let input_document =
                    ServiceResolver::get_resolver_input(&ctx, &as_field, &resolver_type)?;

                ServiceResolver::guard_resolver(
                    &ctx,
                    &input_document,
                    &entity,
                    service_guards.clone(),
                    &resolver_type,
                )?;

                let operation_type = ServiceResolver::get_operation_type(&resolver_type, &as_field);

                let results =
                    DataSources::execute(&data_sources, input_document, entity, operation_type)
                        .await?;

                Ok(Some(results))
            })
        })
    }
}
