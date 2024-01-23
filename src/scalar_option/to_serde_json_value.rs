use crate::{data_sources::sql::services::ResponseRow, graphql::entity::ServiceEntity};
use log::{debug, error, trace};

use super::ScalarOption;

impl ScalarOption {
    // from response_row to_json
    pub fn to_serde_json_value(
        self,
        response_row: &ResponseRow,
        field_name: &str,
    ) -> Result<serde_json::Value, async_graphql::Error> {
        debug!("Resolving SQL Field");

        let field_value = match self {
            ScalarOption::String => {
                let value = ServiceEntity::resolve_sql_string_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(serde_json::Value::String(value.to_string())),
                    None => Ok(serde_json::Value::Null),
                }
            }
            ScalarOption::Int => {
                let value = ServiceEntity::resolve_sql_int_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(serde_json::Value::Number(serde_json::Number::from(value))),
                    None => Ok(serde_json::Value::Null),
                }
            }
            ScalarOption::Boolean => {
                let value = ServiceEntity::resolve_sql_bool_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(serde_json::Value::Bool(value)),
                    None => Ok(serde_json::Value::Null),
                }
            }
            ScalarOption::UUID => {
                let value = ServiceEntity::resolve_sql_uuid_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(serde_json::Value::String(value.to_string())),
                    None => Ok(serde_json::Value::Null),
                }
            }
            ScalarOption::DateTime => {
                let value = ServiceEntity::resolve_sql_datetime_scalar(response_row, field_name)?;
                match value {
                    Some(value) => Ok(serde_json::Value::String(value.to_rfc3339())),
                    None => Ok(serde_json::Value::Null),
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

        trace!("{:?}", field_value);
        field_value
    }
}
