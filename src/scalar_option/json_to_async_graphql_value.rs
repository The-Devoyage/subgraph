use super::ScalarOption;
use async_graphql::Value;
use json::JsonValue;
use log::{debug, trace};

impl ScalarOption {
    pub fn json_to_async_graphql_value(
        &self,
        json_value: &JsonValue,
        field_name: &str,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving HTTP Field");
        trace!("json_value: {:?}", json_value);

        let value = &json_value[field_name];

        // Match the scalar type and get the value json
        let value = match self {
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
        };

        trace!("{:?}", value);

        value
    }
}
