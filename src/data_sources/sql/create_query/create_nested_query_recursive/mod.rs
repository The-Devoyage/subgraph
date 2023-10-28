use bson::{doc, Bson};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{
    configuration::subgraph::{data_sources::sql::DialectEnum, entities::ServiceEntityConfig},
    data_sources::sql::{SqlDataSource, SqlValueEnum},
    graphql::schema::ResolverType,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FilterOperator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}

impl SqlDataSource {
    pub fn create_nested_query_recursive(
        is_first: bool,
        inputs: &Vec<Bson>,
        entity: &ServiceEntityConfig,
        dialect: &DialectEnum,
        filter_by_operator: FilterOperator,
        has_more: bool,
    ) -> Result<(Option<String>, Vec<SqlValueEnum>), async_graphql::Error> {
        debug!("Creating Recursive Nested Query From: {:?}", inputs);
        let mut nested_query = String::new();
        let mut combined_where_values = vec![];

        // Possibly need this for postgres.
        if is_first {
            nested_query.push_str(" (");
        } else {
            nested_query.push_str(" (");
        }

        for (i, filter) in inputs.iter().enumerate() {
            //get the and and the or filters and handle recursively
            let and_filters = filter.as_document().unwrap().get("AND");
            let or_filters = filter.as_document().unwrap().get("OR");

            let mut initial_input = filter.clone().as_document().unwrap().clone();

            if initial_input.contains_key("AND") {
                initial_input.remove("AND");
            }
            if initial_input.contains_key("OR") {
                debug!("Found OR key in initial input: {:?}", initial_input);
                initial_input.remove("OR");
            }

            // Only accept an initial_input or and_filters/or_filters.
            if (and_filters.is_some() || or_filters.is_some()) && !initial_input.is_empty() {
                return Err(async_graphql::Error::from(format!(
                    "Combining AND/OR filters with other filters is not supported. Found: {:?}",
                    filter
                )));
            }

            // Nest inside a "query" property for recursive calls.
            let query_input = doc! { "query": initial_input };

            let (where_keys, where_values, ..) =
                SqlDataSource::get_key_data(&query_input, entity, &ResolverType::FindOne)?;

            combined_where_values.extend(where_values.clone());

            let parameterized_query =
                SqlDataSource::create_where_clause(&where_keys, dialect, None, &where_values)?;

            nested_query.push_str(&parameterized_query);

            let is_first = i == 0;
            let is_last = i == inputs.len() - 1;
            let has_more = and_filters.is_some() || or_filters.is_some();
            debug!(
                "is_first: {}, is_last: {}, index: {}, input_length: {}",
                is_first,
                is_last,
                i,
                inputs.len()
            );

            if and_filters.is_some() {
                let (and_query, and_where_values) = SqlDataSource::create_nested_query_recursive(
                    is_first,
                    &and_filters.unwrap().as_array().unwrap(),
                    entity,
                    dialect,
                    FilterOperator::And,
                    or_filters.is_some(),
                )?;

                combined_where_values.extend(and_where_values.clone());

                if let Some(and_query) = and_query {
                    nested_query.push_str(&and_query);
                };
            }

            if or_filters.is_some() {
                let (or_query, or_where_values) = SqlDataSource::create_nested_query_recursive(
                    is_first,
                    &or_filters.unwrap().as_array().unwrap(),
                    entity,
                    dialect,
                    FilterOperator::Or,
                    false,
                )?;

                combined_where_values.extend(or_where_values.clone());

                if let Some(or_query) = or_query {
                    nested_query.push_str(&or_query);
                };
            }

            if i != inputs.len() - 1 {
                match filter_by_operator {
                    FilterOperator::And => nested_query.push_str(" AND "),
                    FilterOperator::Or => nested_query.push_str(" OR "),
                }
            }
        }

        nested_query.push_str(")");

        if has_more {
            nested_query.push_str(" AND ");
        }

        debug!("Nested query: {}", nested_query);

        if nested_query == " ()" || nested_query.is_empty() || nested_query == " And ()" {
            return Ok((None, combined_where_values));
        }

        Ok((Some(nested_query), combined_where_values))
    }
}
