use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityField,
    graphql::schema::ServiceSchemaBuilder,
};

use async_graphql::{dynamic::ResolverContext, indexmap::IndexMap, ErrorExtensions, Value};
use bson::to_document;
use log::debug;

impl ServiceSchemaBuilder {
    pub fn resolve_nested(
        ctx: &ResolverContext,
        entity_field: &ServiceEntityField,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolving Nested Field: {:?}", ctx.field().name());

        let field_name = ctx.field().name();

        let object = match ctx.parent_value.as_value() {
            Some(value) => value,
            None => {
                let index_map = IndexMap::new();
                return Ok(Some(Value::from(index_map)));
            }
        };

        let json = match object.clone().into_json() {
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

        let json_object = if json.is_string() {
            let json_str = match json.as_str() {
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
            json
        };

        let document = if json_object.is_array() {
            match to_document(&json_object[0]) {
                Ok(value) => value,
                Err(_) => {
                    return Err(async_graphql::Error::new(
                        "Invalid JSON Object - Failed to convert to BSON document",
                    )
                    .extend_with(|_err, e| {
                        e.set("field", field_name);
                    }))
                }
            }
        } else if json_object.is_object() {
            match to_document(&json_object) {
                Ok(value) => value,
                Err(_) => {
                    return Err(async_graphql::Error::new(
                        "Invalid JSON Object - Failed to convert to BSON document",
                    )
                    .extend_with(|_err, e| {
                        e.set("field", field_name);
                    }))
                }
            }
        } else {
            return Err(async_graphql::Error::new(
                "Invalid JSON Object - Received unexpected JSON type",
            )
            .extend_with(|_err, e| {
                e.set("field", field_name);
                e.set("received", json_object.to_string());
            }));
        };

        let value = ServiceSchemaBuilder::resolve_document_field(
            &document,
            field_name,
            entity_field.scalar.clone(),
            entity_field.list.unwrap_or(false),
        );

        let value = match value {
            Ok(value) => value,
            Err(_) => {
                return Err(
                    async_graphql::Error::new("Failed to resolve document field.").extend_with(
                        |_err, e| {
                            e.set("field", field_name);
                        },
                    ),
                )
            }
        };

        debug!(
            "Found Document Field Value: {:?}: {:?} - {:?}",
            field_name, value, entity_field.scalar
        );

        Ok(Some(value.clone()))
    }
}
