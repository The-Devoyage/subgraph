use bson::{oid::ObjectId, Bson};
use log::{debug, error, trace};

use super::DocumentUtils;

#[derive(Debug)]
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
    Null,
    None,
}

impl DocumentUtils {
    pub fn get_document_string_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!("Getting Document String Scalar: {}", field_name);

        if document.get(field_name).is_none() {
            return Ok(DocumentValue::None);
        }

        if document.get(field_name).unwrap().as_null().is_some() {
            return Ok(DocumentValue::Null);
        }

        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let valid_strings = documents.iter().all(|value| value.as_str().is_some());

                if !valid_strings {
                    error!("Not all values are strings for field {}", field_name);
                    return Err(async_graphql::Error::new(format!(
                        "Not all values are strings for field {}",
                        field_name
                    )));
                }

                let values = documents
                    .into_iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect::<Vec<String>>();
                trace!("Document Value String Array: {:?}", values);
                return Ok(DocumentValue::StringArray(values));
            } else {
                trace!("Document Value String Array: Empty Vec");
                return Ok(DocumentValue::StringArray(vec![]));
            }
        }

        let value = document.get_str(field_name).map_err(|err| {
            error!("Value is not a string: {}", err);
            async_graphql::Error::new(format!("Value is not a string: {}", err))
        })?;

        trace!("Found String Value: {:?}", value);
        Ok(DocumentValue::String(value.to_string()))
    }

    pub fn get_document_int_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!("Getting Document Int Scalar: {}", field_name);

        if document.get(field_name).is_none() {
            return Ok(DocumentValue::None);
        }

        if document.get(field_name).unwrap().as_null().is_some() {
            return Ok(DocumentValue::Null);
        }

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
                    error!("Not all values are ints for field {}", field_name);
                    return Err(async_graphql::Error::new(format!(
                        "Not all values are ints for field {}",
                        field_name
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
                trace!("Document Value Int Array: {:?}", values);
                return Ok(DocumentValue::IntArray(values));
            } else {
                trace!("Document Value Int Array: Empty Vec");
                return Ok(DocumentValue::IntArray(vec![]));
            }
        }

        let value = document.get(field_name).unwrap();
        let i32_value = value.as_i32();
        if i32_value.is_none() {
            let i64_value = value.as_i64();
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
        trace!("Found Int Value: {:?}", value);
        Ok(DocumentValue::Int(i32_value.unwrap()))
    }

    pub fn get_document_boolean_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!("Getting Document Boolean Scalar: {}", field_name);

        if document.get(field_name).is_none() {
            return Ok(DocumentValue::None);
        }

        if document.get(field_name).unwrap().as_null().is_some() {
            return Ok(DocumentValue::Null);
        }

        if is_list {
            let valid_bools = document
                .get_array(field_name)?
                .into_iter()
                .all(|value| value.as_bool().is_some());

            if !valid_bools {
                error!("Not all values are booleans for field {}", field_name);
                return Err(async_graphql::Error::new(format!(
                    "Not all values are booleans for field {}",
                    field_name
                )));
            }

            let values = document
                .get_array(field_name)?
                .into_iter()
                .map(|value| value.as_bool().unwrap())
                .collect::<Vec<bool>>();
            trace!("Document Value Boolean Array: {:?}", values);
            return Ok(DocumentValue::BooleanArray(values));
        }

        let value = document.get_bool(field_name).map_err(|err| {
            error!("Value is not a boolean: {}", err);
            async_graphql::Error::new(format!("Value is not a boolean: {}", err))
        })?;
        trace!("Found Boolean Value: {:?}", value);
        Ok(DocumentValue::Boolean(value))
    }

    pub fn get_document_uuid_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!("Getting Document UUID Scalar: {}", field_name);

        if document.get(field_name).is_none() {
            return Ok(DocumentValue::None);
        }

        if document.get(field_name).unwrap().as_null().is_some() {
            return Ok(DocumentValue::Null);
        }

        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let valid_uuids = documents.iter().all(|value| {
                    let value = value.as_str().unwrap_or("");
                    let uuid = uuid::Uuid::parse_str(value);
                    if uuid.is_err() {
                        return false;
                    } else {
                        return true;
                    }
                });

                if !valid_uuids {
                    error!("Not all values are uuids for field {}", field_name);
                    return Err(async_graphql::Error::new(format!(
                        "Not all values are uuids for field {}",
                        field_name
                    )));
                }

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
                trace!("Document Value UUID Array: {:?}", values);
                return Ok(DocumentValue::UUIDArray(values));
            } else {
                trace!("Document Value UUID Array: Empty Vec");
                return Ok(DocumentValue::UUIDArray(vec![]));
            }
        }

        let value = document.get_str(field_name).map_err(|err| {
            error!("Value is not a uuid: {}", err);
            async_graphql::Error::new(format!("Value is not a uuid: {}", err))
        })?;

        trace!("Document Value UUID: {:?}", value);
        Ok(DocumentValue::UUID(uuid::Uuid::parse_str(value).unwrap()))
    }

    pub fn get_document_datetime_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!("Getting Document DateTime Scalar: {}", field_name);

        if document.get(field_name).is_none() {
            return Ok(DocumentValue::None);
        }

        if document.get(field_name).unwrap().as_null().is_some() {
            return Ok(DocumentValue::Null);
        }

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
                    error!("Not all values are valid dates for field {}", field_name);
                    return Err(async_graphql::Error::new("Invalid DateTime"));
                }
                let values = documents
                    .into_iter()
                    .map(|value| {
                        let value = value.as_datetime().unwrap();
                        value.to_chrono()
                    })
                    .collect();
                trace!("Document Value DateTime Array: {:?}", values);
                return Ok(DocumentValue::DateTimeArray(values));
            } else {
                trace!("Document Value DateTime Array: Empty Vec");
                return Ok(DocumentValue::DateTimeArray(vec![]));
            }
        }

        let value = document.get_datetime(field_name).map_err(|err| {
            error!("Value is not a datetime: {}", err);
            async_graphql::Error::new(format!("Value is not a datetime: {}", err))
        })?;
        // convert bson datetime to chrono datetime
        trace!("Document Value DateTime: {:?}", value);
        Ok(DocumentValue::DateTime(value.to_chrono()))
    }

    pub fn get_document_object_id_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!("Getting Document ObjectID Scalar: {}", field_name);

        if document.get(field_name).is_none() {
            return Ok(DocumentValue::None);
        }

        if document.get(field_name).unwrap().as_null().is_some() {
            return Ok(DocumentValue::Null);
        }

        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let valid_object_ids = documents.iter().all(|value| {
                    let value = value.as_object_id();
                    if value.is_none() {
                        return false;
                    }
                    true
                });

                if !valid_object_ids {
                    error!("Not all values are object ids for field {}", field_name);
                    return Err(async_graphql::Error::new(format!(
                        "Not all values are object ids for field {}",
                        field_name
                    )));
                }

                let value = documents
                    .into_iter()
                    .map(|value| value.as_object_id().unwrap())
                    .collect::<Vec<ObjectId>>();
                trace!("Document Value ObjectID Array: {:?}", value);
                return Ok(DocumentValue::ObjectIDArray(value));
            } else {
                trace!("Document Value ObjectID Array: Empty Vec");
                return Ok(DocumentValue::ObjectIDArray(vec![]));
            }
        }
        let value = document.get_object_id(field_name).map_err(|err| {
            error!("Value is not an object id: {}", err);
            async_graphql::Error::new(format!("Value is not an object id: {}", err))
        })?;
        trace!("Document Value ObjectID: {:?}", value);
        Ok(DocumentValue::ObjectID(value))
    }

    pub fn get_document_object_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!("Resolving Object Scalar");

        if document.get(field_name).is_none() {
            return Ok(DocumentValue::None);
        }

        if document.get(field_name).unwrap().as_null().is_some() {
            return Ok(DocumentValue::Null);
        }

        let value = document.get(field_name).unwrap();

        if is_list {
            if let Some(bson_array) = value.as_array() {
                let valid_docs = bson_array.iter().all(|value| {
                    let value = value.as_document();
                    if value.is_none() {
                        return false;
                    }
                    true
                });

                if !valid_docs {
                    error!("Not all values are documents for field {}", field_name);
                    return Err(async_graphql::Error::new(format!(
                        "Not all values are documents for field {}",
                        field_name
                    )));
                }

                let values = bson_array
                    .into_iter()
                    .map(|value| value.as_document().unwrap().clone())
                    .collect::<Vec<bson::Document>>();
                trace!("Document Value Object Array: {:?}", values);
                return Ok(DocumentValue::DocumentArray(values));
            } else {
                trace!("Document Value Object Array: Empty Vec");
                return Ok(DocumentValue::DocumentArray(vec![]));
            }
        } else {
            trace!("Document Value Object: {:?}", value);
            Ok(DocumentValue::Document(
                value.as_document().unwrap().clone(),
            ))
        }
    }

    pub fn get_document_enum_scalar(
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!("Resolving Enum Scalar");

        if document.get(field_name).is_none() {
            return Ok(DocumentValue::None);
        }

        if document.get(field_name).unwrap().as_null().is_some() {
            return Ok(DocumentValue::Null);
        }

        if is_list {
            if let Some(Bson::Array(documents)) = document.get(field_name) {
                let valid_strings = documents.iter().all(|value| value.as_str().is_some());

                if !valid_strings {
                    error!("Not all values are strings for field {}", field_name);
                    return Err(async_graphql::Error::new(format!(
                        "Not all values are strings for field {}",
                        field_name
                    )));
                }

                let values = documents
                    .into_iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect::<Vec<String>>();
                trace!("Document Value String Array: {:?}", values);
                return Ok(DocumentValue::StringArray(values));
            } else {
                trace!("Document Value String Array: Empty Vec");
                return Ok(DocumentValue::StringArray(vec![]));
            }
        }

        let value = document.get_str(field_name).map_err(|err| {
            error!("Value is not a string: {}", err);
            async_graphql::Error::new(format!("Value is not a string: {}", err))
        })?;

        trace!("Found String Value: {:?}", value);
        Ok(DocumentValue::String(value.to_string()))
    }
}
