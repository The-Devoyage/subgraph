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

        // Add the `AND` filter, which is an empty vector.
        query_input.insert::<_, Vec<Bson>>("AND", vec![]);

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
                    let join_on_value = parent_value
                        .get_array(&field_name)
                        .unwrap() // Safe to unwrap, already checked.
                        .iter()
                        .map(|v| {
                            ServiceResolver::get_string_value(v.as_document().unwrap(), field_name)
                                .unwrap()
                                .unwrap() // Assuming, safe to unwrap. Value from DB.
                        })
                        .collect::<Vec<String>>();
                    let join_query = doc! { join_on: join_on_value };
                    let bson = bson::to_bson(&join_query).unwrap();
                    query_input.get_array_mut("AND").unwrap().push(bson);
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
                if is_list {
                    let join_on_value = parent_value
                        .get_array(&field_name)
                        .unwrap() // Safe to unwrap, already checked.
                        .iter()
                        .map(|v| {
                            ServiceResolver::get_int_value(v.as_document().unwrap(), field_name)
                                .unwrap()
                                .unwrap() // Assuming, safe to unwrap. Value from DB.
                        })
                        .collect::<Vec<i64>>();
                    let join_query = doc! { join_on: join_on_value };
                    let bson = bson::to_bson(&join_query).unwrap();
                    query_input.get_array_mut("AND").unwrap().push(bson);
                } else {
                    let join_on_value = ServiceResolver::get_int_value(parent_value, field_name)?;
                    if join_on_value.is_some() {
                        let join_query = doc! { join_on: join_on_value };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input.get_array_mut("AND").unwrap().push(bson);
                    }
                }
            }
            ScalarOptions::Boolean => {
                if is_list {
                    let join_on_value = parent_value
                        .get_array(&field_name)
                        .unwrap() // Safe to unwrap, already checked.
                        .iter()
                        .map(|v| {
                            let bool_value = ServiceResolver::get_bool_value(
                                v.as_document().unwrap(),
                                field_name,
                            );
                            bool_value.unwrap().unwrap() // Assuming, safe to unwrap. Value from DB.
                        })
                        .collect::<Vec<bool>>();
                    let join_query = doc! { join_on: join_on_value };
                    let bson = bson::to_bson(&join_query).unwrap();
                    query_input.get_array_mut("AND").unwrap().push(bson);
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
                if is_list {
                    let join_on_value = parent_value
                        .get_array(&field_name)
                        .unwrap() // Safe to unwrap, already checked.
                        .iter()
                        .map(|v| {
                            ServiceResolver::get_object_id_value(
                                v.as_document().unwrap(),
                                field_name,
                            )
                            .unwrap()
                            .unwrap()
                        })
                        .collect::<Vec<ObjectId>>();
                    let join_query = doc! { join_on: join_on_value };
                    let bson = bson::to_bson(&join_query).unwrap();
                    query_input.get_array_mut("AND").unwrap().push(bson);
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
        let value = parent_value.get_object_id(field_name).ok();
        trace!("ObjectID value: {:?}", value);
        Ok(value)
    }
}
