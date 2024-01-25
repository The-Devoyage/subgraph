use bson::Document;
use log::{debug, error, trace};
use sqlx::{mysql::MySqlRow, Row};

use crate::configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig;

use super::ScalarOption;

impl ScalarOption {
    /// Converts a response row into a document, or returns empty document.
    /// This document is to create a internal input.
    /// Types converted are specifc to the mysql database.
    pub fn mysql_response_row_to_input_doc(
        mysql_row: &MySqlRow,
        as_type_field: &ServiceEntityFieldConfig,
        field_name: &str,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Converting Mysql Row to Document");
        trace!("Field Name: {}", field_name);

        let mut document = Document::new();

        // If the config does not provide a value to join on, then allow
        // them to search with any criteria
        if as_type_field.join_on.is_none() {
            return Ok(document);
        }

        match as_type_field.scalar {
            ScalarOption::String
            | ScalarOption::ObjectID
            | ScalarOption::UUID
            | ScalarOption::DateTime => {
                let column_value: Option<&str> = mysql_row.try_get(field_name).map_err(|e| {
                    error!("Error getting string column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            ScalarOption::Int => {
                let column_value: Option<i64> = mysql_row.try_get(field_name).map_err(|e| {
                    error!("Error getting int column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            ScalarOption::Boolean => {
                let column_value: Option<bool> = mysql_row.try_get(field_name).map_err(|e| {
                    error!("Error getting boolean column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            _ => {
                error!("Unsupported scalar type: {:?}", as_type_field.scalar);
                Err(async_graphql::Error::new(format!(
                    "Unsupported scalar type: {:?}",
                    as_type_field.scalar
                )))?
            }
        }

        trace!("{:?}", document);
        Ok(document)
    }
}