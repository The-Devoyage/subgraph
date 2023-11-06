use async_graphql::dynamic::ResolverContext;
use bson::{doc, Document};
use log::{debug, error};
use sqlx::Row;
use uuid::Uuid;

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
    ) -> Result<Option<Document>, async_graphql::Error> {
        debug!("Creating Internal Input: {:?}", ctx.field().name());
        debug!("As Type Field: {:?}", as_type_field);

        let field_name = if let Some(join_from) = as_type_field.join_from {
            join_from
        } else {
            ctx.field().name().to_string()
        };

        // Try to downcast as a document, which is what is returned from mongo db.
        // If it fails, then try to downcast as a response row, which is what is returned from sql.
        let parent_value = match ctx.parent_value.try_downcast_ref::<Option<Document>>() {
            Ok(parent_value) => {
                if let Some(parent_value) = parent_value {
                    debug!("Mongo Parent Value: {:?}", parent_value);
                    Some(parent_value.clone())
                } else {
                    debug!("Mongo Parent Value: None");
                    None
                }
            }
            Err(_) => {
                if let Some(rr) = ctx.parent_value.downcast_ref::<Option<ResponseRow>>() {
                    debug!("SQL Parent Value");
                    match rr {
                        // If the parent value is a ResponseRow, we can use it to get the value of the
                        // field.
                        // Map the value into a Document, which is what the resolver expects.
                        Some(response_row) => match response_row {
                            ResponseRow::SqLite(rr) => {
                                let mut document = Document::new();

                                // If the config does not provide a value to join on, then allow
                                // them to search with any criteria
                                if as_type_field.join_on.is_none() {
                                    return Ok(Some(document));
                                }

                                match as_type_field.scalar {
                                    ScalarOptions::Int => {
                                        let column_value: Option<i64> =
                                            rr.try_get(field_name.as_str()).map_err(|e| {
                                                error!("Error getting int column value: {}", e);
                                                async_graphql::Error::new(format!(
                                                    "Error getting column value: {}",
                                                    e
                                                ))
                                            })?;
                                        if let Some(column_value) = column_value {
                                            document.insert(&field_name, column_value);
                                        }
                                    }
                                    ScalarOptions::String
                                    | ScalarOptions::ObjectID
                                    | ScalarOptions::UUID
                                    | ScalarOptions::DateTime => {
                                        let column_value: Option<&str> =
                                            rr.try_get(field_name.as_str()).map_err(|e| {
                                                error!("Error getting string column value: {}", e);
                                                async_graphql::Error::new(format!(
                                                    "Error getting column value: {}",
                                                    e
                                                ))
                                            })?;
                                        if let Some(column_value) = column_value {
                                            document.insert(&field_name, column_value);
                                        }
                                    }
                                    ScalarOptions::Boolean => {
                                        let column_value: Option<bool> =
                                            rr.try_get(field_name.as_str()).map_err(|e| {
                                                error!("Error getting boolean column value: {}", e);
                                                async_graphql::Error::new(format!(
                                                    "Error getting column value: {}",
                                                    e
                                                ))
                                            })?;
                                        if let Some(column_value) = column_value {
                                            document.insert(&field_name, column_value);
                                        }
                                    }
                                    _ => Err(async_graphql::Error::new(format!(
                                        "Unsupported scalar type: {:?}",
                                        as_type_field.scalar
                                    )))?,
                                }
                                Some(document)
                            }
                            ResponseRow::MySql(rr) => {
                                let mut document = Document::new();

                                // If the config does not provide a value to join on, then allow
                                // them to search with any criteria
                                if as_type_field.join_on.is_none() {
                                    return Ok(Some(document));
                                }

                                match as_type_field.scalar {
                                    ScalarOptions::Int => {
                                        let column_value: Option<i64> =
                                            rr.try_get(field_name.as_str()).map_err(|e| {
                                                error!("Error getting int column value: {}", e);
                                                async_graphql::Error::new(format!(
                                                    "Error getting column value: {}",
                                                    e
                                                ))
                                            })?;
                                        if let Some(column_value) = column_value {
                                            document.insert(&field_name, column_value);
                                        }
                                    }
                                    ScalarOptions::String
                                    | ScalarOptions::ObjectID
                                    | ScalarOptions::UUID
                                    | ScalarOptions::DateTime => {
                                        let column_value: Option<&str> =
                                            rr.try_get(field_name.as_str()).map_err(|e| {
                                                error!("Error getting string column value: {}", e);
                                                async_graphql::Error::new(format!(
                                                    "Error getting column value: {}",
                                                    e
                                                ))
                                            })?;
                                        if let Some(column_value) = column_value {
                                            document.insert(&field_name, column_value);
                                        }
                                    }
                                    ScalarOptions::Boolean => {
                                        let column_value: Option<bool> =
                                            rr.try_get(field_name.as_str()).map_err(|e| {
                                                error!("Error getting boolean column value: {}", e);
                                                async_graphql::Error::new(format!(
                                                    "Error getting column value: {}",
                                                    e
                                                ))
                                            })?;
                                        if let Some(column_value) = column_value {
                                            document.insert(&field_name, column_value);
                                        }
                                    }
                                    _ => Err(async_graphql::Error::new(format!(
                                        "Unsupported scalar type: {:?}",
                                        as_type_field.scalar
                                    )))?,
                                }
                                Some(document)
                            }
                            ResponseRow::Postgres(rr) => {
                                let mut document = Document::new();

                                // If the config does not provide a value to join on, then allow
                                // them to search with any criteria
                                if as_type_field.join_on.is_none() {
                                    return Ok(Some(document));
                                }

                                match as_type_field.scalar {
                                    ScalarOptions::Int => {
                                        let column_value: Option<i64> =
                                            rr.try_get(field_name.as_str()).map_err(|e| {
                                                error!("Error getting int column value: {}", e);
                                                async_graphql::Error::new(format!(
                                                    "Error getting column value: {}",
                                                    e
                                                ))
                                            })?;
                                        if let Some(column_value) = column_value {
                                            document.insert(&field_name, column_value);
                                        }
                                    }
                                    ScalarOptions::String
                                    | ScalarOptions::ObjectID
                                    | ScalarOptions::DateTime => {
                                        let column_value: Option<&str> =
                                            rr.try_get(field_name.as_str()).map_err(|e| {
                                                error!("Error getting string column value: {}", e);
                                                async_graphql::Error::new(format!(
                                                    "Error getting column value: {}",
                                                    e
                                                ))
                                            })?;
                                        if let Some(column_value) = column_value {
                                            document.insert(&field_name, column_value);
                                        }
                                    }
                                    //TODO: Ensure nothing is added if null is received.
                                    ScalarOptions::UUID => {
                                        let column_value: Option<Uuid> =
                                            rr.try_get(field_name.as_str()).map_err(|e| {
                                                error!("Error getting uuid column value: {}", e);
                                                async_graphql::Error::new(format!(
                                                    "Error getting column value: {}",
                                                    e
                                                ))
                                            })?;

                                        if let Some(column_value) = column_value {
                                            document.insert(&field_name, column_value.to_string());
                                        }
                                    }
                                    ScalarOptions::Boolean => {
                                        let column_value: Option<bool> =
                                            rr.try_get(field_name.as_str()).map_err(|e| {
                                                error!("Error getting boolean column value: {}", e);
                                                async_graphql::Error::new(format!(
                                                    "Error getting column value: {}",
                                                    e
                                                ))
                                            })?;
                                        if let Some(column_value) = column_value {
                                            document.insert(&field_name, column_value);
                                        }
                                    }
                                    _ => Err(async_graphql::Error::new(format!(
                                        "Unsupported scalar type: {:?}",
                                        as_type_field.scalar
                                    )))?,
                                }
                                Some(document)
                            }
                        },
                        None => {
                            debug!("No Parent Value - SQL");
                            None
                        }
                    }
                } else {
                    None
                }
            }
        };

        let parent_value = if let Some(parent_value) = parent_value {
            parent_value
        } else {
            Document::new()
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
        let list = as_type_field.list.unwrap_or(false);

        //Get the query input, then modify it to include the parent value(s)
        let mut query_input = field_input
            .get("query")
            .unwrap()
            .as_document()
            .unwrap()
            .clone();

        match list {
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
            return Ok(None);
        }

        let field_input = doc! {
            "query": query_input,
        };

        debug!("Internal Input: {:?}", field_input);
        Ok(Some(field_input))
    }
}
