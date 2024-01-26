use async_graphql::dynamic::InputObject;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    data_sources::DataSource, graphql::input::ServiceInput, resolver_type::ResolverType,
};

impl ServiceInput {
    pub fn format_child_field_name(
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

    pub fn get_entity_object_nested_inputs(
        entity_field: &ServiceEntityFieldConfig,
        resolver_type: &ResolverType,
        parent_input_prefix: &str,
        entity_data_source: &DataSource,
    ) -> Vec<InputObject> {
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
            entity_data_source.clone(),
        )
        .build(None);

        for input in object_inputs {
            inputs.push(input);
        }

        inputs
    }
}
