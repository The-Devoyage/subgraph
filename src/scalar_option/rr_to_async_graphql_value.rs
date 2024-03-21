use crate::{data_sources::sql::services::ResponseRow, graphql::entity::ServiceEntity};
use async_graphql::Value;
use log::{debug, error, trace};

use super::ScalarOption;

impl ScalarOption {
    // needs to accept response row or json value
    pub fn rr_to_async_graphql_value(
        self,
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving SQL Field");

        let value = match self {
            ScalarOption::String => {
                let value = ServiceEntity::resolve_sql_string_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(Value::from(value.to_string())),
                    None => Ok(Value::Null),
                }
            }
            ScalarOption::Int => {
                let value = ServiceEntity::resolve_sql_int_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ScalarOption::Boolean => {
                let value = ServiceEntity::resolve_sql_bool_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ScalarOption::UUID => {
                let value = ServiceEntity::resolve_sql_uuid_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(Value::from(value.to_string())),
                    None => Ok(Value::Null),
                }
            }
            ScalarOption::DateTime => {
                let value = ServiceEntity::resolve_sql_datetime_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(Value::from(value.to_rfc3339())),
                    None => Ok(Value::Null),
                }
            }
            _ => {
                error!("ScalarOption {:?} not implemented", self);
                Err(async_graphql::Error::new(format!(
                    "ScalarOption {:?} not implemented",
                    self
                )))
            }
        };

        trace!("{:?}", value);
        value
    }
}
