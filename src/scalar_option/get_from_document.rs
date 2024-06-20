use crate::utils::document::{get_from_document::DocumentValue, DocumentUtils};
use log::{debug, trace};

use super::ScalarOption;

impl ScalarOption {
    /// Get a DocumentValue from a document.
    /// Uses the scalar type from the field to determine how to get the value.
    pub fn get_from_document(
        &self,
        document: &bson::Document,
        field_name: &str,
        is_list: bool,
    ) -> Result<DocumentValue, async_graphql::Error> {
        debug!("Getting value from document");
        trace!(
            "Resolving Field {}, of type {:?} in {:?}",
            field_name,
            self,
            document
        );

        let value = match self {
            ScalarOption::String => {
                DocumentUtils::get_document_string_scalar(document, &field_name, is_list)
            }
            ScalarOption::Int => {
                DocumentUtils::get_document_int_scalar(document, &field_name, is_list)
            }
            ScalarOption::Boolean => {
                DocumentUtils::get_document_boolean_scalar(document, &field_name, is_list)
            }
            ScalarOption::ObjectID => {
                DocumentUtils::get_document_object_id_scalar(document, &field_name, is_list)
            }
            ScalarOption::Object => {
                DocumentUtils::get_document_object_scalar(document, &field_name, is_list)
            }
            ScalarOption::UUID => {
                DocumentUtils::get_document_uuid_scalar(document, &field_name, is_list)
            }
            ScalarOption::DateTime => {
                DocumentUtils::get_document_datetime_scalar(document, &field_name, is_list)
            }
            ScalarOption::Enum => {
                DocumentUtils::get_document_enum_scalar(document, &field_name, is_list)
            }
        };
        trace!("Value: {:?}", value);
        value
    }
}
