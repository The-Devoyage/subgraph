use async_graphql::dynamic::{InputObject, TypeRef};
use log::debug;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    data_sources::DataSource, resolver_type::ResolverType, scalar_option::ScalarOption,
};

use super::ServiceInput;

pub struct TypeRefWithInputs {
    pub type_ref: TypeRef,
    pub inputs: Vec<InputObject>,
}

mod get_entity_object_nested_inputs;

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
            ScalarOption::Object => {
                let type_ref_with_inputs = ServiceInput::get_entity_object_nested_inputs(
                    entity_field,
                    resolver_type,
                    parent_input_prefix,
                    entity_data_source,
                );

                for input in type_ref_with_inputs {
                    inputs.push(input);
                }

                let input_name = ServiceInput::format_child_field_name(
                    parent_input_prefix,
                    &entity_field.name,
                    resolver_type,
                );

                let type_ref = entity_field
                    .scalar
                    .to_input_type_ref(is_list, is_required, resolver_type, Some(&input_name))
                    .unwrap(); //HACK: should return a Result
                type_ref
            }
            _ => {
                let type_ref = entity_field
                    .scalar
                    .to_input_type_ref(is_list, is_required, resolver_type, None)
                    .unwrap(); //HACK: should return a Result
                type_ref
            }
        };

        TypeRefWithInputs { type_ref, inputs }
    }
}
