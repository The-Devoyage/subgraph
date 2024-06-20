use bson::to_document;
use log::{debug, error, trace};

use super::DocumentUtils;

impl DocumentUtils {
    pub fn json_to_document(
        json_value: &serde_json::Value,
    ) -> Result<Option<bson::Document>, async_graphql::Error> {
        debug!("Converting JSON to Document");
        trace!("JSON Value: {:?}", json_value);

        let document = if json_value.is_array() {
            match to_document(&json_value[0]) {
                Ok(value) => Some(value),
                Err(_) => {
                    error!("Invalid JSON Object - Failed to convert Array to BSON document");
                    return Err(async_graphql::Error::new(
                        "Invalid JSON Object - Failed to convert Array to BSON document",
                    ));
                }
            }
        } else if json_value.is_object() {
            trace!("JSON Value is Object");
            match to_document(&json_value) {
                Ok(value) => Some(value),
                Err(_) => {
                    error!("Invalid JSON Object - Failed to convert Object to BSON document");
                    return Err(async_graphql::Error::new(
                        "Invalid JSON Object - Failed to convert Object to BSON document",
                    ));
                }
            }
        } else if json_value.is_null() {
            trace!("JSON Value is Null, returning None");
            Some(bson::Document::new()) // WARN: This is not going to work... For some reason, it
                                        // is resolving Null Values for required fields when the parent value is Null.
        } else {
            error!("Invalid JSON Object - Received unexpected JSON type");
            return Err(async_graphql::Error::new(
                "Invalid JSON Object - Received unexpected JSON type",
            ));
        };

        trace!("Converted JSON to Document: {:?}", document);

        Ok(document)
    }
}
