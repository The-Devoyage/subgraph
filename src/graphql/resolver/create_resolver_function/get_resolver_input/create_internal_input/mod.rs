use async_graphql::dynamic::ResolverContext;
use bson::Document;
use log::debug;

use crate::configuration::subgraph::entities::service_entity_field::ServiceEntityField;

use super::ServiceResolver;

mod combine_list_values;
mod combine_primitive_value;

impl ServiceResolver {
    pub fn create_internal_input(
        ctx: &ResolverContext,
        field: ServiceEntityField,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Creating Internal Input: {:?}", ctx.field().name());

        let field_name = ctx.field().name().to_string();
        let parent_value = match ctx.parent_value.downcast_ref::<Document>() {
            Some(parent_value) => Some(parent_value.clone()),
            None => None,
        };
        let parent_value = if let Some(parent_value) = parent_value {
            parent_value
        } else {
            Document::new()
        };

        let field_input = ctx.args.try_get(&format!("{}", ctx.field().name()))?;
        let mut field_input = match field_input.deserialize::<Document>() {
            Ok(field_input) => field_input,
            Err(_) => {
                return Err(async_graphql::Error::new(format!(
                    "Invalid input for field: {}",
                    field_name
                )))
            }
        };

        let join_on = match field.join_on.clone() {
            Some(join_on) => join_on,
            None => {
                return Ok(field_input);
            }
        };
        let scalar = field.scalar.clone();
        let list = field.list.unwrap_or(false);

        match list {
            true => {
                field_input = ServiceResolver::combine_list_values(
                    &parent_value,
                    &mut field_input,
                    &field_name,
                    &scalar,
                    &join_on,
                )?
            }
            false => {
                field_input = ServiceResolver::combine_primitive_value(
                    &parent_value,
                    &mut field_input,
                    &field_name,
                    &scalar,
                    &join_on,
                )?
            }
        };

        debug!("Internal Input: {:?}", field_input);
        Ok(field_input)
    }
}
