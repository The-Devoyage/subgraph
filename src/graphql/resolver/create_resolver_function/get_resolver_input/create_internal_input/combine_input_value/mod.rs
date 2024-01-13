use std::str::FromStr;

use crate::{configuration::subgraph::entities::ScalarOptions, graphql::resolver::ServiceResolver};
use bson::{doc, oid::ObjectId, Bson, Document};
use log::{debug, error, trace};

impl ServiceResolver {
    /// Extracts the primitive value from the input query and combines it with the parent value.
    pub fn combine_input_value(
        parent_value: &Document,       // The parent value, with data from the ds.
        query_document: &mut Document, // The user provided input
        field_name: &str,
        scalar: &ScalarOptions,
        join_on: &str,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Combining Primitive Value With Input");
        trace!("Parent Value: {:?}", parent_value);
        trace!(
            "Join On: {} as {:?} with field name: {}",
            join_on,
            scalar,
            field_name
        );

        // Declare the new query input.
        let mut query_input = Document::new();

        // Add the `AND` and `OR` filter, which is an empty vector.
        query_input.insert::<_, Vec<Bson>>("AND", vec![]);
        query_input.insert::<_, Vec<Bson>>("OR", vec![]);

        // Add the original parent value to the `AND` filter.
        if !query_document.is_empty() {
            let bson = bson::to_bson(&query_document).unwrap();
            query_input.get_array_mut("AND").unwrap().push(bson);
        }

        // Determine if the value provided is an array/vec or not.
        let is_list = match parent_value.get_array(&field_name) {
            Ok(_) => true,
            Err(_) => false,
        };

        // Replace the key of the input with the correct key to join on.
        // Map the value to the correct type based on the scalar.
        match scalar {
            ScalarOptions::String | ScalarOptions::UUID | ScalarOptions::DateTime => {
                if is_list {
                    trace!("Combining String Value With Input - Is List");
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
                    for value in join_on_value {
                        let join_query = doc! { join_on: value };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input.get_array_mut("OR").unwrap().push(bson);
                    }
                } else {
                    let join_on_value =
                        ServiceResolver::get_string_value(parent_value, field_name)?;
                    if join_on_value.is_some() {
                        let join_query = doc! { join_on: join_on_value };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input.get_array_mut("AND").unwrap().push(bson);
                    }
                }
            }
            ScalarOptions::Int => {
                trace!("Combining Int Value With Input");
                if is_list {
                    trace!("Combining Int Value With Input - Is List");
                    // Check that all values in array are of type int.
                    let valid_ints = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .all(|v| v.as_i32().is_some());

                    if !valid_ints {
                        error!("Invalid value provided for field: {}. All values are not of type `int`.", field_name);
                        return Err(async_graphql::Error::from(format!(
                            "Invalid value provided for field: {}. All values are not of type `int`.",
                            field_name
                        )));
                    }

                    let join_on_value = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .map(|v| v.as_i32().unwrap() as i64)
                        .collect::<Vec<i64>>();

                    for value in join_on_value {
                        let join_query = doc! { join_on: value };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input.get_array_mut("OR").unwrap().push(bson);
                    }
                } else {
                    trace!("Combining Int Value With Input - Is Not List");
                    let join_on_value = ServiceResolver::get_int_value(parent_value, field_name)?;
                    if join_on_value.is_some() {
                        let join_query = doc! { join_on: join_on_value };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input.get_array_mut("AND").unwrap().push(bson);
                        trace!("Join Query: {:?}", query_input);
                    }
                }
            }
            ScalarOptions::Boolean => {
                trace!("Combining Boolean Value With Input");
                if is_list {
                    trace!("Combining Boolean Value With Input - Is List");
                    // Check that all values in array are of type bool.
                    let valid_bools = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .all(|v| v.as_bool().is_some());

                    if !valid_bools {
                        error!("Invalid value provided for field: {}. All values are not of type `bool`.", field_name);
                        return Err(async_graphql::Error::from(format!(
                            "Invalid value provided for field: {}. All values are not of type `bool`.",
                            field_name
                        )));
                    }

                    let join_on_value = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .map(|v| if v.as_bool().unwrap() { true } else { false })
                        .collect::<Vec<bool>>();

                    for value in join_on_value {
                        let join_query = doc! { join_on: value };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input.get_array_mut("OR").unwrap().push(bson);
                    }
                } else {
                    let join_on_value = ServiceResolver::get_bool_value(parent_value, field_name)?;
                    if join_on_value.is_some() {
                        let join_query = doc! { join_on: join_on_value };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input.get_array_mut("AND").unwrap().push(bson);
                    }
                }
            }
            ScalarOptions::ObjectID => {
                trace!("Combining ObjectID Value With Input");
                if is_list {
                    debug!("Combining ObjectID Value With Input - Is List");

                    // Check that all values in array are of type ObjectID.
                    let valid_object_ids = parent_value
                        .get_array(&field_name)
                        .unwrap()
                        .iter()
                        .all(|v| v.as_object_id().is_some());

                    if !valid_object_ids {
                        error!("Invalid value provided for field: {}. All values are not of type `ObjectID`.", field_name);
                        return Err(async_graphql::Error::from(format!(
                            "Invalid value provided for field: {}. All values are not of type `ObjectID`.",
                            field_name
                        )));
                    }

                    let join_on_value = parent_value
                        .get_array(&field_name)
                        .unwrap() // Safe to unwrap, already checked.
                        .iter()
                        .map(|v| {
                            v.as_object_id()
                                .unwrap() // Safe to unwrap, already checked.
                                .clone()
                        })
                        .collect::<Vec<ObjectId>>();

                    for value in join_on_value {
                        let join_query = doc! { join_on: value };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input.get_array_mut("OR").unwrap().push(bson);
                    }
                } else {
                    let join_on_value =
                        ServiceResolver::get_object_id_value(parent_value, field_name)?;
                    if join_on_value.is_some() {
                        let join_query = doc! { join_on: join_on_value };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input.get_array_mut("AND").unwrap().push(bson);
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

        // If there are no values to join on, return empty document.
        if query_input.get_array("AND").unwrap().is_empty() {
            query_input.remove("AND");
        }
        if query_input.get_array("OR").unwrap().is_empty() {
            query_input.remove("OR");
        }

        trace!("Joined Query: {:?}", query_input);
        Ok(query_input.clone())
    }

    pub fn get_string_value(
        parent_value: &Document,
        field_name: &str,
    ) -> Result<Option<String>, async_graphql::Error> {
        debug!("Getting string value for field: {}", field_name);
        let value = match parent_value.get_str(field_name) {
            Ok(value) => Some(value.to_string()),
            Err(_) => None,
        };
        trace!("String value: {:?}", value);
        Ok(value)
    }

    pub fn get_int_value(
        parent_value: &Document,
        field_name: &str,
    ) -> Result<Option<i64>, async_graphql::Error> {
        debug!("Getting int value for field: {}", field_name);
        let value = parent_value.get_i64(field_name).ok();
        // If not an i64, try getting it as an i32.
        let value = match value {
            Some(value) => Some(value),
            None => match parent_value.get_i32(field_name) {
                Ok(value) => Some(value as i64),
                Err(_) => None,
            },
        };
        trace!("Int value: {:?}", value);
        Ok(value)
    }

    pub fn get_bool_value(
        parent_value: &Document,
        field_name: &str,
    ) -> Result<Option<bool>, async_graphql::Error> {
        debug!("Getting bool value for field: {}", field_name);
        let value = parent_value.get_bool(field_name).ok();
        trace!("Bool value: {:?}", value);
        Ok(value)
    }

    pub fn get_object_id_value(
        parent_value: &Document,
        field_name: &str,
    ) -> Result<Option<ObjectId>, async_graphql::Error> {
        debug!("Getting ObjectID value for field: {}", field_name);
        let mut value = parent_value.get_object_id(field_name).ok();
        if value.is_none() {
            // If coming from a SQL dialect, the ObjectID may be stored as a string.
            let string_obj_id = parent_value.get_str(field_name).ok();
            if string_obj_id.is_some() {
                value = ObjectId::from_str(string_obj_id.unwrap()).ok();
            }
        }
        trace!("ObjectID value: {:?}", value);
        Ok(value)
    }
}
