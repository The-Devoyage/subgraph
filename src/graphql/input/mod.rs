use async_graphql::dynamic::{InputObject, InputValue, TypeRef};
use log::debug;

use crate::{
    configuration::subgraph::entities::service_entity_field::{
        exclude_from_input::ExcludeFromInput, ServiceEntityFieldConfig,
    },
    data_sources::DataSource,
    filter_operator::FilterOperator,
    resolver_type::ResolverType,
};

mod get_entity_field_type;

pub struct ServiceInput {
    input_name: String,
    fields: Vec<ServiceEntityFieldConfig>,
    resolver_type: ResolverType,
    exclude_from_input: Option<ExcludeFromInput>,
    entity_data_source: DataSource,
}

impl ServiceInput {
    pub fn new(
        input_name: String,
        fields: Vec<ServiceEntityFieldConfig>,
        resolver_type: ResolverType,
        exclude_from_input: Option<ExcludeFromInput>,
        entity_data_source: DataSource,
    ) -> Self {
        ServiceInput {
            input_name,
            fields,
            resolver_type,
            exclude_from_input,
            entity_data_source,
        }
    }

    pub fn build(self, include_filters: Option<bool>) -> Vec<InputObject> {
        debug!("Creating Input: {:?}", self.input_name);
        let mut inputs = Vec::new();

        // Create the main input object.
        let mut input = InputObject::new(&self.input_name);
        let mut excluded_count = 0; // Track excluded count, if all excluded, don't create input.

        // For each field in the entity, create an input field.
        for field in &self.fields {
            let is_excluded = ServiceEntityFieldConfig::is_excluded_input_field(
                field,
                self.exclude_from_input.clone(),
            );

            if !is_excluded {
                let parent_input_name = &self.input_name.clone().replace("_input", "");

                // Get the type refs and the inputs for the field.
                // This will recursively create inputs for nested fields.
                let type_ref_with_inputs = ServiceInput::get_entity_field_type(
                    field,
                    &self.resolver_type,
                    &parent_input_name,
                    &self.entity_data_source,
                );

                // Push inputs into vec to register all at once after creation.
                for input in type_ref_with_inputs.inputs {
                    inputs.push(input);
                }

                // Add the input field to the input object.
                input = input.field(InputValue::new(
                    field.name.clone(),
                    type_ref_with_inputs.type_ref,
                ));
            } else {
                excluded_count = excluded_count + 1
            }
        }

        // Only add filter inputs for specific resolvers.
        let include_filters = include_filters.unwrap_or(false);

        // Only add filter inputs for specific DataSources.
        let include_filters = match self.entity_data_source {
            DataSource::SQL(_) | DataSource::Mongo(_) => include_filters,
            DataSource::HTTP(_) => false,
        };

        // If include_filters is true, add the filter inputs.
        if include_filters {
            let filter_operators = FilterOperator::list();
            for filter_operator in filter_operators {
                input = input.field(InputValue::new(
                    filter_operator.as_str(),
                    TypeRef::named_nn_list(&self.input_name),
                ));
            }
        }

        // If all fields are excluded, don't add the input.
        if excluded_count != self.fields.len() {
            inputs.push(input);
        }

        debug!("Created Inputs: {:?}", inputs);

        inputs
    }
}
