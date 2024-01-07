use async_graphql::{
    dynamic::{FieldFuture, ResolverContext},
    SelectionField,
};
use http::HeaderMap;
use log::debug;

use crate::data_sources::DataSources;

use super::ServiceResolver;

mod get_operation_type;
mod get_resolver_input;
mod get_token_data;
mod guard_resolver;
mod handle_default_values;
mod remove_virtual_fields;

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
        let subgraph_config = self.subgraph_config.clone();

        Box::new(move |ctx: ResolverContext| {
            debug!("Resolving Field: {}", ctx.field().name());
            let entity = entity.clone();
            let as_field = as_field.clone();
            let resolver_type = resolver_type.clone();
            let service_guards = service_guards.clone();
            let is_auth = is_auth.clone();
            let subgraph_config = subgraph_config.clone();

            FieldFuture::new(async move {
                debug!("Start Resolving");
                let data_sources = ctx.data_unchecked::<DataSources>().clone();
                let headers = ctx.data_unchecked::<HeaderMap>().clone();
                let mut token_data = None;

                if is_auth {
                    token_data = ServiceResolver::get_token_data(&ctx, headers.clone())?;
                }

                let input_document = ServiceResolver::get_resolver_input(
                    &ctx,
                    &as_field,
                    &resolver_type,
                    &data_sources,
                    &entity,
                )?;

                // If input document is none, then return none.
                // This is the case when peforming internal joins without any provided input from
                // the ds or the client and is not an error, but should not be resolved.
                if input_document.is_none() {
                    return Ok(None);
                }

                let selection_fields = ctx
                    .field()
                    .selection_set()
                    .into_iter()
                    .map(|f| f)
                    .collect::<Vec<SelectionField>>();
                // Find the field that matches the entity name.
                let entity_field = selection_fields
                    .iter()
                    .find(|f| f.name() == "data")
                    .unwrap();
                let entity_fields = entity_field
                    .selection_set()
                    .into_iter()
                    .map(|f| f)
                    .collect::<Vec<SelectionField>>();

                let guard_context = ServiceResolver::guard_resolver_function(
                    entity_fields,
                    &input_document.clone().unwrap(),
                    &entity,
                    service_guards.clone(),
                    &resolver_type,
                    headers,
                    token_data,
                    &data_sources,
                    &subgraph_config,
                )
                .await?;

                // Handle default values
                let input_document = ServiceResolver::handle_default_values(
                    &input_document.unwrap(),
                    &entity,
                    &resolver_type,
                    guard_context,
                )?;

                let input_document =
                    ServiceResolver::remove_virtual_fields(&input_document, &entity.fields);

                let operation_type = ServiceResolver::get_operation_type(&resolver_type, &as_field);

                let results = DataSources::execute(
                    &data_sources,
                    input_document,
                    entity,
                    operation_type,
                    &subgraph_config,
                )
                .await?;

                Ok(results)
            })
        })
    }
}
