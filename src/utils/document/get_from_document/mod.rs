use bson::{oid::ObjectId, Bson};
use log::{debug, error};

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    scalar_option::ScalarOption,
};

use super::DocumentUtils;

pub enum DocumentValue {
    String(String),
    StringArray(Vec<String>),
    Int(i32),
    IntArray(Vec<i32>),
    Boolean(bool),
    BooleanArray(Vec<bool>),
    ObjectID(bson::oid::ObjectId),
    ObjectIDArray(Vec<bson::oid::ObjectId>),
    Document(bson::Document),
    DocumentArray(Vec<bson::Document>),
    UUID(uuid::Uuid),
    UUIDArray(Vec<uuid::Uuid>),
    DateTime(chrono::DateTime<chrono::Utc>),
    DateTimeArray(Vec<chrono::DateTime<chrono::Utc>>),
    None,
}

impl DocumentUtils {
    /// Get a value from a document.
    /// Uses the scalar type to determine how to get the value.
    pub fn get_from_document(
        document: &bson::Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!(
            "Resolving Mongo Field {}, of type {:?} in {:?}",
            field.name, field.scalar, document
        );

        match field.scalar {
            ScalarOption::String => DocumentUtils::get_document_string_scalar(
                document,
                &field.name,
                field.list.unwrap_or(false),
            ),
            ScalarOption::Int => DocumentUtils::get_document_int_scalar(
                document,
                &field.name,
                field.list.unwrap_or(false),
            ),
            ScalarOption::Boolean => DocumentUtils::get_document_boolean_scalar(
                document,
                &field.name,
                field.list.unwrap_or(false),
            ),
            ScalarOption::ObjectID => DocumentUtils::get_document_object_id_scalar(
                document,
                &field.name,
                field.list.unwrap_or(false),
            ),
            ScalarOption::Object => DocumentUtils::get_document_object_scalar(
                document,
                &field.name,
                field.list.unwrap_or(false),
            ),
            ScalarOption::UUID => DocumentUtils::get_document_uuid_scalar(
                document,
                &field.name,
                field.list.unwrap_or(false),
            ),
            ScalarOption::DateTime => DocumentUtils::get_document_datetime_scalar(
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
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!("Resolving String Scalar");
        if is_list {
            debug!("---Is List: {:?}", is_list);
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let values = documents
                    .into_iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect::<Vec<String>>();
                debug!("Found String Values: {:?}", values);
                return Ok(DocumentValue::StringArray(values));
            } else {
                return Ok(DocumentValue::StringArray(vec![]));
            }
        }
        let value = document.get_str(field_name)?;
        debug!("Found String Value: {:?}", value);
        Ok(DocumentValue::String(value.to_string()))
    }

    pub fn get_document_int_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                // Check that all values are i32 or i64
                let valid = documents.iter().all(|value| {
                    let i32_value = value.as_i32();
                    if i32_value.is_none() {
                        let i64_value = value.as_f64();
                        if i64_value.is_some() {
                            return true;
                        } else {
                            return false;
                        }
                    }
                    return true;
                });

                if !valid {
                    error!("Could not parse int value: {:?}", documents);
                    return Err(async_graphql::Error::new(format!(
                        "Could not parse int value: {:?}",
                        documents
                    )));
                }

                let values = documents
                    .into_iter()
                    .map(|value| {
                        let i32_value = value.as_i32();
                        if i32_value.is_none() {
                            let i64_value = value.as_f64();
                            if i64_value.is_some() {
                                return i64_value.unwrap() as i32;
                            } else {
                                // Alrady checked above.
                                error!("Could not parse int value: {:?}", value);
                                return -1;
                            }
                        }
                        return i32_value.unwrap();
                    })
                    .collect::<Vec<i32>>();
                debug!("Found Int Values: {:?}", values);
                return Ok(DocumentValue::IntArray(values));
            } else {
                return Ok(DocumentValue::IntArray(vec![]));
            }
        }

        let value = document.get(field_name).unwrap();
        let i32_value = value.as_i32();
        if i32_value.is_none() {
            let i64_value = value.as_f64();
            if i64_value.is_some() {
                return Ok(DocumentValue::Int(i64_value.unwrap() as i32));
            } else {
                error!("Could not parse int value: {:?}", value);
                return Err(async_graphql::Error::new(format!(
                    "Could not parse int value: {:?}",
                    value
                )));
            }
        }
        debug!("Found Int Value: {:?}", value);
        Ok(DocumentValue::Int(i32_value.unwrap()))
    }

    pub fn get_document_boolean_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        if is_list {
            let values = document
                .get_array(field_name)?
                .into_iter()
                .map(|value| value.as_bool().unwrap())
                .collect::<Vec<bool>>();
            debug!("Found Boolean Value: {:?}", values);
            return Ok(DocumentValue::BooleanArray(values));
        }

        let value = document.get_bool(field_name)?;
        debug!("Found Boolean Value: {:?}", value);
        Ok(DocumentValue::Boolean(value))
    }

    pub fn get_document_uuid_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let values = documents
                    .into_iter()
                    .map(|value| {
                        let value = value.as_str().unwrap_or("");
                        let uuid = uuid::Uuid::parse_str(value);
                        if uuid.is_err() {
                            uuid::Uuid::nil()
                        } else {
                            uuid.unwrap()
                        }
                    })
                    .collect();
                debug!("Found UUID Values: {:?}", values);
                return Ok(DocumentValue::UUIDArray(values));
            } else {
                return Ok(DocumentValue::UUIDArray(vec![]));
            }
        }

        let value = document.get_str(field_name)?;
        debug!("Found UUID Value: {:?}", value);
        Ok(DocumentValue::UUID(uuid::Uuid::parse_str(value).unwrap()))
    }

    pub fn get_document_datetime_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                // Check all values are valid dates
                let is_valid = documents.iter().all(|value| {
                    let value = value.as_datetime();
                    if value.is_none() {
                        return false;
                    }
                    true
                });
                if !is_valid {
                    return Err(async_graphql::Error::new("Invalid DateTime"));
                }
                let values = documents
                    .into_iter()
                    .map(|value| {
                        let value = value.as_datetime().unwrap();
                        value.to_chrono()
                    })
                    .collect();
                debug!("Found DateTime Values: {:?}", values);
                return Ok(DocumentValue::DateTimeArray(values));
            } else {
                return Ok(DocumentValue::DateTimeArray(vec![]));
            }
        }

        let value = document.get_datetime(field_name)?;
        // convert bson datetime to chrono datetime
        debug!("Found DateTime Value: {:?}", value);
        Ok(DocumentValue::DateTime(value.to_chrono()))
    }

    pub fn get_document_object_id_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let value = documents
                    .into_iter()
                    .map(|value| value.as_object_id().unwrap())
                    .collect::<Vec<ObjectId>>();
                debug!("Found ObjectID Value: {:?}", value);
                return Ok(DocumentValue::ObjectIDArray(value));
            } else {
                return Ok(DocumentValue::ObjectIDArray(vec![]));
            }
        }

        let value = document.get_object_id(field_name)?;
        debug!("Found ObjectID Value: {:?}", value);
        Ok(DocumentValue::ObjectID(value))
    }

    pub fn get_document_object_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
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
                return Ok(DocumentValue::DocumentArray(values));
            } else {
                return Ok(DocumentValue::DocumentArray(vec![]));
            }
        } else {
            Ok(DocumentValue::Document(
                value.as_document().unwrap().clone(),
            ))
        }
    }
}
