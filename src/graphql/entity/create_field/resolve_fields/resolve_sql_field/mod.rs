use log::{debug, error};
use sqlx::Row;

use crate::{data_sources::sql::services::ResponseRow, graphql::entity::ServiceEntity};

impl ServiceEntity {
    // From response row to String
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

    pub fn resolve_sql_enum_scalar(
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Option<String>, async_graphql::Error> {
        debug!("Resolving SQL Enum Scalar");

        let value = match response_row {
            ResponseRow::MySql(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Enum field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Enum field: {:?}",
                        e.to_string()
                    ))
                })?;
                value.map(|s| s.to_string())
            }
            ResponseRow::SqLite(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Enum field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Enum field: {:?}",
                        e.to_string()
                    ))
                })?;
                value.map(|s| s.to_string())
            }
            ResponseRow::Postgres(row) => {
                let value: Option<&str> = row.try_get(field_name).map_err(|e| {
                    error!("Error resolving Enum field: {:?}", e.to_string());
                    async_graphql::Error::new(format!(
                        "Error resolving Enum field: {:?}",
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
}
