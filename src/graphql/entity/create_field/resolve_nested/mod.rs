use async_graphql::{dynamic::ResolverContext, indexmap::IndexMap, Value};
use log::debug;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    graphql::entity::ServiceEntity, scalar_option::ScalarOption, utils::document::DocumentUtils,
};

mod get_parent_value;

impl ServiceEntity {
    pub fn resolve_nested(
        ctx: &ResolverContext,
        entity_field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        let field_name = ctx.field().name();

        debug!("Resolving Nested Field: {:?}", &field_name);

        let parent_value = match ctx.parent_value.as_value() {
            Some(value) => value,
            None => {
                debug!("Parent Value is None, creating empty IndexMap");
                let index_map = IndexMap::new();
                return Ok(Some(Value::from(index_map)));
            }
        };

        let parent_value = ServiceEntity::get_parent_value(&parent_value, field_name)?;

        let document = DocumentUtils::json_to_document(&parent_value)?;

        let value = ScalarOption::resolve_document_field(&document, &entity_field)?;

        debug!("Resolved Nested Value: {:?}", value,);

        Ok(Some(value))
    }
}
