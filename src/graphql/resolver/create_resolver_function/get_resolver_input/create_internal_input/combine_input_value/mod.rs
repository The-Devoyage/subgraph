use std::str::FromStr;

use crate::{
    filter_operator::FilterOperator, graphql::resolver::ServiceResolver,
    scalar_option::ScalarOption, utils::document::get_from_document::DocumentValue,
};
use bson::{doc, oid::ObjectId, Bson, Document};
use log::{debug, trace};

impl ServiceResolver {
    /// Extracts the primitive value from the input query and combines it with the parent value.
    pub fn combine_input_value(
        parent_value: &Document,       // The parent value, with data from the ds.
        query_document: &mut Document, // The user provided input
        field_name: &str,
        scalar: &ScalarOption,
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
        query_input.insert::<_, Vec<Bson>>(FilterOperator::And.as_str(), vec![]);
        query_input.insert::<_, Vec<Bson>>(FilterOperator::Or.as_str(), vec![]);

        // Add the original parent value to the `AND` filter.
        if !query_document.is_empty() {
            let bson = bson::to_bson(&query_document).unwrap();
            query_input
                .get_array_mut(FilterOperator::And.as_str())
                .unwrap()
                .push(bson);
        }

        // Determine if the value provided is an array/vec or not.
        let is_list = match parent_value.get_array(&field_name) {
            Ok(_) => true,
            Err(_) => false,
        };

        let join_on_value = scalar
            .get_from_document(parent_value, field_name, is_list)
            .ok();

        // If the scalar is ObjectID and the join_on_value is none,
        // it might need to be deserialized from a string as sql does not
        // support ObjectID.
        let join_on_value = match scalar {
            ScalarOption::ObjectID => {
                if join_on_value.is_none() {
                    let value = parent_value.get_str(field_name).unwrap();
                    let value = ObjectId::from_str(value).unwrap();
                    Some(DocumentValue::ObjectID(value))
                } else {
                    join_on_value
                }
            }
            _ => join_on_value,
        };

        if join_on_value.is_some() {
            match join_on_value.unwrap() {
                DocumentValue::String(v) => {
                    let join_query = doc! { join_on: Some(v) };
                    let bson = bson::to_bson(&join_query).unwrap();
                    query_input
                        .get_array_mut(FilterOperator::And.as_str())
                        .unwrap()
                        .push(bson);
                }
                DocumentValue::UUID(v) => {
                    // Needs to be converted to String for input type.
                    let join_query = doc! { join_on: Some(v.to_string()) };
                    let bson = bson::to_bson(&join_query).unwrap();
                    query_input
                        .get_array_mut(FilterOperator::And.as_str())
                        .unwrap()
                        .push(bson);
                }
                DocumentValue::DateTime(v) => {
                    // Needs to be converted to String for input type.
                    let join_query = doc! { join_on: Some(v.to_string()) };
                    let bson = bson::to_bson(&join_query).unwrap();
                    query_input
                        .get_array_mut(FilterOperator::And.as_str())
                        .unwrap()
                        .push(bson);
                }
                DocumentValue::StringArray(v) => {
                    for value in v {
                        let join_query = doc! { join_on: Some(value) };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input
                            .get_array_mut(FilterOperator::Or.as_str())
                            .unwrap()
                            .push(bson);
                    }
                }
                DocumentValue::UUIDArray(v) => {
                    for value in v {
                        // Needs to be converted to String for input type.
                        let join_query = doc! { join_on: Some(value.to_string()) };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input
                            .get_array_mut(FilterOperator::Or.as_str())
                            .unwrap()
                            .push(bson);
                    }
                }
                DocumentValue::DateTimeArray(v) => {
                    for value in v {
                        // Needs to be converted to String for input type.
                        let join_query = doc! { join_on: Some(value.to_string()) };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input
                            .get_array_mut(FilterOperator::Or.as_str())
                            .unwrap()
                            .push(bson);
                    }
                }
                DocumentValue::Int(v) => {
                    // Needs to be converted to i64 for input type.
                    let join_query = doc! { join_on: Some(v as i64) };
                    let bson = bson::to_bson(&join_query).unwrap();
                    query_input
                        .get_array_mut(FilterOperator::And.as_str())
                        .unwrap()
                        .push(bson);
                }
                DocumentValue::IntArray(v) => {
                    for value in v {
                        // Needs to be converted to i64 for input type.
                        let join_query = doc! { join_on: Some(value as i64) };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input
                            .get_array_mut(FilterOperator::Or.as_str())
                            .unwrap()
                            .push(bson);
                    }
                }
                DocumentValue::ObjectID(v) => {
                    let join_query = doc! { join_on: Some(v) };
                    let bson = bson::to_bson(&join_query).unwrap();
                    query_input
                        .get_array_mut(FilterOperator::And.as_str())
                        .unwrap()
                        .push(bson);
                }
                DocumentValue::ObjectIDArray(v) => {
                    for value in v {
                        let join_query = doc! { join_on: Some(value) };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input
                            .get_array_mut(FilterOperator::Or.as_str())
                            .unwrap()
                            .push(bson);
                    }
                }
                DocumentValue::Boolean(v) => {
                    let join_query = doc! { join_on: Some(v) };
                    let bson = bson::to_bson(&join_query).unwrap();
                    query_input
                        .get_array_mut(FilterOperator::And.as_str())
                        .unwrap()
                        .push(bson);
                }
                DocumentValue::BooleanArray(v) => {
                    for value in v {
                        let join_query = doc! { join_on: Some(value) };
                        let bson = bson::to_bson(&join_query).unwrap();
                        query_input
                            .get_array_mut(FilterOperator::Or.as_str())
                            .unwrap()
                            .push(bson);
                    }
                }
                _ => {}
            }
        }

        // If there are no values to join on, return empty document.
        if query_input
            .get_array(FilterOperator::And.as_str())
            .unwrap()
            .is_empty()
        {
            query_input.remove(FilterOperator::And.as_str());
        }
        if query_input
            .get_array(FilterOperator::Or.as_str())
            .unwrap()
            .is_empty()
        {
            query_input.remove(FilterOperator::Or.as_str());
        }

        trace!("Joined Query: {:?}", query_input);
        Ok(query_input.clone())
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
}
