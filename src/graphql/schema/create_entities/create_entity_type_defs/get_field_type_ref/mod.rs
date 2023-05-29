use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityField, ScalarOptions, ServiceEntity,
    },
    data_sources::DataSources,
};

use super::{ServiceSchemaBuilder, TypeRefsAndDefs};
use async_graphql::dynamic::TypeRef;
use log::debug;

impl ServiceSchemaBuilder {
    pub fn get_field_type_ref(
        &self,
        entity_field: &ServiceEntityField,
        data_sources: &DataSources,
        entity: &ServiceEntity,
    ) -> TypeRefsAndDefs {
        debug!("Getting Field Type Ref And Defs");

        if entity_field.as_type.is_some() {
            return self.create_internal_type_field_refs(entity_field);
        }

        let type_refs_and_defs = match entity_field.required {
            Some(true) => self.create_required_type_refs(entity, entity_field, data_sources),
            _ => self.create_optional_type_refs(entity, entity_field, data_sources),
        };

        debug!(
            "Finished Getting Field Type Ref And Defs: {:#?}",
            type_refs_and_defs
        );

        type_refs_and_defs
    }

    pub fn create_internal_type_field_refs(
        &self,
        entity_field: &ServiceEntityField,
    ) -> TypeRefsAndDefs {
        debug!("Creating Internal Type Field Refs");

        let type_ref = if let Some(as_type_name) = &entity_field.as_type.clone() {
            if entity_field.required.is_some() && entity_field.required.unwrap() {
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
            is_root_object: true,
        }
    }

    pub fn create_required_type_refs(
        &self,
        entity: &ServiceEntity,
        entity_field: &ServiceEntityField,
        data_sources: &DataSources,
    ) -> TypeRefsAndDefs {
        debug!("Creating Required Type Refs");

        let mut type_defs = Vec::new();
        let mut is_root_object = true;

        let type_ref = match entity_field.scalar.clone() {
            ScalarOptions::String => {
                if entity_field.list.is_some() && entity_field.list.unwrap() {
                    TypeRef::named_nn_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named_nn(TypeRef::STRING)
                }
            }
            ScalarOptions::Int => {
                if entity_field.list.is_some() && entity_field.list.unwrap() {
                    TypeRef::named_nn_list_nn(TypeRef::INT)
                } else {
                    TypeRef::named_nn(TypeRef::INT)
                }
            }
            ScalarOptions::Boolean => {
                if entity_field.list.is_some() && entity_field.list.unwrap() {
                    TypeRef::named_nn_list_nn(TypeRef::BOOLEAN)
                } else {
                    TypeRef::named_nn(TypeRef::BOOLEAN)
                }
            }
            ScalarOptions::ObjectID => {
                if entity_field.list.is_some() && entity_field.list.unwrap() {
                    TypeRef::named_nn_list_nn("ObjectID")
                } else {
                    TypeRef::named_nn("ObjectID")
                }
            }
            ScalarOptions::Object => {
                let object_type_defs = self.create_type_defs(
                    data_sources,
                    entity,
                    entity_field.name.clone(),
                    entity_field.fields.clone().unwrap_or(Vec::new()),
                );

                for object in object_type_defs {
                    type_defs.push(object);
                }

                is_root_object = false;

                if entity_field.list.is_some() && entity_field.list.unwrap() {
                    TypeRef::named_nn_list_nn(entity_field.name.clone())
                } else {
                    TypeRef::named_nn(entity_field.name.clone())
                }
            }
        };

        TypeRefsAndDefs {
            type_ref,
            type_defs,
            is_root_object,
        }
    }

    pub fn create_optional_type_refs(
        &self,
        entity: &ServiceEntity,
        entity_field: &ServiceEntityField,
        data_sources: &DataSources,
    ) -> TypeRefsAndDefs {
        debug!("Creating Optional Type Refs");
        let mut type_defs = Vec::new();

        let mut is_root_object = true;

        let type_ref = match entity_field.scalar.clone() {
            ScalarOptions::String => {
                if entity_field.list.is_some() && entity_field.list.unwrap() {
                    TypeRef::named_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named(TypeRef::STRING)
                }
            }
            ScalarOptions::Int => {
                if entity_field.list.is_some() && entity_field.list.unwrap() {
                    TypeRef::named_list_nn(TypeRef::INT)
                } else {
                    TypeRef::named(TypeRef::INT)
                }
            }
            ScalarOptions::Boolean => {
                if entity_field.list.is_some() && entity_field.list.unwrap() {
                    TypeRef::named_list_nn(TypeRef::BOOLEAN)
                } else {
                    TypeRef::named(TypeRef::BOOLEAN)
                }
            }
            ScalarOptions::ObjectID => {
                if entity_field.list.is_some() && entity_field.list.unwrap() {
                    TypeRef::named_list_nn("ObjectID")
                } else {
                    TypeRef::named("ObjectID")
                }
            }
            ScalarOptions::Object => {
                let object_type_defs = self.create_type_defs(
                    data_sources,
                    entity,
                    entity_field.name.clone(),
                    entity_field.fields.clone().unwrap_or(vec![]),
                );

                for object in object_type_defs {
                    type_defs.push(object)
                }

                is_root_object = false;

                if entity_field.list.is_some() && entity_field.list.unwrap() {
                    TypeRef::named_list_nn(entity_field.name.clone())
                } else {
                    TypeRef::named(entity_field.name.clone())
                }
            }
        };

        TypeRefsAndDefs {
            type_ref,
            type_defs,
            is_root_object,
        }
    }
}
