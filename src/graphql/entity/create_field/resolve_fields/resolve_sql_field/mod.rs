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
                let value = ServiceEntity::resolve_sql_string_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(Value::from(value.to_string())),
                    None => Ok(Value::Null),
                }
            }
            ScalarOptions::Int => {
                let value = ServiceEntity::resolve_sql_int_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ScalarOptions::Boolean => {
                let value = ServiceEntity::resolve_sql_bool_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ScalarOptions::UUID => {
                let value = ServiceEntity::resolve_sql_uuid_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(Value::from(value.to_string())),
                    None => Ok(Value::Null),
                }
            }
            ScalarOptions::DateTime => {
                let value = ServiceEntity::resolve_sql_datetime_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(Value::from(value.to_rfc3339())),
                    None => Ok(Value::Null),
                }
            }
            _ => unreachable!("Unreachable scalar type: {:?}", scalar),
        }
    }

    pub fn resolve_sql_field_json(
        response_row: &ResponseRow,
        field_name: &str,
        scalar: ScalarOptions,
    ) -> Result<serde_json::Value, async_graphql::Error> {
        debug!("Resolving SQL Field");
        let field_value = match scalar {
            ScalarOptions::String => {
                let value = ServiceEntity::resolve_sql_string_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(serde_json::Value::String(value.to_string())),
                    None => Ok(serde_json::Value::Null),
                }
            }
            ScalarOptions::Int => {
                let value = ServiceEntity::resolve_sql_int_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(serde_json::Value::Number(serde_json::Number::from(value))),
                    None => Ok(serde_json::Value::Null),
                }
            }
            ScalarOptions::Boolean => {
                let value = ServiceEntity::resolve_sql_bool_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(serde_json::Value::Bool(value)),
                    None => Ok(serde_json::Value::Null),
                }
            }
            ScalarOptions::UUID => {
                let value = ServiceEntity::resolve_sql_uuid_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(serde_json::Value::String(value.to_string())),
                    None => Ok(serde_json::Value::Null),
                }
            }
            ScalarOptions::DateTime => {
                let value = ServiceEntity::resolve_sql_datetime_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(serde_json::Value::String(value.to_rfc3339())),
                    None => Ok(serde_json::Value::Null),
                }
            }
            _ => unreachable!("Unreachable scalar type: {:?}", scalar),
        };
        debug!("Resolved SQL Field: {:?}", field_value);
        field_value
    }

    pub fn resolve_sql_string_scalar(
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Option<String>, async_graphql::Error> {
        debug!("Resolving SQL String Scalar");

        let value = match response_row {
            ResponseRow::MySql(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving String field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving String field: {:?}",
                        e.to_string()
                    ))
                })?;
                value.map(|s| s.to_string())
            }
            ResponseRow::SqLite(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving String field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving String field: {:?}",
                        e.to_string()
                    ))
                })?;
                value.map(|s| s.to_string())
            }
            ResponseRow::Postgres(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving String field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving String field: {:?}",
                        e.to_string()
                    ))
                })?;
                value.map(|s| s.to_string())
            }
        };

        match value {
            Some(value) => Ok(Some(value)),
            None => Ok(None),
        }
    }

    pub fn resolve_sql_int_scalar(
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Option<i32>, async_graphql::Error> {
        debug!("Resolving SQL Int Scalar");

        let value = match response_row {
            ResponseRow::MySql(row) => {
                let value = match row.try_get_unchecked::<Option<i32>, _>(field_name) {
                    Ok(value) => value,
                    Err(_) => {
                        let value: Option<i64> = row.try_get(field_name).map_err(|e| {
                            error!("Error resolving Int field: {:?}", e.to_string());
                            async_graphql::Error::new(format!(
                                "Error resolving Int field: {:?}",
                                e.to_string()
                            ))
                        })?;
                        value.map(|v| v as i32)
                    }
                };
                value
            }
            ResponseRow::SqLite(row) => {
                let value: Option<i32> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Int field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Int field: {:?}",
                        e.to_string()
                    ))
                })?;
                value
            }
            ResponseRow::Postgres(row) => {
                let value: Option<i32> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Int field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Int field: {:?}",
                        e.to_string()
                    ))
                })?;
                value
            }
        };

        match value {
            Some(value) => Ok(Some(value)),
            None => Ok(None),
        }
    }

    pub fn resolve_sql_bool_scalar(
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Option<bool>, async_graphql::Error> {
        debug!("Resolving SQL Bool Scalar");

        let value = match response_row {
            ResponseRow::MySql(row) => {
                let value: Option<bool> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Bool field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Bool field: {:?}",
                        e.to_string()
                    ))
                })?;
                value
            }
            ResponseRow::SqLite(row) => {
                let value: Option<bool> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Bool field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Bool field: {:?}",
                        e.to_string()
                    ))
                })?;
                value
            }
            ResponseRow::Postgres(row) => {
                let value: Option<bool> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Bool field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Bool field: {:?}",
                        e.to_string()
                    ))
                })?;
                value
            }
        };

        match value {
            Some(value) => Ok(Some(value)),
            None => Ok(None),
        }
    }

    pub fn resolve_sql_uuid_scalar(
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Option<uuid::Uuid>, async_graphql::Error> {
        debug!("Resolving SQL UUID Scalar");

        let value = match response_row {
            ResponseRow::MySql(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving UUID field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving UUID field: {:?}",
                        e.to_string()
                    ))
                })?;
                match value {
                    Some(value) => Some(uuid::Uuid::parse_str(value).map_err(|e| {
                        error!("Error resolving UUID field: {:?}", e.to_string());
                        async_graphql::Error::new(format!(
                            "Error resolving UUID field: {:?}",
                            e.to_string()
                        ))
                    })?),
                    None => None,
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
                    Some(value) => Some(uuid::Uuid::parse_str(value).map_err(|e| {
                        error!("Error resolving UUID field: {:?}", e.to_string());
                        async_graphql::Error::new(format!(
                            "Error resolving UUID field: {:?}",
                            e.to_string()
                        ))
                    })?),
                    None => None,
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
                    Some(value) => Some(uuid::Uuid::parse_str(&value).map_err(|e| {
                        error!("Error resolving UUID field: {:?}", e.to_string());
                        async_graphql::Error::new(format!(
                            "Error resolving UUID field: {:?}",
                            e.to_string()
                        ))
                    })?),
                    None => None,
                }
            }
        };

        match value {
            Some(value) => Ok(Some(value)),
            None => Ok(None),
        }
    }

    pub fn resolve_sql_datetime_scalar(
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Option<chrono::DateTime<chrono::Utc>>, async_graphql::Error> {
        debug!("Resolving SQL DateTime Scalar");

        let value = match response_row {
            ResponseRow::MySql(row) => {
                let value: Option<chrono::DateTime<chrono::Utc>> =
                    row.try_get(field_name).map_err(|e| {
                        error!("Error resolving DateTime field: {:?}", e.to_string());
                        async_graphql::Error::new(format!(
                            "Error resolving DateTime field: {:?}",
                            e.to_string()
                        ))
                    })?;
                value
            }
            ResponseRow::SqLite(row) => {
                let value = row.try_get(field_name);
                match value {
                    Ok(value) => Some(value),
                    Err(_) => None,
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
                value
            }
        };

        match value {
            Some(value) => Ok(Some(value)),
            None => Ok(None),
        }
    }
}
