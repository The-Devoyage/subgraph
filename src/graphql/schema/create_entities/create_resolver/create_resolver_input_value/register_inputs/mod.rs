use async_graphql::dynamic::InputObject;
use log::debug;

use crate::graphql::schema::ServiceSchema;

impl ServiceSchema {
    pub fn register_inputs(mut self, inputs: Vec<InputObject>) -> Self {
        debug!("Registering Inputs");

        for input in inputs {
            debug!("Registering Input: {}", input.type_name());
            self.schema_builder = self.schema_builder.register(input);
        }

        self
    }
}
