use async_graphql::dynamic::{InputObject, InputValue, TypeRef};

use crate::graphql::schema::ServiceSchemaBuilder;

impl ServiceSchemaBuilder {
    pub fn create_options_input(mut self) -> Self {
        // Create shared input, `options_input`
        let mut root_input = InputObject::new("options_input");
        root_input = root_input.field(InputValue::new("per_page", TypeRef::named(TypeRef::INT)));
        root_input = root_input.field(InputValue::new("page", TypeRef::named(TypeRef::INT)));

        self = self.register_inputs(vec![root_input]);
        self
    }
}
