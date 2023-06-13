use crate::configuration::subgraph::entities::ScalarOptions;
use crate::graphql::schema::ServiceSchemaBuilder;

use async_graphql::{indexmap::IndexMap, Name, Value};
use bson::{Bson, Document};
use log::debug;

impl ServiceSchemaBuilder {
    pub fn resolve_document_field(
        document: &Document,
        field_name: &str,
        scalar: ScalarOptions,
        is_list: bool,
    ) -> Result<Value, async_graphql::Error> {
        debug!(
            "Resolving Mongo Field/Scalar: '{}: {:?}'",
            field_name, scalar
        );

        match scalar {
            ScalarOptions::String => {
                ServiceSchemaBuilder::resolve_document_string_scalar(document, field_name, is_list)
            }
            ScalarOptions::Int => {
                ServiceSchemaBuilder::resolve_document_int_scalar(document, field_name, is_list)
            }
            ScalarOptions::Boolean => {
                ServiceSchemaBuilder::resolve_document_boolean_scalar(document, field_name, is_list)
            }
            ScalarOptions::ObjectID => ServiceSchemaBuilder::resolve_document_object_id_scalar(
                document, field_name, is_list,
            ),
            ScalarOptions::Object => {
                ServiceSchemaBuilder::resolve_document_object_scalar(document, field_name, is_list)
            }
        }
    }

    pub fn resolve_document_string_scalar(
        document: &Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving String Scalar");
        if is_list {
            debug!("---Is List: {:?}", is_list);
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let value = documents
                    .into_iter()
                    .map(|value| Value::from(value.as_str().unwrap()))
                    .collect::<Vec<Value>>();
                debug!("---Found String Value: {:?}", value);
                return Ok(Value::List(value));
            } else {
                return Ok(Value::List(vec![]));
            }
        }
        let value = document.get_str(field_name)?;
        debug!("---Found String Value: {:?}", value);
        Ok(Value::from(value))
    }

    pub fn resolve_document_int_scalar(
        document: &Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<Value, async_graphql::Error> {
        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let value = documents
                    .into_iter()
                    .map(|value| {
                        if let Some(value) = value.as_i32() {
                            Value::from(value)
                        } else {
                            Value::from(value.as_i64().unwrap())
                        }
                    })
                    .collect::<Vec<Value>>();
                debug!("Found Int Value: {:?}", value);
                return Ok(Value::List(value));
            } else {
                return Ok(Value::List(vec![]));
            }
        }

        let value = document.get(field_name).unwrap();
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

    pub fn resolve_document_boolean_scalar(
        document: &Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<Value, async_graphql::Error> {
        if is_list {
            let value = document
                .get_array(field_name)?
                .into_iter()
                .map(|value| Value::from(value.as_bool().unwrap()))
                .collect::<Vec<Value>>();
            debug!("Found Boolean Value: {:?}", value);
            return Ok(Value::List(value));
        }

        let value = document.get_bool(field_name)?;
        debug!("Found Boolean Value: {:?}", value);
        Ok(Value::from(value))
    }

    pub fn resolve_document_object_id_scalar(
        document: &Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<Value, async_graphql::Error> {
        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let value = documents
                    .into_iter()
                    .map(|value| Value::from(value.as_str().unwrap()))
                    .collect::<Vec<Value>>();
                debug!("Found ObjectID Value: {:?}", value);
                return Ok(Value::List(value));
            } else {
                return Ok(Value::List(vec![]));
            }
        }

        let value = document.get_object_id(field_name)?;
        debug!("Found ObjectID Value: {:?}", value);
        Ok(Value::from(value.to_string()))
    }

    pub fn parse_bson_object(value: &Bson) -> Result<IndexMap<Name, Value>, async_graphql::Error> {
        let document = value.as_document().unwrap();

        let mut index_map = IndexMap::new();

        for (key, bson) in document.into_iter() {
            let name = Name::new(key);
            debug!("---Found BSON Element Type: {:?}", bson.element_type());
            let bson_element_type = bson.element_type();

            if ScalarOptions::String.to_bson_type() == bson_element_type {
                let value = ServiceSchemaBuilder::resolve_document_field(
                    document,
                    key,
                    ScalarOptions::String,
                    false,
                )?;
                index_map.insert(name, value);
            } else if ScalarOptions::Int.to_bson_type() == bson_element_type {
                let value = ServiceSchemaBuilder::resolve_document_field(
                    document,
                    key,
                    ScalarOptions::Int,
                    false,
                )?;
                index_map.insert(name, value);
            } else if ScalarOptions::Boolean.to_bson_type() == bson_element_type {
                let value = ServiceSchemaBuilder::resolve_document_field(
                    document,
                    key,
                    ScalarOptions::Boolean,
                    false,
                )?;
                index_map.insert(name, value);
            } else if ScalarOptions::ObjectID.to_bson_type() == bson_element_type {
                let value = ServiceSchemaBuilder::resolve_document_field(
                    document,
                    key,
                    ScalarOptions::ObjectID,
                    false,
                )?;
                index_map.insert(name, value);
            } else if ScalarOptions::Object.to_bson_type() == bson_element_type {
                let value = ServiceSchemaBuilder::resolve_document_field(
                    document,
                    key,
                    ScalarOptions::Object,
                    false,
                )?;
                index_map.insert(name, value);
            }
        }
        Ok(index_map)
    }

    pub fn resolve_document_object_scalar(
        document: &Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving Object Scalar");
        let value = document.get(field_name);

        if value.is_none() {
            return Ok(Value::Null);
        }
        debug!("Found Object Value: {:?}", value);

        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let value = documents
                    .into_iter()
                    .map(|value| ServiceSchemaBuilder::parse_bson_object(value).unwrap())
                    .collect::<Vec<IndexMap<Name, Value>>>();
                debug!("Found Object Value: {:?}", value);
                return Ok(Value::List(
                    value
                        .into_iter()
                        .map(|value| Value::Object(value))
                        .collect::<Vec<Value>>(),
                ));
            } else {
                return Ok(Value::List(vec![]));
            }
        }

        let index_map = ServiceSchemaBuilder::parse_bson_object(value.unwrap())?;

        debug!("Converted To Index Map: {:?}", index_map);

        Ok(Value::from(index_map))
    }
}
