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
                let value: &str = row.try_get(field_name)?;
                Ok(Value::from(value.to_string()))
            }
            ResponseRow::SqLite(row) => {
                let value: &str = row.try_get(field_name)?;
                Ok(Value::from(value.to_string()))
            }
            ResponseRow::Postgres(row) => {
                let value: &str = row.try_get(field_name)?;
                Ok(Value::from(value.to_string()))
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
                let value = row.try_get_unchecked::<i32, _>(field_name);
                match value {
                    Ok(value) => Ok(Value::from(value)),
                    Err(_) => {
                        let value: i64 = row.try_get(field_name)?;
                        Ok(Value::from(value))
                    }
                }
            }
            ResponseRow::SqLite(row) => {
                let value: i32 = row.try_get(field_name)?;
                Ok(Value::from(value))
            }
            ResponseRow::Postgres(row) => {
                let value: i32 = row.try_get(field_name)?;
                Ok(Value::from(value))
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
                let value: bool = row.try_get(field_name)?;
                Ok(Value::from(value))
            }
            ResponseRow::SqLite(row) => {
                let value: bool = row.try_get(field_name)?;
                Ok(Value::from(value))
            }
            ResponseRow::Postgres(row) => {
                let value: bool = row.try_get(field_name)?;
                Ok(Value::from(value))
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
                let value: &str = row.try_get(field_name)?;
                Ok(Value::from(value.to_string()))
            }
            ResponseRow::SqLite(row) => {
                let value: &str = row.try_get(field_name)?;
                Ok(Value::from(value.to_string()))
            }
            ResponseRow::Postgres(row) => {
                let value = row
                    .try_get(field_name)
                    .map(|value: uuid::Uuid| value.to_string())
                    .map_err(|e| {
                        error!("Error resolving UUID field: {:?}", e.to_string());
                        async_graphql::Error::new(format!(
                            "Error resolving UUID field: {:?}",
                            e.to_string()
                        ))
                    })?;
                Ok(Value::from(value.to_string()))
            }
        }
    }
}
