use bson::Document;
use log::{debug, error};
use sqlx::{mysql::MySqlRow, postgres::PgRow, sqlite::SqliteRow, Row};
use uuid::Uuid;

use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityFieldConfig, ScalarOptions,
    },
    graphql::resolver::ServiceResolver,
};

impl ServiceResolver {
    /// Converts a response row into a document, or returns empty document.
    pub fn sqlite_response_row_to_doc(
        sqlite_row: &SqliteRow,
        as_type_field: &ServiceEntityFieldConfig,
        field_name: &str,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Converting Sqlite Row to Document");
        let mut document = Document::new();

        // If the config does not provide a value to join on, then allow
        // search with any criteria
        if as_type_field.join_on.is_none() {
            return Ok(document);
        }

        match as_type_field.scalar {
            ScalarOptions::String
            | ScalarOptions::ObjectID
            | ScalarOptions::UUID
            | ScalarOptions::DateTime => {
                let column_value: Option<&str> = sqlite_row.try_get(field_name).map_err(|e| {
                    error!("Error getting string column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            ScalarOptions::Int => {
                let column_value: Option<i64> = sqlite_row.try_get(field_name).map_err(|e| {
                    error!("Error getting int column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            ScalarOptions::Boolean => {
                let column_value: Option<bool> = sqlite_row.try_get(field_name).map_err(|e| {
                    error!("Error getting boolean column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            _ => Err(async_graphql::Error::new(format!(
                "Unsupported scalar type: {:?}",
                as_type_field.scalar
            )))?,
        }
        debug!("Sqlite Row Converted to Document: {:?}", document);
        Ok(document)
    }

    pub fn mysql_response_row_to_doc(
        mysql_row: &MySqlRow,
        as_type_field: &ServiceEntityFieldConfig,
        field_name: &str,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Converting Mysql Row to Document");

        let mut document = Document::new();

        // If the config does not provide a value to join on, then allow
        // them to search with any criteria
        if as_type_field.join_on.is_none() {
            return Ok(document);
        }

        match as_type_field.scalar {
            ScalarOptions::String
            | ScalarOptions::ObjectID
            | ScalarOptions::UUID
            | ScalarOptions::DateTime => {
                let column_value: Option<&str> = mysql_row.try_get(field_name).map_err(|e| {
                    error!("Error getting string column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            ScalarOptions::Int => {
                let column_value: Option<i64> = mysql_row.try_get(field_name).map_err(|e| {
                    error!("Error getting int column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            ScalarOptions::Boolean => {
                let column_value: Option<bool> = mysql_row.try_get(field_name).map_err(|e| {
                    error!("Error getting boolean column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            _ => Err(async_graphql::Error::new(format!(
                "Unsupported scalar type: {:?}",
                as_type_field.scalar
            )))?,
        }
        Ok(document)
    }
    pub fn postgres_response_row_to_doc(
        pg_row: &PgRow,
        as_type_field: &ServiceEntityFieldConfig,
        field_name: &str,
    ) -> Result<Document, async_graphql::Error> {
        let mut document = Document::new();

        // If the config does not provide a value to join on, then allow
        // them to search with any criteria
        if as_type_field.join_on.is_none() {
            return Ok(document);
        }

        match as_type_field.scalar {
            ScalarOptions::String | ScalarOptions::ObjectID | ScalarOptions::DateTime => {
                let column_value: Option<&str> = pg_row.try_get(field_name).map_err(|e| {
                    error!("Error getting string column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            ScalarOptions::Int => {
                let column_value: Option<i64> = pg_row.try_get(field_name).map_err(|e| {
                    error!("Error getting int column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            //TODO: Ensure nothing is added if null is received.
            ScalarOptions::UUID => {
                let column_value: Option<Uuid> = pg_row.try_get(field_name).map_err(|e| {
                    error!("Error getting uuid column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;

                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value.to_string());
                }
            }
            ScalarOptions::Boolean => {
                let column_value: Option<bool> = pg_row.try_get(field_name).map_err(|e| {
                    error!("Error getting boolean column value: {}", e);
                    async_graphql::Error::new(format!("Error getting column value: {}", e))
                })?;
                if let Some(column_value) = column_value {
                    document.insert(field_name, column_value);
                }
            }
            _ => Err(async_graphql::Error::new(format!(
                "Unsupported scalar type: {:?}",
                as_type_field.scalar
            )))?,
        }
        Ok(document)
    }
}
