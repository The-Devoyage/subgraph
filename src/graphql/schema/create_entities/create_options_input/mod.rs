use async_graphql::dynamic::{Enum, EnumItem, InputObject, InputValue, TypeRef};

use crate::graphql::schema::ServiceSchemaBuilder;

impl ServiceSchemaBuilder {
    pub fn create_options_input(mut self) -> Self {
        // Create the sort/order input list
        let mut sort_input = InputObject::new("sort_input");
        sort_input = sort_input.field(InputValue::new("field", TypeRef::named(TypeRef::STRING)));
        sort_input = sort_input.field(InputValue::new(
            "direction",
            TypeRef::named("sort_direction"),
        ));
        self = self.register_inputs(vec![sort_input]);

        // Create shared input, `options_input`
        let mut root_input = InputObject::new("options_input");
        root_input = root_input.field(InputValue::new("per_page", TypeRef::named(TypeRef::INT)));
        root_input = root_input.field(InputValue::new("page", TypeRef::named(TypeRef::INT)));
        root_input = root_input.field(InputValue::new(
            "sort",
            TypeRef::named_nn_list("sort_input"),
        ));
        self = self.register_inputs(vec![root_input]);

        // Create the order enum
        let mut sort_direction = Enum::new("sort_direction");
        sort_direction = sort_direction.items(vec![EnumItem::new("ASC"), EnumItem::new("DESC")]);
        self = self.register_enums(vec![sort_direction]);

        self
    }
}
