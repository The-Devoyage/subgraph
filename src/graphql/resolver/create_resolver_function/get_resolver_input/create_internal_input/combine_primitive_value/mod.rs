use crate::{configuration::subgraph::entities::ScalarOptions, graphql::resolver::ServiceResolver};
use bson::Document;
use log::debug;

impl ServiceResolver {
    pub fn combine_primitive_value(
        parent_value: &Document,
        field_input_query: &mut Document,
        field_name: &str,
        scalar: &ScalarOptions,
        join_on: &str,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Combining Primitive Value With Input");
        debug!("Parent Value: {:?}", parent_value);
        debug!("Join On: {}", join_on);

        match scalar {
            ScalarOptions::String | ScalarOptions::UUID | ScalarOptions::DateTime => {
                let join_on_value = parent_value.get_str(&field_name);

                if join_on_value.is_ok() {
                    let join_on_value = join_on_value.unwrap();
                    field_input_query.insert(join_on, join_on_value);
                }
            }
            ScalarOptions::Int => {
                let join_on_value = parent_value.get_i32(&field_name);
                if join_on_value.is_ok() {
                    let join_on_value = join_on_value.unwrap();
                    field_input_query.insert(join_on, join_on_value);
                }
            }
            ScalarOptions::Boolean => {
                let join_on_value = parent_value.get_bool(&field_name);
                if join_on_value.is_ok() {
                    let join_on_value = join_on_value.unwrap();
                    field_input_query.insert(join_on, join_on_value);
                }
            }
            ScalarOptions::ObjectID => {
                let join_on_value = parent_value.get_object_id(&field_name);
                if join_on_value.is_ok() {
                    let join_on_value = join_on_value.unwrap();
                    field_input_query.insert(join_on, join_on_value);
                }
            }
            _ => return Err(async_graphql::Error::new("Invalid Scalar Type")),
        };

        debug!("Field Input Query: {:?}", field_input_query);
        Ok(field_input_query.clone())
    }
}
