use async_graphql::dynamic::ResolverContext;
use bson::Document;
use log::debug;

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
                ServiceResolver::create_internal_input(&ctx, as_field.clone(), data_source)?
            }
            _ => {
                let input = ctx.args.try_get(&format!("{}_input", ctx.field().name()))?;
                let input_document = &input.deserialize::<Document>().unwrap();
                Some(input_document.clone())
            }
        };

        debug!("Resolver Input: {:?}", input_document);
        Ok(input_document)
    }
}
