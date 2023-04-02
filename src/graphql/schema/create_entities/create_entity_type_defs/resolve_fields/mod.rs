use crate::configuration::subgraph::entities::ScalarOptions;
use crate::graphql::schema::ServiceSchemaBuilder;

use async_graphql::{indexmap::IndexMap, Name, Value};
use bson::Document;
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

    pub fn resolve_document_field(
        doc: &Document,
        field_name: &str,
        scalar: ScalarOptions,
    ) -> Result<Value, async_graphql::Error> {
        debug!(
            "Resolving Mongo Field/Scalar: '{}: {:?}'",
            field_name, scalar
        );

        match scalar {
            ScalarOptions::String => {
                let value = doc.get_str(field_name)?;
                debug!("Found String Value: {:?}", value);
                Ok(Value::from(value))
            }
            ScalarOptions::Int => {
                let value = doc.get(field_name).unwrap();
                match value {
                    bson::Bson::Int32(value) => {
                        debug!("Found Int Value: {:?}", value);
                        Ok(Value::from(value.clone() as i32))
                    }
                    bson::Bson::Int64(value) => {
                        debug!("Found Int Value: {:?}", value);
                        Ok(Value::from(value.clone() as i64))
                    }
                    _ => Ok(Value::Null),
                }
            }
            ScalarOptions::Boolean => {
                let value = doc.get_bool(field_name)?;
                debug!("Found Boolean Value: {:?}", value);
                Ok(Value::from(value))
            }
            ScalarOptions::ObjectID => {
                let value = doc.get_object_id(field_name)?;
                debug!("Found ObjectID Value: {:?}", value);
                Ok(Value::from(value.to_string()))
            }
            ScalarOptions::Object => {
                let value = doc.get(field_name);

                if value.is_none() {
                    return Ok(Value::Null);
                }

                let document = value.unwrap().as_document().unwrap();

                debug!("Found Object Value: {:?}", document);

                let mut index_map = IndexMap::new();

                for (key, bson) in document.into_iter() {
                    let name = Name::new(key);
                    debug!("Found BSON Element Type: {:?}", bson.element_type());
                    let bson_element_type = bson.element_type();

                    if ScalarOptions::String.to_bson_type() == bson_element_type {
                        let value = ServiceSchemaBuilder::resolve_document_field(
                            document,
                            key,
                            ScalarOptions::String,
                        )?;
                        index_map.insert(name, value);
                    } else if ScalarOptions::Int.to_bson_type() == bson_element_type {
                        let value = ServiceSchemaBuilder::resolve_document_field(
                            document,
                            key,
                            ScalarOptions::Int,
                        )?;
                        index_map.insert(name, value);
                    } else if ScalarOptions::Boolean.to_bson_type() == bson_element_type {
                        let value = ServiceSchemaBuilder::resolve_document_field(
                            document,
                            key,
                            ScalarOptions::Boolean,
                        )?;
                        index_map.insert(name, value);
                    } else if ScalarOptions::ObjectID.to_bson_type() == bson_element_type {
                        let value = ServiceSchemaBuilder::resolve_document_field(
                            document,
                            key,
                            ScalarOptions::ObjectID,
                        )?;
                        index_map.insert(name, value);
                    } else if ScalarOptions::Object.to_bson_type() == bson_element_type {
                        let value = ServiceSchemaBuilder::resolve_document_field(
                            document,
                            key,
                            ScalarOptions::Object,
                        )?;
                        index_map.insert(name, value);
                    }
                }

                debug!("Converted To Index Map: {:?}", index_map);

                Ok(Value::from(index_map))
            }
        }
    }
}
