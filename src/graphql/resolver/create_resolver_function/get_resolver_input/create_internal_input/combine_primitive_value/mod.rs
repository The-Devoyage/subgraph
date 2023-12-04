use crate::{configuration::subgraph::entities::ScalarOptions, graphql::resolver::ServiceResolver};
use bson::Document;
use log::{debug, error};

impl ServiceResolver {
    /// Extracts the primitive value from the input query and combines it with the parent value.
    pub fn combine_primitive_value(
        parent_value: &Document,       // The parent value, with data from the ds.
        query_document: &mut Document, // The user provided input
        field_name: &str,
        scalar: &ScalarOptions,
        join_on: &str,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Combining Primitive Value With Input");
        debug!("Parent Value: {:?}", parent_value);
        debug!("Join On: {} as {:?}", join_on, scalar);

        match scalar {
            ScalarOptions::String | ScalarOptions::UUID | ScalarOptions::DateTime => {
                let join_on_value = parent_value.get_str(&field_name);

                if join_on_value.is_ok() {
                    let join_on_value = join_on_value.unwrap();
                    query_document.insert(join_on, join_on_value);
                }
            }
            ScalarOptions::Int => {
                let join_on_value = parent_value.get_i64(&field_name);
                if join_on_value.is_ok() {
                    let join_on_value = join_on_value.unwrap();
                    query_document.insert(join_on, join_on_value);
                }
            }
            ScalarOptions::Boolean => {
                let join_on_value = parent_value.get_bool(&field_name);
                if join_on_value.is_ok() {
                    let join_on_value = join_on_value.unwrap();
                    query_document.insert(join_on, join_on_value);
                }
            }
            ScalarOptions::ObjectID => {
                let join_on_value = parent_value.get_object_id(&field_name);
                if join_on_value.is_ok() {
                    let join_on_value = join_on_value.unwrap();
                    query_document.insert(join_on, join_on_value);
                } else {
                    // If sql dialect, attempt to get from string
                    let join_on_value = parent_value.get_str(&field_name);
                    if join_on_value.is_ok() {
                        let join_on_value = join_on_value.unwrap();
                        query_document.insert(join_on, join_on_value);
                    }
                }
            }
            _ => {
                error!("Unsupported scalar type: {:?}", scalar);
                return Err(async_graphql::Error::new(
                    "Failed to create internally joined query. Unsupported scalar type.",
                ));
            }
        };

        debug!("Joined Query: {:?}", query_document);
        Ok(query_document.clone())
    }
}
