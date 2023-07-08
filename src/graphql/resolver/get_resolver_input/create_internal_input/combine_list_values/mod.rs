use std::str::FromStr;

use bson::{oid::ObjectId, Document};
use log::{debug, error};

use crate::{configuration::subgraph::entities::ScalarOptions, graphql::resolver::ServiceResolver};

impl ServiceResolver {
    pub fn combine_list_values(
        parent_value: &Document,
        field_input: &mut Document,
        field_name: &str,
        scalar: &ScalarOptions,
        join_on: &str,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Combining List Values With Input");
        let join_on_value = parent_value.get_array(field_name.clone());
        let join_on_value = match join_on_value {
            Ok(join_on_value) => join_on_value,
            Err(_) => {
                return Err(async_graphql::Error::new(format!(
                    "Field {} not found.",
                    field_name
                )))
            }
        };
        match scalar {
            ScalarOptions::String => {
                let join_on_value = join_on_value
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>();
                field_input.insert(join_on.clone(), join_on_value);
            }
            ScalarOptions::Int => {
                if join_on_value.iter().any(|value| value.as_i32().is_none()) {
                    error!("Field {} not found. Invalid Int", field_name);
                    return Err(async_graphql::Error::new(format!(
                        "Field {} not found. Invalid Int",
                        field_name
                    )));
                }

                let join_on_value = join_on_value
                    .iter()
                    .map(|value| value.as_i32().unwrap())
                    .collect::<Vec<i32>>();
                field_input.insert(join_on.clone(), join_on_value);
            }
            ScalarOptions::Boolean => {
                if join_on_value.iter().any(|value| value.as_bool().is_none()) {
                    error!("Field {} not found. Invalid Boolean", field_name);
                    return Err(async_graphql::Error::new(format!(
                        "Field {} not found. Invalid Boolean",
                        field_name
                    )));
                }
                let join_on_value = join_on_value
                    .iter()
                    .map(|value| value.as_bool().unwrap())
                    .collect::<Vec<bool>>();
                field_input.insert(join_on.clone(), join_on_value);
            }
            ScalarOptions::ObjectID => {
                if join_on_value.iter().any(|value| value.as_str().is_none()) {
                    error!("Field {} not found. Invalid ObjectID", field_name);
                    return Err(async_graphql::Error::new(format!(
                        "Field {} not found. Invalid ObjectID",
                        field_name
                    )));
                }
                if join_on_value
                    .iter()
                    .any(|value| ObjectId::from_str(value.as_str().unwrap()).is_err())
                {
                    error!(
                        "Field {} not found. Failed to convert to Object ID.",
                        field_name
                    );
                    return Err(async_graphql::Error::new(format!(
                        "Field {} not found. Invalid ObjectID",
                        field_name
                    )));
                }
                let join_on_value = join_on_value
                    .iter()
                    .map(|value| ObjectId::from_str(value.as_str().unwrap()).unwrap())
                    .collect::<Vec<ObjectId>>();
                field_input.insert(join_on.clone(), join_on_value);
            }
            _ => panic!("Invalid Scalar Type"),
        };
        Ok(field_input.clone())
    }
}
