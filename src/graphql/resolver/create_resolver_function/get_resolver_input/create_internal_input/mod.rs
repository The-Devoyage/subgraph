use async_graphql::dynamic::ResolverContext;
use bson::Document;
use log::{debug, error};
use sqlx::Row;

use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityFieldConfig, ScalarOptions,
    },
    data_sources::sql::services::ResponseRow,
};

use super::ServiceResolver;

mod combine_list_values;
mod combine_primitive_value;

impl ServiceResolver {
    pub fn create_internal_input(
        ctx: &ResolverContext,
        as_type_field: ServiceEntityFieldConfig,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Creating Internal Input: {:?}", ctx.field().name());
        debug!("As Type Field: {:?}", as_type_field);

        let field_name = ctx.field().name().to_string();

        let parent_value = match ctx.parent_value.try_downcast_ref::<Document>() {
            Ok(parent_value) => {
                debug!("Mongo Parent Value");
                Some(parent_value.clone())
            }
            Err(_) => match ctx.parent_value.downcast_ref::<ResponseRow>() {
                Some(response_row) => match response_row {
                    ResponseRow::SqLite(rr) => {
                        let mut document = Document::new();

                        match as_type_field.scalar {
                            ScalarOptions::Int => {
                                let column_value: i64 =
                                    rr.try_get(field_name.as_str()).map_err(|e| {
                                        error!("Error getting int column value: {}", e);
                                        async_graphql::Error::new(format!(
                                            "Error getting column value: {}",
                                            e
                                        ))
                                    })?;
                                document.insert(&field_name, column_value);
                            }
                            ScalarOptions::String | ScalarOptions::ObjectID => {
                                let column_value: &str =
                                    rr.try_get(field_name.as_str()).map_err(|e| {
                                        error!("Error getting string column value: {}", e);
                                        async_graphql::Error::new(format!(
                                            "Error getting column value: {}",
                                            e
                                        ))
                                    })?;
                                document.insert(&field_name, column_value);
                            }
                            ScalarOptions::Boolean => {
                                let column_value: bool =
                                    rr.try_get(field_name.as_str()).map_err(|e| {
                                        error!("Error getting boolean column value: {}", e);
                                        async_graphql::Error::new(format!(
                                            "Error getting column value: {}",
                                            e
                                        ))
                                    })?;
                                document.insert(&field_name, column_value);
                            }
                            _ => Err(async_graphql::Error::new(format!(
                                "Unsupported scalar type: {:?}",
                                as_type_field.scalar
                            )))?,
                        }
                        Some(document)
                    }
                    _ => panic!("Unsupported data source"),
                },
                None => {
                    debug!("No Parent Value - SQL");
                    None
                }
            },
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

        let join_on = match as_type_field.join_on.clone() {
            Some(join_on) => join_on,
            None => {
                return Ok(field_input);
            }
        };
        let scalar = as_type_field.scalar.clone();
        let list = as_type_field.list.unwrap_or(false);

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
