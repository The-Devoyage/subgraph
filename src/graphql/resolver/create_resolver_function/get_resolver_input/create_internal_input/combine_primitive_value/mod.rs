use std::str::FromStr;

use crate::{configuration::subgraph::entities::ScalarOptions, graphql::resolver::ServiceResolver};
use bson::{oid::ObjectId, Document};
use log::{debug, error};

impl ServiceResolver {
    pub fn combine_primitive_value(
        parent_value: &Document,
        field_input: &mut Document,
        field_name: &str,
        scalar: &ScalarOptions,
        join_on: &str,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Combining Primitive Value With Input");
        debug!("Parent Value: {:?}", parent_value);

        match scalar {
            ScalarOptions::Int => {
                let join_on_value = parent_value.get_i32(&field_name);
                let join_on_value = match join_on_value {
                    Ok(join_on_value) => join_on_value,
                    Err(_) => {
                        error!("Field {} not found. Invalid Int", field_name);
                        return Err(async_graphql::Error::new(format!(
                            "Field {} not found. Invalid Int",
                            field_name
                        )));
                    }
                };
                field_input.insert(join_on, join_on_value);
            }
            ScalarOptions::String => {
                let join_on_value = parent_value.get_str(&field_name);
                let join_on_value = match join_on_value {
                    Ok(join_on_value) => join_on_value,
                    Err(_) => {
                        error!("Field {} not found. Invalid String", field_name);
                        return Err(async_graphql::Error::new(format!(
                            "Field {} not found. Invalid String",
                            field_name
                        )));
                    }
                };
                field_input.insert(join_on, join_on_value);
            }
            ScalarOptions::Boolean => {
                let join_on_value = parent_value.get_bool(&field_name);
                let join_on_value = match join_on_value {
                    Ok(join_on_value) => join_on_value,
                    Err(_) => {
                        error!("Field {} not found. Invalid Boolean", field_name);
                        return Err(async_graphql::Error::new(format!(
                            "Field {} not found. Invalid Boolean",
                            field_name
                        )));
                    }
                };
                field_input.insert(join_on, join_on_value);
            }
            ScalarOptions::ObjectID => {
                let join_on_value = parent_value.get_object_id(&field_name);
                let join_on_value = match join_on_value {
                    Ok(join_on_value) => join_on_value,
                    Err(_) => {
                        let strign_object_id = parent_value.get_str(field_name)?;
                        let join_on_value = ObjectId::from_str(strign_object_id)?;
                        join_on_value
                    }
                };
                field_input.insert(join_on, join_on_value);
            }
            _ => return Err(async_graphql::Error::new("Invalid Scalar Type")),
        };

        Ok(field_input.clone())
    }
}
