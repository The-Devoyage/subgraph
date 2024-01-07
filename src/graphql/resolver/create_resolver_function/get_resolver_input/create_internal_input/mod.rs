use async_graphql::dynamic::ResolverContext;
use bson::{doc, Document};
use log::debug;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    data_sources::DataSource,
};

use super::ServiceResolver;

mod combine_http_input_value;
mod combine_input_value;
mod get_parent_value;

impl ServiceResolver {
    /// Creates an internal input for a field. Internal inputs represent "joins" or creating the
    /// input for an extended graphql type.
    pub fn create_internal_input(
        ctx: &ResolverContext,
        as_type_field: ServiceEntityFieldConfig,
        data_source: &DataSource,
    ) -> Result<Option<Document>, async_graphql::Error> {
        debug!("Creating Internal Input: {:?}", ctx.field().name());
        debug!("As Type Field: {:?}", as_type_field);

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

        let is_http_ds = match data_source {
            DataSource::HTTP(_) => true,
            _ => false,
        };

        if is_http_ds {
            query_input = ServiceResolver::combine_http_input_value(
                &parent_value,
                &mut query_input,
                &field_name,
                &scalar,
                &join_on,
            )?;
        } else {
            query_input = ServiceResolver::combine_input_value(
                &parent_value,
                &mut query_input,
                &field_name,
                &scalar,
                &join_on,
            )?;
        }

        if query_input.is_empty() {
            debug!("Empty Internal Query Input.");
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
