use async_graphql::Value;
use bson::Document;
use log::{debug, trace};

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    graphql::entity::ServiceEntity,
};

use super::ScalarOption;

impl ScalarOption {
    pub fn resolve_document_field(
        document: &Document,
        field: &ServiceEntityFieldConfig,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Resolving Document Field: {:?}", field.name);

        let value = match &field.scalar {
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
        };

        trace!("Resolved Document Field: {:?}", field.name);

        value
    }
}
