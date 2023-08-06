use async_graphql::dynamic::{Object, TypeRef};
use log::debug;

use crate::configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig;

use super::ServiceEntity;

mod create_internal_type_field_refs;
mod create_optional_type_refs;
mod create_required_type_refs;

#[derive(Debug)]
pub struct TypeRefsAndDefs {
    pub type_ref: TypeRef,
    pub type_defs: Vec<Object>,
}

impl ServiceEntity {
    pub fn get_field_type_ref(&self, entity_field: &ServiceEntityFieldConfig) -> TypeRefsAndDefs {
        debug!("Creating Field Type Ref And Defs");

        if entity_field.as_type.is_some() {
            return self.create_internal_type_field_refs(entity_field);
        }

        let type_refs_and_defs = match entity_field.required {
            Some(true) => {
                self.create_required_type_refs(&self.entity, entity_field, &self.data_sources)
            }
            _ => self.create_optional_type_refs(&self.entity, entity_field, &self.data_sources),
        };

        debug!("Created Field Type Ref And Defs: {:#?}", type_refs_and_defs);

        type_refs_and_defs
    }
}
