use async_graphql::dynamic::{InputObject, TypeRef};
use log::debug;

use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityFieldConfig, ScalarOptions,
    },
    data_sources::DataSource,
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
    /// Get the type ref for the entity field.
    /// If field is an object, it will also create an input objects, recursivly, for the field.
    pub fn get_entity_field_type(
        entity_field: &ServiceEntityFieldConfig,
        resolver_type: &ResolverType,
        parent_input_prefix: &str,
        entity_data_source: &DataSource,
    ) -> TypeRefWithInputs {
        debug!("Creating Entity Field Type For {:?}", entity_field.name);

        let mut inputs = Vec::new();
        let is_list = entity_field.list.unwrap_or(false);
        let is_required = entity_field.required.unwrap_or(false);
        let is_eager = entity_field.eager.unwrap_or(false);

        // If the field is eager, we don't need to create an input for it.
        // Just use the field name as the input.
        if is_eager
            && (resolver_type == &ResolverType::FindMany || resolver_type == &ResolverType::FindOne)
        {
            let as_type_name = entity_field.as_type.clone();
            if as_type_name.is_none() {
                panic!(
                    "Eager field {} must have an as_type defined",
                    entity_field.name
                );
            }
            let input_type_name = format!("get_{}_query_input", as_type_name.unwrap());
            return TypeRefWithInputs {
                type_ref: TypeRef::named(&input_type_name),
                inputs,
            };
        }

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
                    entity_data_source,
                );

                for input in type_ref_with_inputs.inputs {
                    inputs.push(input);
                }

                type_ref_with_inputs.type_ref
            }
            ScalarOptions::UUID => {
                ServiceInput::get_entity_string_field_type(resolver_type, is_list, is_required)
            }
            ScalarOptions::DateTime => {
                ServiceInput::get_entity_string_field_type(resolver_type, is_list, is_required)
            }
        };

        TypeRefWithInputs { type_ref, inputs }
    }
}
