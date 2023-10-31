use async_graphql::Value;
use json::JsonValue;
use log::debug;

use crate::{configuration::subgraph::entities::ScalarOptions, graphql::entity::ServiceEntity};

impl ServiceEntity {
    pub fn resolve_http_field(
        json_value: &JsonValue,
        field_name: &str,
        scalar: ScalarOptions,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving HTTP Field");

        let value = &json_value[field_name];

        match scalar {
            ScalarOptions::String => {
                if value.is_null() || value == "null" {
                    return Ok(Value::Null);
                }
                Ok(Value::from(value.to_string()))
            }
            ScalarOptions::Int => {
                let value = value.as_i32();
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ScalarOptions::Boolean => {
                let value = value.as_bool();

                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ScalarOptions::ObjectID => {
                let value = value.to_string();
                Ok(Value::from(value))
            }
            ScalarOptions::Object => {
                let value = value.to_string();
                Ok(Value::from(value))
            }
            ScalarOptions::UUID => {
                let value = value.to_string();
                Ok(Value::from(value))
            }
            ScalarOptions::DateTime => {
                let value = value.to_string();
                Ok(Value::from(value))
            }
        }
    }
}
