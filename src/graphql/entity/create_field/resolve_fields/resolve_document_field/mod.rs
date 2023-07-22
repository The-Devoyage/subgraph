use async_graphql::{indexmap::IndexMap, Name, Value};
use bson::{Bson, Document};
use log::debug;

use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityFieldConfig, ScalarOptions,
    },
    graphql::entity::ServiceEntity,
    utils::document::{get_from_document::GetDocumentResultType, DocumentUtils},
};

impl ServiceEntity {
    pub fn resolve_document_field(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving Document Field: {:?}", field.name);

        match &field.scalar {
            ScalarOptions::String | ScalarOptions::ObjectID => {
                ServiceEntity::resolve_document_string_scalar(document, field)
            }
            ScalarOptions::Int => ServiceEntity::resolve_document_int_scalar(document, field),
            ScalarOptions::Boolean => {
                ServiceEntity::resolve_document_boolean_scalar(document, field)
            }
            ScalarOptions::Object => ServiceEntity::resolve_document_object_scalar(document, field),
        }
    }

    pub fn resolve_document_string_scalar(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving String Scalar");
        let resolved = DocumentUtils::get_from_document(document, field)?;

        match resolved {
            GetDocumentResultType::String(value) => Ok(Value::from(value)),
            GetDocumentResultType::StringArray(values) => Ok(Value::List(
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

        let resolved = DocumentUtils::get_from_document(document, field)?;

        match resolved {
            GetDocumentResultType::Int(value) => Ok(Value::from(value)),
            GetDocumentResultType::IntArray(values) => Ok(Value::List(
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

        let resolved = DocumentUtils::get_from_document(document, field)?;

        match resolved {
            GetDocumentResultType::Boolean(value) => Ok(Value::from(value)),
            GetDocumentResultType::BooleanArray(values) => Ok(Value::List(
                values.into_iter().map(|value| Value::from(value)).collect(),
            )),
            _ => unreachable!("Invalid result type for boolean scalar"),
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
