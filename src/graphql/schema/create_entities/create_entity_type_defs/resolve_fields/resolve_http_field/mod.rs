use crate::configuration::subgraph::entities::ScalarOptions;
use crate::graphql::schema::ServiceSchemaBuilder;

use async_graphql::Value;
use json::JsonValue;
use log::{debug, info};

impl ServiceSchemaBuilder {
    pub async fn resolve_http_field(
        json_value: &JsonValue,
        field_name: &str,
        scalar: ScalarOptions,
    ) -> Result<Value, async_graphql::Error> {
        info!("Resolving HTTP Field");

        let value = &json_value[field_name];

        debug!("Accessed Field '{}': {:?}", field_name, value);

        match scalar {
            ScalarOptions::String => {
                debug!("Found String Value: {:?}", value);
                if value.is_null() || value == "null" {
                    return Ok(Value::Null);
                }
                Ok(Value::from(value.to_string()))
            }
            ScalarOptions::Int => {
                debug!("Found Int Value: {:?}", value);
                let value = value.as_i32();
                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ScalarOptions::Boolean => {
                info!("Found Boolean Value: {:?}", value);
                let value = value.as_bool();

                match value {
                    Some(value) => Ok(Value::from(value)),
                    None => Ok(Value::Null),
                }
            }
            ScalarOptions::ObjectID => {
                debug!("Found ObjectID Value: {:?}", value);
                let value = value.to_string();
                Ok(Value::from(value))
            }
            ScalarOptions::Object => {
                debug!("Found Object Value: {:?}", value);
                let value = value.to_string();
                Ok(Value::from(value))
            }
        }
    }
}
