use async_graphql::dynamic::{InputObject, TypeRef};
use log::debug;

use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityFieldConfig, ScalarOptions,
    },
    graphql::schema::ResolverType,
};

use super::ServiceInput;

pub struct TypeRefWithInputs {
    pub type_ref: TypeRef,
    pub inputs: Vec<InputObject>,
}

mod get_entity_bool_field_type;
mod get_entity_int_field_type;
mod get_entity_object_field_type;
mod get_entity_object_id_field_type;
mod get_entity_string_field_type;

impl ServiceInput {
    pub fn get_entity_field_type(
        entity_field: &ServiceEntityFieldConfig,
        resolver_type: &ResolverType,
        parent_input_prefix: &str,
    ) -> TypeRefWithInputs {
        debug!("Creating Entity Field Type For {:?}", entity_field.name);

        let mut inputs = Vec::new();
        let is_list = entity_field.list.unwrap_or(false);
        let is_required = entity_field.required.unwrap_or(false);

        let type_ref = match &entity_field.scalar {
            ScalarOptions::String => {
                ServiceInput::get_entity_string_field_type(resolver_type, is_list, is_required)
            }
            ScalarOptions::Int => {
                ServiceInput::get_entity_int_field_type(resolver_type, is_list, is_required)
            }
            ScalarOptions::Boolean => {
                ServiceInput::get_entity_bool_field_type(resolver_type, is_list, is_required)
            }
            ScalarOptions::ObjectID => {
                ServiceInput::get_entity_object_id_field_type(resolver_type, is_list, is_required)
            }
            ScalarOptions::Object => {
                let type_ref_with_inputs = ServiceInput::get_entity_object_field_type(
                    entity_field,
                    resolver_type,
                    parent_input_prefix,
                );

                for input in type_ref_with_inputs.inputs {
                    inputs.push(input);
                }

                type_ref_with_inputs.type_ref
            }
        };

        TypeRefWithInputs { type_ref, inputs }
    }
}
