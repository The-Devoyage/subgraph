use async_graphql::dynamic::ResolverContext;
use bson::Document;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityField,
    graphql::schema::ResolverType,
};

use super::ServiceResolver;

mod create_internal_input;

impl ServiceResolver {
    pub fn get_resolver_input(
        ctx: &ResolverContext,
        as_field: &Option<ServiceEntityField>,
        resolver_type: &ResolverType,
    ) -> Result<Document, async_graphql::Error> {
        let input_document = match resolver_type {
            ResolverType::InternalType => {
                let as_field = if let Some(as_field) = as_field {
                    as_field
                } else {
                    return Err(async_graphql::Error::new("Field is not a nested field"));
                };
                ServiceResolver::create_internal_input(&ctx, as_field.clone())?
            }
            _ => {
                let input = ctx.args.try_get(&format!("{}_input", ctx.field().name()))?;
                let input_document = &input.deserialize::<Document>().unwrap();
                input_document.clone()
            }
        };

        Ok(input_document)
    }
}
