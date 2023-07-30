use async_graphql::{ErrorExtensions, Value};
use log::debug;

use crate::graphql::entity::ServiceEntity;

impl ServiceEntity {
    pub fn get_parent_value(
        parent_value: &Value,
        field_name: &str,
    ) -> Result<serde_json::Value, async_graphql::Error> {
        debug!(
            "Getting Value, {:?}, from parent, {:?}",
            field_name, parent_value
        );

        let parent_value = match parent_value.clone().into_json() {
            Ok(value) => value,
            Err(_) => {
                return Err(async_graphql::Error::new(
                    "Invalid JSON Object - Failed to convert to JSON",
                )
                .extend_with(|_err, e| {
                    e.set("field", field_name);
                }))
            }
        };

        let parent_value = if parent_value.is_string() {
            let json_str = match parent_value.as_str() {
                Some(value) => value,
                None => {
                    return Err(async_graphql::Error::new(
                        "Invalid JSON Object - Failed to convert to JSON string",
                    )
                    .extend_with(|_err, e| {
                        e.set("field", field_name);
                    }))
                }
            };

            match serde_json::from_str(&json_str) {
                Ok(value) => value,
                Err(_) => {
                    return Err(async_graphql::Error::new(
                        "Invalid JSON Object - Failed to convert to JSON object",
                    )
                    .extend_with(|_err, e| {
                        e.set("field", field_name);
                    }))
                }
            }
        } else {
            parent_value
        };

        debug!("Parent Value: {:?}", parent_value);

        Ok(parent_value)
    }
}
