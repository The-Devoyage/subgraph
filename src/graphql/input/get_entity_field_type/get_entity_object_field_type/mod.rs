use async_graphql::dynamic::TypeRef;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    graphql::{input::ServiceInput, schema::ResolverType},
};

use super::TypeRefWithInputs;

impl ServiceInput {
    fn format_child_field_name(
        parent_field_name: &str,
        child_field_name: &str,
        resolver_type: &ResolverType,
    ) -> String {
        match resolver_type {
            ResolverType::FindOne
            | ResolverType::CreateOne
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => {
                format!("{}_{}_input", parent_field_name, child_field_name)
            }
            ResolverType::FindMany => format!("{}_{}s_input", parent_field_name, child_field_name),
            _ => panic!("Invalid resolver type"),
        }
    }

    pub fn get_entity_object_field_type(
        entity_field: &ServiceEntityFieldConfig,
        resolver_type: &ResolverType,
        parent_input_prefix: &str,
    ) -> TypeRefWithInputs {
        let mut inputs = Vec::new();

        let input_name = ServiceInput::format_child_field_name(
            parent_input_prefix,
            &entity_field.name,
            resolver_type,
        );

        let object_inputs = ServiceInput::new(
            input_name.clone(),
            entity_field.fields.clone().unwrap_or(Vec::new()),
            resolver_type.clone(),
            None,
        )
        .build();

        for input in object_inputs {
            inputs.push(input);
        }

        let type_ref = match resolver_type {
            ResolverType::FindOne
            | ResolverType::FindMany
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => {
                if entity_field.list == Some(true) {
                    TypeRef::named_nn_list(input_name)
                } else {
                    TypeRef::named(input_name)
                }
            }
            ResolverType::CreateOne => match entity_field.required {
                Some(true) => {
                    if entity_field.list == Some(true) {
                        TypeRef::named_nn_list_nn(input_name)
                    } else {
                        TypeRef::named_nn(input_name)
                    }
                }
                _ => {
                    if entity_field.list == Some(true) {
                        TypeRef::named_nn_list_nn(input_name)
                    } else {
                        TypeRef::named(input_name)
                    }
                }
            },
            _ => panic!("Invalid resolver type"),
        };

        TypeRefWithInputs { type_ref, inputs }
    }
}
