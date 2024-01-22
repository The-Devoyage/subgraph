use async_graphql::dynamic::TypeRef;
use log::debug;

use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityFieldConfig, ServiceEntityConfig,
    },
    data_sources::DataSources,
    graphql::entity::ServiceEntity,
    scalar_option::ScalarOption,
};

use super::TypeRefsAndDefs;

impl ServiceEntity {
    pub fn create_required_type_refs(
        &self,
        entity: &ServiceEntityConfig,
        entity_field: &ServiceEntityFieldConfig,
        data_sources: &DataSources,
    ) -> TypeRefsAndDefs {
        debug!("Creating Required Type Refs");

        let mut type_defs = Vec::new();

        let type_ref = match entity_field.scalar.clone() {
            ScalarOption::String => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_nn_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named_nn(TypeRef::STRING)
                }
            }
            ScalarOption::Int => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_nn_list_nn(TypeRef::INT)
                } else {
                    TypeRef::named_nn(TypeRef::INT)
                }
            }
            ScalarOption::Boolean => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_nn_list_nn(TypeRef::BOOLEAN)
                } else {
                    TypeRef::named_nn(TypeRef::BOOLEAN)
                }
            }
            ScalarOption::ObjectID => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_nn_list_nn("ObjectID")
                } else {
                    TypeRef::named_nn("ObjectID")
                }
            }
            ScalarOption::Object => {
                let object_type_defs = ServiceEntity::new(
                    data_sources.clone(),
                    entity.clone(),
                    entity_field.name.clone(),
                    entity_field.fields.clone().unwrap_or(Vec::new()),
                    self.subgraph_config.clone(),
                    Some(false),
                )
                .build();

                for object in object_type_defs {
                    type_defs.push(object);
                }

                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_nn_list_nn(entity_field.name.clone())
                } else {
                    TypeRef::named_nn(entity_field.name.clone())
                }
            }
            ScalarOption::UUID => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_nn_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named_nn(TypeRef::STRING)
                }
            }
            ScalarOption::DateTime => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_nn_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named_nn(TypeRef::STRING)
                }
            }
        };

        TypeRefsAndDefs {
            type_ref,
            type_defs,
        }
    }
}
