use async_graphql::dynamic::ResolverContext;
use bson::Document;
use log::{debug, trace};

use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityFieldConfig, ServiceEntityConfig,
    },
    data_sources::DataSources,
    resolver_type::ResolverType,
};

use super::ServiceResolver;

mod create_internal_input;

impl ServiceResolver {
    /// Gets the resolver input. This is the input that will be used to query the database.
    /// If the resolver is an internal type, then the input will be created from the parent value
    /// combined with the input provided from the client.
    pub fn get_resolver_input(
        ctx: &ResolverContext,
        as_type_field: &Option<ServiceEntityFieldConfig>,
        resolver_type: &ResolverType,
        data_sources: &DataSources,
        entity: &ServiceEntityConfig,
    ) -> Result<Option<Document>, async_graphql::Error> {
        debug!("Getting Resolver Input");

        let data_source = DataSources::get_entity_data_soruce(data_sources, entity);
        let input_document = match resolver_type {
            ResolverType::InternalType => {
                let as_field = if let Some(as_field) = as_type_field {
                    as_field
                } else {
                    return Err(async_graphql::Error::new("Field is not a nested field"));
                };

                let join_on = if let Some(join_on) = as_field.join_on.clone() {
                    join_on
                } else {
                    return Err(async_graphql::Error::new(
                        "Field does not have a join_on field",
                    ));
                };

                // Determine if the `join_on` field is eager
                let join_on_field =
                    ServiceEntityConfig::get_field(entity.clone(), join_on.clone())?;

                ServiceResolver::create_internal_input(
                    &ctx,
                    as_field.clone(),
                    join_on_field,
                    data_source,
                )?
            }
            _ => {
                let input = ctx.args.try_get(&format!("{}_input", ctx.field().name()))?;
                let input_document = &input.deserialize::<Document>().unwrap();
                Some(input_document.clone())
            }
        };

        trace!("Resolver Input: {:?}", input_document);
        Ok(input_document)
    }
}
