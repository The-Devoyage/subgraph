use async_graphql::dynamic::{InputObject, InputValue};
use log::debug;

use crate::configuration::subgraph::entities::service_entity_field::ServiceEntityField;

use super::schema::{ExcludeFromInput, ResolverType};

mod get_entity_field_type;

pub struct ServiceInput {
    input_name: String,
    fields: Vec<ServiceEntityField>,
    resolver_type: ResolverType,
    exclude_from_input: Option<ExcludeFromInput>,
}

impl ServiceInput {
    pub fn new(
        input_name: String,
        fields: Vec<ServiceEntityField>,
        resolver_type: ResolverType,
        exclude_from_input: Option<ExcludeFromInput>,
    ) -> Self {
        ServiceInput {
            input_name,
            fields,
            resolver_type,
            exclude_from_input,
        }
    }

    pub fn build(self) -> Vec<InputObject> {
        debug!("Creating Input: {:?}", self.input_name);
        let mut inputs = Vec::new();
        let mut input = InputObject::new(&self.input_name);
        let mut excluded_count = 0;

        for field in &self.fields {
            let is_excluded =
                ServiceEntityField::is_excluded_input_field(field, self.exclude_from_input.clone());

            if !is_excluded {
                let parent_input_name = &self.input_name.clone().replace("_input", "");
                let type_ref_with_inputs = ServiceInput::get_entity_field_type(
                    field,
                    &self.resolver_type,
                    &parent_input_name,
                );

                for input in type_ref_with_inputs.inputs {
                    inputs.push(input);
                }

                input = input.field(InputValue::new(
                    field.name.clone(),
                    type_ref_with_inputs.type_ref,
                ));
            } else {
                excluded_count = excluded_count + 1
            }
        }

        if excluded_count != self.fields.len() {
            inputs.push(input);
        }

        debug!("Created Inputs: {:?}", inputs);

        inputs
    }
}
