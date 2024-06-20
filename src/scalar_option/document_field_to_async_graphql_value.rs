use async_graphql::Value;
use bson::Document;
use log::{debug, trace};

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    graphql::entity::ServiceEntity,
};

use super::ScalarOption;

impl ScalarOption {
    /// Converts a document field to a async_graphql Value
    pub fn document_field_to_async_graphql_value(
        self,
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolving Document Field: {:?}", field.name);

        let value = match self {
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
            ScalarOption::Enum => ServiceEntity::resolve_document_enum_scalar(document, field),
        };

        trace!("Resolved Document Field: {:?}", field.name);

        value
    }
}
