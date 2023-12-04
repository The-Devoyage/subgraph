use async_graphql::dynamic::ResolverContext;
use bson::{doc, Document};
use log::debug;

use crate::configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig;

use super::ServiceResolver;

mod combine_list_values;
mod combine_primitive_value;
mod get_parent_value;

impl ServiceResolver {
    /// Creates an internal input for a field. Internal inputs represent "joins" or creating the
    /// input for an extended graphql type.
    pub fn create_internal_input(
        ctx: &ResolverContext,
        as_type_field: ServiceEntityFieldConfig,
    ) -> Result<Option<Document>, async_graphql::Error> {
        debug!("Creating Internal Input: {:?}", ctx.field().name());
        debug!("As Type Field: {:?}", as_type_field);

        // Variables
        let field_name = if let Some(join_from) = as_type_field.join_from.clone() {
            join_from // Use the join_from field name if provided.
        } else {
            ctx.field().name().to_string() // Otherwise use the field name
        };
        let field_input = ctx.args.try_get(&format!("{}", ctx.field().name()))?;
        let field_input = match field_input.deserialize::<Document>() {
            Ok(field_input) => field_input,
            Err(_) => {
                return Err(async_graphql::Error::new(format!(
                    "Invalid input for field: {}",
                    field_name
                )))
            }
        };

        let join_on = match as_type_field.join_on.clone() {
            Some(join_on) => join_on,
            None => {
                return Ok(Some(field_input));
            }
        };
        let scalar = as_type_field.scalar.clone();

        // Logic
        // Get parent value, which may come from various data sources. May or may not exists.
        let parent_value = ServiceResolver::get_parent_value(ctx, &field_name, &as_type_field)
            .map(|parent_value| {
                if let Some(parent_value) = parent_value {
                    parent_value
                } else {
                    Document::new()
                }
            })?;

        // Get the query input, then modify it to include the parent value(s)
        // `query` will exist because it is required from the graphql schema.
        let mut query_input = field_input
            .get("query")
            .unwrap()
            .as_document()
            .unwrap()
            .clone();
        debug!("Current Query Input: {:?}", query_input);

        // Is the parent value is a list, an array from mongo for example.
        let is_array = match parent_value.get_array(&field_name) {
            Ok(_) => true,
            Err(_) => false,
        };

        match is_array {
            true => {
                query_input = ServiceResolver::combine_list_values(
                    &parent_value,
                    &mut query_input,
                    &field_name,
                    &scalar,
                    &join_on,
                )?
            }
            false => {
                query_input = ServiceResolver::combine_primitive_value(
                    &parent_value,
                    &mut query_input,
                    &field_name,
                    &scalar,
                    &join_on,
                )?
            }
        };

        debug!("Query Input: {:?}", query_input);

        if query_input.is_empty() {
            debug!("Query input is empty. Returning None.");
            return Ok(None);
        }

        // Recreate the input with the new query input.
        let field_input = doc! {
            "query": query_input,
        };

        debug!("Internal Input: {:?}", field_input);
        Ok(Some(field_input))
    }
}
