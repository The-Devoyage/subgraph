use async_graphql::dynamic::{FieldFuture, ResolverContext};
use http::HeaderMap;
use log::debug;

use crate::data_sources::DataSources;

use super::ServiceResolver;

mod get_operation_type;
mod get_resolver_input;
mod get_token_data;
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
        let is_auth = self.subgraph_config.service.auth.is_some();

        Box::new(move |ctx: ResolverContext| {
            debug!("Resolving Field: {}", ctx.field().name());
            let entity = entity.clone();
            let as_field = as_field.clone();
            let resolver_type = resolver_type.clone();
            let service_guards = service_guards.clone();
            let is_auth = is_auth.clone();

            FieldFuture::new(async move {
                debug!("Start Resolving");
                let data_sources = ctx.data_unchecked::<DataSources>().clone();
                let headers = ctx.data_unchecked::<HeaderMap>().clone();
                let mut token_data = None;

                if is_auth {
                    token_data = ServiceResolver::get_token_data(&ctx, headers.clone())?;
                }

                let input_document =
                    ServiceResolver::get_resolver_input(&ctx, &as_field, &resolver_type)?;

                // If as_field is Some, it is assumed to be a Internal Join.
                // Require input_document to be non-empty.
                if input_document.is_none() {
                    return Ok(None);
                }

                ServiceResolver::guard_resolver(
                    &ctx,
                    &input_document.clone().unwrap(),
                    &entity,
                    service_guards.clone(),
                    &resolver_type,
                    headers,
                    token_data,
                )?;

                let operation_type = ServiceResolver::get_operation_type(&resolver_type, &as_field);

                let results = DataSources::execute(
                    &data_sources,
                    input_document.unwrap(),
                    entity,
                    operation_type,
                )
                .await?;

                Ok(results)
            })
        })
    }
}
