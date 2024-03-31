use async_graphql::{indexmap::IndexMap, Name, Value};
use bson::{Bson, Document};
use log::{debug, trace};

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    graphql::entity::ServiceEntity, utils::document::get_from_document::DocumentValue,
};

impl ServiceEntity {
    pub fn resolve_document_object_id_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolving Object ID Scalar");
        let resolved =
            field
                .scalar
                .get_from_document(document, &field.name, field.list.unwrap_or(false))?;

        match resolved {
            DocumentValue::ObjectID(object_id) => Ok(Some(Value::from(object_id.to_string()))),
            DocumentValue::ObjectIDArray(object_ids) => Ok(Some(Value::List(
                object_ids
                    .into_iter()
                    .map(|object_id| Value::from(object_id.to_string()))
                    .collect(),
            ))),
            DocumentValue::Null => Ok(Some(Value::Null)),
            DocumentValue::None => Ok(None),
            _ => Err(async_graphql::Error::from(
                "Invalid result type for object id scalar",
            )),
        }
    }

    pub fn resolve_document_string_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolving String Scalar");
        let resolved =
            field
                .scalar
                .get_from_document(document, &field.name, field.list.unwrap_or(false))?;

        match resolved {
            DocumentValue::String(value) => Ok(Some(Value::from(value))),
            DocumentValue::StringArray(values) => Ok(Some(Value::List(
                values.into_iter().map(|value| Value::from(value)).collect(),
            ))),
            DocumentValue::None => Ok(None),
            DocumentValue::Null => Ok(Some(Value::Null)),
            _ => Err(async_graphql::Error::from(
                "Invalid result type for string scalar",
            )),
        }
    }

    pub fn resolve_document_int_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolving Int Scalar");

        let resolved =
            field
                .scalar
                .get_from_document(document, &field.name, field.list.unwrap_or(false))?;

        match resolved {
            DocumentValue::Int(value) => Ok(Some(Value::from(value))),
            DocumentValue::IntArray(values) => Ok(Some(Value::List(
                values.into_iter().map(|value| Value::from(value)).collect(),
            ))),
            DocumentValue::None => Ok(None),
            DocumentValue::Null => Ok(Some(Value::Null)),
            _ => Err(async_graphql::Error::from(
                "Invalid result type for int scalar",
            )),
        }
    }

    pub fn resolve_document_boolean_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolving Boolean Scalar");

        let resolved =
            field
                .scalar
                .get_from_document(document, &field.name, field.list.unwrap_or(false))?;

        match resolved {
            DocumentValue::Boolean(value) => Ok(Some(Value::from(value))),
            DocumentValue::BooleanArray(values) => Ok(Some(Value::List(
                values.into_iter().map(|value| Value::from(value)).collect(),
            ))),
            DocumentValue::Null => Ok(Some(Value::Null)),
            DocumentValue::None => Ok(None),
            _ => Err(async_graphql::Error::from(
                "Invalid result type for boolean scalar",
            )),
        }
    }

    pub fn resolve_document_uuid_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolving UUID Scalar");

        let resolved =
            field
                .scalar
                .get_from_document(document, &field.name, field.list.unwrap_or(false))?;

        match resolved {
            DocumentValue::UUID(value) => Ok(Some(Value::from(value.to_string()))),
            DocumentValue::UUIDArray(values) => Ok(Some(Value::List(
                values
                    .into_iter()
                    .map(|value| Value::from(value.to_string()))
                    .collect(),
            ))),
            DocumentValue::Null => Ok(Some(Value::Null)),
            DocumentValue::None => Ok(None),
            _ => Err(async_graphql::Error::from(
                "Invalid result type for UUID scalar",
            )),
        }
    }

    pub fn resolve_document_datetime_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolving DateTime Scalar");

        let resolved =
            field
                .scalar
                .get_from_document(document, &field.name, field.list.unwrap_or(false))?;

        match resolved {
            DocumentValue::DateTime(value) => Ok(Some(Value::from(value.to_rfc3339()))),
            DocumentValue::DateTimeArray(values) => Ok(Some(Value::List(
                values
                    .into_iter()
                    .map(|value| Value::from(value.to_rfc3339()))
                    .collect(),
            ))),
            DocumentValue::Null => Ok(Some(Value::Null)),
            DocumentValue::None => Ok(None),
            _ => Err(async_graphql::Error::from(
                "Invalid result type for DateTime scalar",
            )),
        }
    }

    pub fn parse_bson_object(
        value: &Bson,
        field: &ServiceEntityFieldConfig,
    ) -> Result<IndexMap<Name, Value>, async_graphql::Error> {
        let document = match value.as_document() {
            Some(document) => document,
            None => return Err(async_graphql::Error::from("Invalid BSON Document")),
        };

        let mut index_map = IndexMap::new();

        for (key, bson) in document.into_iter() {
            let name = Name::new(key);
            trace!("Found BSON Element Type: {:?}", bson.element_type());

            let field = ServiceEntityFieldConfig::get_field(
                field.fields.clone().unwrap_or(Vec::new()),
                key.clone(),
            )?;
            let value = field
                .scalar
                .clone()
                .document_field_to_async_graphql_value(document, &field)?;
            if let Some(value) = value {
                index_map.insert(name, value);
            }
        }
        Ok(index_map)
    }

    pub fn resolve_document_object_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolve Document Object Scalar");
        let value = document.get(&field.name);

        if value.is_none() {
            trace!("Value is None, returning null");
            return Ok(None);
        }

        if field.list.unwrap_or(false) {
            if let Some(Bson::Array(documents)) = document.get(field.name.clone()) {
                let mut values = vec![];
                for document in documents {
                    let value = ServiceEntity::parse_bson_object(document, field)?;
                    values.push(Value::Object(value));
                }

                return Ok(Some(Value::List(values)));
            } else {
                return Ok(Some(Value::List(vec![])));
            }
        }

        let index_map = ServiceEntity::parse_bson_object(value.unwrap(), field)?;

        Ok(Some(Value::from(index_map)))
    }

    pub fn resolve_document_enum_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolving Enum Scalar");

        let resolved =
            field
                .scalar
                .get_from_document(document, &field.name, field.list.unwrap_or(false))?;

        match resolved {
            DocumentValue::String(value) => Ok(Some(Value::from(value))),
            DocumentValue::StringArray(values) => Ok(Some(Value::List(
                values.into_iter().map(|value| Value::from(value)).collect(),
            ))),
            DocumentValue::None => Ok(None),
            DocumentValue::Null => Ok(Some(Value::Null)),
            _ => Err(async_graphql::Error::from(
                "Invalid result type for enum scalar",
            )),
        }
    }
}
