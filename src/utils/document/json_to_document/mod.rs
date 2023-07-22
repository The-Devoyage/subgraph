use bson::to_document;
use log::debug;

use super::DocumentUtils;

impl DocumentUtils {
    pub fn json_to_document(
        json_value: &serde_json::Value,
    ) -> Result<bson::Document, async_graphql::Error> {
        debug!("Converting JSON to Document");

        let document = if json_value.is_array() {
            match to_document(&json_value[0]) {
                Ok(value) => value,
                Err(_) => {
                    return Err(async_graphql::Error::new(
                        "Invalid JSON Object - Failed to convert Array to BSON document",
                    ))
                }
            }
        } else if json_value.is_object() {
            match to_document(&json_value) {
                Ok(value) => value,
                Err(_) => {
                    return Err(async_graphql::Error::new(
                        "Invalid JSON Object - Failed to convert Object to BSON document",
                    ))
                }
            }
        } else {
            return Err(async_graphql::Error::new(
                "Invalid JSON Object - Received unexpected JSON type",
            ));
        };

        debug!("Converted JSON to Document: {:?}", document);

        Ok(document)
    }
}
