use async_graphql::Value;
use log::{debug, error};
use sqlx::Row;

use crate::{
    configuration::subgraph::entities::ScalarOptions, data_sources::sql::services::ResponseRow,
    graphql::entity::ServiceEntity,
};

impl ServiceEntity {
    pub fn resolve_sql_field(
        response_row: &ResponseRow,
        field_name: &str,
        scalar: ScalarOptions,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving SQL Field");

        match scalar {
            ScalarOptions::String => {
                ServiceEntity::resolve_sql_string_scalar(response_row, field_name)
            }
            ScalarOptions::Int => ServiceEntity::resolve_sql_int_scalar(response_row, field_name),
            ScalarOptions::Boolean => {
                ServiceEntity::resolve_sql_bool_scalar(response_row, field_name)
            }
            ScalarOptions::UUID => ServiceEntity::resolve_sql_uuid_scalar(response_row, field_name),
            ScalarOptions::DateTime => {
                ServiceEntity::resolve_sql_datetime_scalar(response_row, field_name)
            }
            _ => unreachable!("Unreachable scalar type: {:?}", scalar),
        }
    }

    pub fn resolve_sql_string_scalar(
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving SQL String Scalar");

        match response_row {
            ResponseRow::MySql(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving String field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving String field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Ok(Value::from(value.to_string())),
                    None => Ok(Value::Null),
                }
            }
            ResponseRow::SqLite(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving String field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving String field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Ok(Value::from(value.to_string())),
                    None => Ok(Value::Null),
                }
            }
            ResponseRow::Postgres(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving String field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving String field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Ok(Value::from(value.to_string())),
                    None => Ok(Value::Null),
                }
            }
        }
    }

    pub fn resolve_sql_int_scalar(
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving SQL Int Scalar");

        match response_row {
            ResponseRow::MySql(row) => {
                let value = row.try_get_unchecked::<Option<i32>, _>(field_name);
                match value {
                    Ok(value) => match value {
                        Some(value) => Ok(Value::from(value)),
                        None => Ok(Value::Null),
                    },
                    Err(_) => {
                        let value: Option<i64> = row.try_get(field_name).map_err(|e| {
                            error!("Error resolving Int field: {:?}", e.to_string());
                            async_graphql::Error::new(format!(
                                "Error resolving Int field: {:?}",
                                e.to_string()
                            ))
                        })?;
                        match value {
                            Some(value) => Ok(Value::from(value)),
                            None => Ok(Value::Null),
                        }
                    }
                }
            }
            ResponseRow::SqLite(row) => {
                let value: Option<i32> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Int field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Int field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ResponseRow::Postgres(row) => {
                let value: Option<i32> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Int field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Int field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
        }
    }

    pub fn resolve_sql_bool_scalar(
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving SQL Bool Scalar");

        match response_row {
            ResponseRow::MySql(row) => {
                let value: Option<bool> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Bool field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Bool field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ResponseRow::SqLite(row) => {
                let value: Option<bool> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Bool field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Bool field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ResponseRow::Postgres(row) => {
                let value: Option<bool> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Bool field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Bool field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
        }
    }

    pub fn resolve_sql_uuid_scalar(
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving SQL UUID Scalar");

        match response_row {
            ResponseRow::MySql(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving UUID field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving UUID field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Ok(Value::from(value.to_string())),
                    None => Ok(Value::Null),
                }
            }
            ResponseRow::SqLite(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving UUID field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving UUID field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Ok(Value::from(value.to_string())),
                    None => Ok(Value::Null),
                }
            }
            ResponseRow::Postgres(row) => {
                let value = row
                    .try_get(field_name)
                    .map(|value: uuid::Uuid| Some(value.to_string()))
                    .map_err(|e| {
                        error!("Error resolving UUID field: {:?}", e.to_string());
                        async_graphql::Error::new(format!(
                            "Error resolving UUID field: {:?}",
                            e.to_string()
                        ))
                    })?;
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
        }
    }

    pub fn resolve_sql_datetime_scalar(
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving SQL DateTime Scalar");

        match response_row {
            ResponseRow::MySql(row) => {
                let value: Option<chrono::DateTime<chrono::Utc>> =
                    row.try_get(field_name).map_err(|e| {
                        error!("Error resolving DateTime field: {:?}", e.to_string());
                        async_graphql::Error::new(format!(
                            "Error resolving DateTime field: {:?}",
                            e.to_string()
                        ))
                    })?;
                match value {
                    Some(value) => Ok(Value::from(value.to_rfc3339())),
                    None => Ok(Value::Null),
                }
            }
            ResponseRow::SqLite(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving DateTime field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving DateTime field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Ok(Value::from(value.to_string())),
                    None => Ok(Value::Null),
                }
            }
            ResponseRow::Postgres(row) => {
                let value: Option<chrono::DateTime<chrono::Utc>> =
                    row.try_get(field_name).map_err(|e| {
                        error!("Error resolving DateTime field: {:?}", e.to_string());
                        async_graphql::Error::new(format!(
                            "Error resolving DateTime field: {:?}",
                            e.to_string()
                        ))
                    })?;
                match value {
                    Some(value) => Ok(Value::from(value.to_rfc3339())),
                    None => Ok(Value::Null),
                }
            }
        }
    }
}
