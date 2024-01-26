use bson::Document;
use log::{debug, error, trace};
use sqlx::{sqlite::SqliteRow, Row};

use crate::configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig;

use super::ScalarOption;

impl ScalarOption {
    /// Converts a response row into a document, or returns empty document.
    /// This document is to create a internal input.
    /// Types converted are specifc to the sqlite database.
    pub fn sqlite_rr_to_input_doc(
        &self,
        sqlite_row: &SqliteRow,
        as_type_field: &ServiceEntityFieldConfig,
        field_name: &str,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Converting Sqlite Row to Document");
        trace!("Field Name: {}", field_name);
        let mut document = Document::new();

        // If the config does not provide a value to join on, then allow
        // search with any criteria
        if as_type_field.join_on.is_none() {
            trace!("No Join On Value Provided");
            return Ok(document);
        }

        match self {
            ScalarOption::String
            | ScalarOption::ObjectID
            | ScalarOption::UUID
            | ScalarOption::DateTime => {
                trace!("Getting String Column Value");
                let column_value: Option<&str> = sqlite_row.try_get(field_name).map_err(|e| {
                    error!("Error getting string column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            ScalarOption::Int => {
                trace!("Getting Int Column Value");
                let column_value: Option<i64> = sqlite_row.try_get(field_name).map_err(|e| {
                    error!("Error getting int column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            ScalarOption::Boolean => {
                trace!("Getting Boolean Column Value");
                let column_value: Option<bool> = sqlite_row.try_get(field_name).map_err(|e| {
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
