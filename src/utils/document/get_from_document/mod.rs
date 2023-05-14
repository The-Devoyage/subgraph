use bson::Bson;
use log::debug;

use crate::configuration::subgraph::entities::{
    service_entity_field::ServiceEntityField, ScalarOptions,
};

use super::Document;

pub enum GetDocumentResultType {
    String(String),
    StringArray(Vec<String>),
    Int(i32),
    IntArray(Vec<i32>),
    Boolean(bool),
    BooleanArray(Vec<bool>),
    Document(bson::Document),
    DocumentArray(Vec<bson::Document>),
}

impl Document {
    /// Get a value from a document.
    pub fn get_from_document(
        document: &bson::Document,
        field: ServiceEntityField,
    ) -> Result<GetDocumentResultType, async_graphql::Error> {
        debug!(
            "Resolving Mongo Field/Scalar: '{}: {:?}'",
            field.name, field.scalar
        );

        match field.scalar {
            ScalarOptions::String => Document::get_document_string_scalar(
                document,
                &field.name,
                field.list.unwrap_or(false),
            ),
            ScalarOptions::Int => Document::get_document_int_scalar(
                document,
                &field.name,
                field.list.unwrap_or(false),
            ),
            ScalarOptions::Boolean => Document::get_document_boolean_scalar(
                document,
                &field.name,
                field.list.unwrap_or(false),
            ),
            ScalarOptions::ObjectID => Document::get_document_object_id_scalar(
                document,
                &field.name,
                field.list.unwrap_or(false),
            ),
            ScalarOptions::Object => Document::get_document_object_scalar(
                document,
                &field.name,
                field.list.unwrap_or(false),
            ),
        }
    }

    pub fn get_document_string_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<GetDocumentResultType, async_graphql::Error> {
        debug!("Resolving String Scalar");
        if is_list {
            debug!("---Is List: {:?}", is_list);
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let values = documents
                    .into_iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect::<Vec<String>>();
                debug!("Found String Values: {:?}", values);
                return Ok(GetDocumentResultType::StringArray(values));
            } else {
                return Ok(GetDocumentResultType::StringArray(vec![]));
            }
        }
        let value = document.get_str(field_name)?;
        debug!("Found String Value: {:?}", value);
        Ok(GetDocumentResultType::String(value.to_string()))
    }

    pub fn get_document_int_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<GetDocumentResultType, async_graphql::Error> {
        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let values = documents
                    .into_iter()
                    .map(|value| value.as_i32().unwrap())
                    .collect::<Vec<i32>>();
                debug!("Found Int Values: {:?}", values);
                return Ok(GetDocumentResultType::IntArray(values));
            } else {
                return Ok(GetDocumentResultType::IntArray(vec![]));
            }
        }

        let value = document.get(field_name).unwrap();
        debug!("Found Int Value: {:?}", value);
        Ok(GetDocumentResultType::Int(value.as_i32().unwrap()))
    }

    pub fn get_document_boolean_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<GetDocumentResultType, async_graphql::Error> {
        if is_list {
            let values = document
                .get_array(field_name)?
                .into_iter()
                .map(|value| value.as_bool().unwrap())
                .collect::<Vec<bool>>();
            debug!("Found Boolean Value: {:?}", values);
            return Ok(GetDocumentResultType::BooleanArray(values));
        }

        let value = document.get_bool(field_name)?;
        debug!("Found Boolean Value: {:?}", value);
        Ok(GetDocumentResultType::Boolean(value))
    }

    pub fn get_document_object_id_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<GetDocumentResultType, async_graphql::Error> {
        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let value = documents
                    .into_iter()
                    .map(|value| value.as_object_id().unwrap().to_string())
                    .collect::<Vec<String>>();
                debug!("Found ObjectID Value: {:?}", value);
                return Ok(GetDocumentResultType::StringArray(value));
            } else {
                return Ok(GetDocumentResultType::StringArray(vec![]));
            }
        }

        let value = document.get_object_id(field_name)?;
        debug!("Found ObjectID Value: {:?}", value);
        Ok(GetDocumentResultType::String(value.to_string()))
    }

    pub fn get_document_object_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<GetDocumentResultType, async_graphql::Error> {
        debug!("Resolving Object Scalar");
        let value = document.get(field_name);

        if value.is_none() {
            return Err(async_graphql::Error::new("No Object Value Found"));
        }

        debug!("Found Object Value: {:?}", value);

        let value = value.unwrap();
        if is_list {
            if let Some(bson_array) = value.as_array() {
                let values = bson_array
                    .into_iter()
                    .map(|value| value.as_document().unwrap().clone())
                    .collect::<Vec<bson::Document>>();
                debug!("Found Object Values: {:?}", values);
                return Ok(GetDocumentResultType::DocumentArray(values));
            } else {
                return Ok(GetDocumentResultType::DocumentArray(vec![]));
            }
        } else {
            Ok(GetDocumentResultType::Document(
                value.as_document().unwrap().clone(),
            ))
        }
    }
}
