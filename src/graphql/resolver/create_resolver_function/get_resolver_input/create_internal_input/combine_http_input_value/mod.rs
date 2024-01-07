use bson::{oid::ObjectId, Document};
use log::{debug, error, trace};

use crate::{configuration::subgraph::entities::ScalarOptions, graphql::resolver::ServiceResolver};

impl ServiceResolver {
    pub fn combine_http_input_value(
        parent_value: &Document,
        query_document: &mut Document,
        field_name: &str,
        scalar: &ScalarOptions,
        join_on: &str,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Combining HTTP Input Value");

        // Determine if the value provided is an array/vec or not.
        let is_list = match parent_value.get_array(&field_name) {
            Ok(_) => true,
            Err(_) => false,
        };

        // Replace the key of the input with the correct key to join on.
        // Map the value to the correct type based on the scalar
        match scalar {
            ScalarOptions::String | ScalarOptions::UUID | ScalarOptions::DateTime => {
                if is_list {
                    // Check that all values in array are of type string.
                    let valid_strings = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .all(|v| v.as_str().is_some());

                    if !valid_strings {
                        error!(
                            "Invalid value provided for field: {}. Value is not of type `string`.",
                            field_name
                        );
                        return Err(async_graphql::Error::from(format!(
                            "Invalid value provided for field: {}. All values are not of type `string`.",
                            field_name
                        )));
                    }

                    let join_on_value = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .map(|v| v.as_str().unwrap().to_string())
                        .collect::<Vec<String>>();

                    query_document.insert(join_on, join_on_value);
                } else {
                    // Check that value is of type string.
                    if parent_value.get_str(&field_name).is_err() {
                        error!("Invalid value provided for field: {}", field_name);
                        return Err(async_graphql::Error::from(format!(
                            "Invalid value provided for field: {}. Value is not of type `string`.",
                            field_name
                        )));
                    }

                    let join_on_value = parent_value.get_str(&field_name).unwrap().to_string();

                    query_document.insert(join_on, join_on_value);
                }
            }
            ScalarOptions::Int => {
                if is_list {
                    // Check that all values in array are of type i32.
                    let valid_ints = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .all(|v| v.as_i32().is_some());

                    if !valid_ints {
                        error!(
                            "Invalid value provided for field: {}. Value is not of type `int`.",
                            field_name
                        );
                        return Err(async_graphql::Error::from(format!(
                            "Invalid value provided for field: {}. All values are not of type `int`.",
                            field_name
                        )));
                    }

                    // Convert to i64
                    let join_on_value = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .map(|v| v.as_i32().unwrap() as i64)
                        .collect::<Vec<i64>>();

                    query_document.insert(join_on, join_on_value);
                } else {
                    // Check that value is of type i32.
                    if parent_value.get_i32(&field_name).is_err() {
                        error!("Invalid value provided for field: {}", field_name);
                        return Err(async_graphql::Error::from(format!(
                            "Invalid value provided for field: {}. Value is not of type `int`.",
                            field_name
                        )));
                    }

                    let join_on_value = ServiceResolver::get_int_value(parent_value, field_name)?;

                    query_document.insert(join_on, join_on_value);
                }
            }
            ScalarOptions::Boolean => {
                if is_list {
                    // Check that all values in array are of type bool.
                    let valid_bools = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .all(|v| v.as_bool().is_some());

                    if !valid_bools {
                        error!(
                            "Invalid value provided for field: {}. Value is not of type `bool`.",
                            field_name
                        );
                        return Err(async_graphql::Error::from(format!(
                            "Invalid value provided for field: {}. All values are not of type `bool`.",
                            field_name
                        )));
                    }

                    let join_on_value = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .map(|v| v.as_bool().unwrap())
                        .collect::<Vec<bool>>();

                    query_document.insert(join_on, join_on_value);
                } else {
                    // Check that value is of type bool.
                    if parent_value.get_bool(&field_name).is_err() {
                        error!("Invalid value provided for field: {}", field_name);
                        return Err(async_graphql::Error::from(format!(
                            "Invalid value provided for field: {}. Value is not of type `bool`.",
                            field_name
                        )));
                    }

                    let join_on_value = parent_value.get_bool(&field_name).unwrap();

                    query_document.insert(join_on, join_on_value);
                }
            }
            ScalarOptions::ObjectID => {
                if is_list {
                    // Check that all values in array are of type object id.
                    let valid_object_ids = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .all(|v| v.as_object_id().is_some());

                    if !valid_object_ids {
                        error!(
                            "Invalid value provided for field: {}. Value is not of type `object id`.",
                            field_name
                        );
                        return Err(async_graphql::Error::from(format!(
                            "Invalid value provided for field: {}. All values are not of type `object id`.",
                            field_name
                        )));
                    }

                    let join_on_value = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .map(|v| v.as_object_id().unwrap().clone())
                        .collect::<Vec<ObjectId>>();

                    query_document.insert(join_on, join_on_value);
                } else {
                    // Check that value is of type object id.
                    if parent_value.get_object_id(&field_name).is_err() {
                        error!("Invalid value provided for field: {}", field_name);
                        return Err(async_graphql::Error::from(format!(
                            "Invalid value provided for field: {}. Value is not of type `object id`.",
                            field_name
                        )));
                    }

                    let join_on_value = parent_value
                        .get_object_id(&field_name)
                        .unwrap()
                        .to_hex()
                        .to_string();

                    query_document.insert(join_on, join_on_value);
                }
            }
            _ => {
                error!("Unsupported scalar type: {:?}", scalar);
                return Err(async_graphql::Error::new(
                    "Failed to create internally joined query. Unsupported scalar type.",
                ));
            }
        }

        Ok(query_document.clone())
    }
}
