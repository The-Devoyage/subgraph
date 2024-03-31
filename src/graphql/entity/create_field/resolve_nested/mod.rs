use async_graphql::{dynamic::ResolverContext, indexmap::IndexMap, Value};
use log::{debug, trace};

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    graphql::entity::ServiceEntity, utils::document::DocumentUtils,
};

mod get_parent_value;

impl ServiceEntity {
    pub fn resolve_nested(
        ctx: &ResolverContext,
        entity_field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolving Nested Field");

        let field_name = ctx.field().name();
        trace!("Field Name: {:?}", field_name);

        let parent_value = match ctx.parent_value.as_value() {
            Some(value) => value,
            None => {
                trace!("Parent is none, returning none");
                return Ok(None);
            }
        };
        trace!("Parent Value: {:?}", parent_value);

        let parent_value = ServiceEntity::get_parent_value(&parent_value, field_name)?;
        trace!("Parent Value: {:?}", parent_value);

        let document = DocumentUtils::json_to_document(&parent_value)?;

        if document.is_none() {
            trace!("Documenet is None, returning none");
            return Ok(None);
        }

        let value = entity_field
            .scalar
            .clone()
            .document_field_to_async_graphql_value(&document.unwrap(), &entity_field)?;

        trace!("Resolved Nested Value: {:?}", value,);

        Ok(value)
    }
}
