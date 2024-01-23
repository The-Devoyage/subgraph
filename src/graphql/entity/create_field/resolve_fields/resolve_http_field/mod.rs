use async_graphql::Value;
use json::JsonValue;
use log::debug;

use crate::{graphql::entity::ServiceEntity, scalar_option::ScalarOption};

impl ServiceEntity {
    pub fn resolve_http_field(
        json_value: &JsonValue,
        field_name: &str,
        scalar: ScalarOption,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving HTTP Field");

        let value = &json_value[field_name];

        // Match the scalar type and get the value json
        match scalar {
            ScalarOption::String => {
                if value.is_null() || value == "null" {
                    return Ok(Value::Null);
                }
                Ok(Value::from(value.to_string()))
            }
            ScalarOption::Int => {
                let value = value.as_i32();
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ScalarOption::Boolean => {
                let value = value.as_bool();

                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ScalarOption::ObjectID => {
                let value = value.to_string();
                Ok(Value::from(value))
            }
            ScalarOption::Object => {
                let value = value.to_string();
                Ok(Value::from(value))
            }
            ScalarOption::UUID => {
                let value = value.to_string();
                Ok(Value::from(value))
            }
            ScalarOption::DateTime => {
                let value = value.to_string();
                Ok(Value::from(value))
            }
        }
    }
}
