use async_graphql::dynamic::TypeRef;
use log::debug;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    graphql::entity::ServiceEntity,
};

use super::TypeRefsAndDefs;

impl ServiceEntity {
    pub fn create_internal_type_field_refs(
        &self,
        entity_field: &ServiceEntityFieldConfig,
    ) -> TypeRefsAndDefs {
        debug!("Creating Internal Type Field Refs");

        let type_ref = if let Some(as_type_name) = &entity_field.as_type.clone() {
            if entity_field.required.unwrap_or(false) {
                match entity_field.list.unwrap_or(false) {
                    true => TypeRef::named_nn_list_nn(as_type_name),
                    false => TypeRef::named_nn(as_type_name),
                }
            } else {
                match entity_field.list.unwrap_or(false) {
                    true => TypeRef::named_list_nn(as_type_name),
                    false => TypeRef::named(as_type_name),
                }
            }
        } else {
            panic!("As Type for Field {} Is Not Defined", entity_field.name)
        };

        TypeRefsAndDefs {
            type_ref,
            type_defs: Vec::new(),
            enum_defs: Vec::new(),
        }
    }
}
