use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    utils::document::{get_from_document::DocumentValue, DocumentUtils},
};
use log::{debug, trace};

use super::ScalarOption;

impl ScalarOption {
    /// Get a value from a document.
    /// Uses the scalar type from the field to determine how to get the value.
    pub fn get_from_document(
        document: &bson::Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!("Getting value from document");
        trace!(
            "Resolving Field {}, of type {:?} in {:?}",
            field.name,
            field.scalar,
            document
        );

        let value = match field.scalar {
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
        };
        trace!("Value: {:?}", value);
        value
    }
}
