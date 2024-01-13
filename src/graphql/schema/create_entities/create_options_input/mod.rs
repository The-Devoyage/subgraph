use async_graphql::dynamic::{Enum, EnumItem, InputObject, InputValue, TypeRef};

use crate::graphql::schema::ServiceSchemaBuilder;

impl ServiceSchemaBuilder {
    pub fn create_options_input(mut self) -> Self {
        // Create shared input, `options_input`
        let mut root_input = InputObject::new("options_input");
        root_input = root_input.field(InputValue::new("per_page", TypeRef::named(TypeRef::INT)));
        root_input = root_input.field(InputValue::new("page", TypeRef::named(TypeRef::INT)));
        root_input = root_input.field(InputValue::new("sort", TypeRef::named(TypeRef::STRING)));
        root_input = root_input.field(InputValue::new("order", TypeRef::named("order_enum")));
        self = self.register_inputs(vec![root_input]);

        // Create the order enum
        let mut order_enum = Enum::new("order_enum");
        order_enum = order_enum.items(vec![EnumItem::new("ASC"), EnumItem::new("DESC")]);
        self = self.register_enums(vec![order_enum]);

        self
    }
}
