use async_graphql::{indexmap::IndexMap, Name, Value};
use bson::{Bson, Document};
use log::debug;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    graphql::entity::ServiceEntity, scalar_option::ScalarOption,
    utils::document::get_from_document::DocumentValue,
};

impl ServiceEntity {
    pub fn resolve_document_field(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving Document Field: {:?}", field.name);

        match &field.scalar {
            ScalarOption::String => ServiceEntity::resolve_document_string_scalar(document, field),
            ScalarOption::ObjectID => {
                ServiceEntity::resolve_document_object_id_scalar(document, field)
            }
            ScalarOption::Int => ServiceEntity::resolve_document_int_scalar(document, field),
            ScalarOption::Boolean => {
                ServiceEntity::resolve_document_boolean_scalar(document, field)
            }
            ScalarOption::Object => ServiceEntity::resolve_document_object_scalar(document, field),
            ScalarOption::UUID => ServiceEntity::resolve_document_uuid_scalar(document, field),
            ScalarOption::DateTime => {
                ServiceEntity::resolve_document_datetime_scalar(document, field)
            }
        }
    }

    pub fn resolve_document_object_id_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving Object ID Scalar");
        let resolved = ScalarOption::get_from_document(document, field)?;

        match resolved {
            DocumentValue::ObjectID(object_id) => Ok(Value::from(object_id.to_string())),
            _ => unreachable!("Invalid result type for object id scalar"),
        }
    }

    pub fn resolve_document_string_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving String Scalar");
        let resolved = ScalarOption::get_from_document(document, field)?;

        match resolved {
            DocumentValue::String(value) => Ok(Value::from(value)),
            DocumentValue::StringArray(values) => Ok(Value::List(
                values.into_iter().map(|value| Value::from(value)).collect(),
            )),
            _ => unreachable!("Invalid result type for string scalar"),
        }
    }

    pub fn resolve_document_int_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving Int Scalar");

        let resolved = ScalarOption::get_from_document(document, field)?;

        match resolved {
            DocumentValue::Int(value) => Ok(Value::from(value)),
            DocumentValue::IntArray(values) => Ok(Value::List(
                values.into_iter().map(|value| Value::from(value)).collect(),
            )),
            _ => unreachable!("Invalid result type for int scalar"),
        }
    }

    pub fn resolve_document_boolean_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving Boolean Scalar");

        let resolved = ScalarOption::get_from_document(document, field)?;

        match resolved {
            DocumentValue::Boolean(value) => Ok(Value::from(value)),
            DocumentValue::BooleanArray(values) => Ok(Value::List(
                values.into_iter().map(|value| Value::from(value)).collect(),
            )),
            _ => unreachable!("Invalid result type for boolean scalar"),
        }
    }

    pub fn resolve_document_uuid_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving UUID Scalar");

        let resolved = ScalarOption::get_from_document(document, field)?;

        match resolved {
            DocumentValue::UUID(value) => Ok(Value::from(value.to_string())),
            DocumentValue::UUIDArray(values) => Ok(Value::List(
                values
                    .into_iter()
                    .map(|value| Value::from(value.to_string()))
                    .collect(),
            )),
            _ => unreachable!("Invalid result type for UUID scalar"),
        }
    }

    pub fn resolve_document_datetime_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving DateTime Scalar");

        let resolved = ScalarOption::get_from_document(document, field)?;

        match resolved {
            // NOTE: Not sure if this is the correct format for DateTime
            DocumentValue::DateTime(value) => Ok(Value::from(value.to_rfc3339())),
            DocumentValue::DateTimeArray(values) => Ok(Value::List(
                values
                    .into_iter()
                    .map(|value| Value::from(value.to_rfc3339()))
                    .collect(),
            )),
            _ => unreachable!("Invalid result type for DateTime scalar"),
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
            debug!("---Found BSON Element Type: {:?}", bson.element_type());

            let field = ServiceEntityFieldConfig::get_field(
                field.fields.clone().unwrap_or(Vec::new()),
                key.clone(),
            )?;
            let value = ServiceEntity::resolve_document_field(document, &field)?;
            index_map.insert(name, value);
        }
        Ok(index_map)
    }

    pub fn resolve_document_object_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving Object Scalar");
        let value = document.get(&field.name);

        if value.is_none() {
            return Ok(Value::Null);
        }

        if field.list.unwrap_or(false) {
            if let Some(Bson::Array(documents)) = document.get(field.name.clone()) {
                let mut values = vec![];
                for document in documents {
                    let value = ServiceEntity::parse_bson_object(document, field)?;
                    values.push(Value::Object(value));
                }

                return Ok(Value::List(values));
            } else {
                return Ok(Value::List(vec![]));
            }
        }

        let index_map = ServiceEntity::parse_bson_object(value.unwrap(), field)?;

        Ok(Value::from(index_map))
    }
}
