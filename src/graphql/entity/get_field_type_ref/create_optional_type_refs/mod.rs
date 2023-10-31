use async_graphql::dynamic::TypeRef;
use log::debug;

use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityFieldConfig, ScalarOptions, ServiceEntityConfig,
    },
    data_sources::DataSources,
    graphql::entity::ServiceEntity,
};

use super::TypeRefsAndDefs;

impl ServiceEntity {
    pub fn create_optional_type_refs(
        &self,
        entity: &ServiceEntityConfig,
        entity_field: &ServiceEntityFieldConfig,
        data_sources: &DataSources,
    ) -> TypeRefsAndDefs {
        debug!("Creating Optional Type Refs");
        let mut type_defs = Vec::new();

        let type_ref = match entity_field.scalar.clone() {
            ScalarOptions::String => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named(TypeRef::STRING)
                }
            }
            ScalarOptions::Int => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(TypeRef::INT)
                } else {
                    TypeRef::named(TypeRef::INT)
                }
            }
            ScalarOptions::Boolean => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(TypeRef::BOOLEAN)
                } else {
                    TypeRef::named(TypeRef::BOOLEAN)
                }
            }
            ScalarOptions::ObjectID => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn("ObjectID")
                } else {
                    TypeRef::named("ObjectID")
                }
            }
            ScalarOptions::Object => {
                let object_type_defs = ServiceEntity::new(
                    data_sources.clone(),
                    entity.clone(),
                    entity_field.name.clone(),
                    entity_field.fields.clone().unwrap_or(vec![]),
                    self.subgraph_config.clone(),
                    Some(false),
                )
                .build();

                for object in object_type_defs {
                    type_defs.push(object)
                }

                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(entity_field.name.clone())
                } else {
                    TypeRef::named(entity_field.name.clone())
                }
            }
            ScalarOptions::UUID => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named(TypeRef::STRING)
                }
            }
            ScalarOptions::DateTime => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named(TypeRef::STRING)
                }
            }
        };

        TypeRefsAndDefs {
            type_ref,
            type_defs,
        }
    }
}
