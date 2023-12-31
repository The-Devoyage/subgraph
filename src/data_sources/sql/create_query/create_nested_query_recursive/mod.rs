use bson::{doc, Bson};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum, entities::ServiceEntityConfig, SubGraphConfig,
    },
    data_sources::sql::{SqlDataSource, SqlValueEnum},
    graphql::schema::ResolverType,
};

use super::JoinClauses;

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
        pg_param_offset: Option<i32>,
        subgraph_config: &SubGraphConfig,
        join_clauses: Option<JoinClauses>,
        disable_eager_loading: bool,
    ) -> Result<(Option<String>, Vec<SqlValueEnum>, JoinClauses), async_graphql::Error> {
        debug!("Creating Recursive Nested Query From: {:?}", inputs);
        debug!("Initial Join Clauses: {:?}", join_clauses);
        let mut nested_query = String::new();
        let mut combined_where_values = vec![];
        let mut combined_join_clauses = join_clauses.unwrap_or(JoinClauses(Vec::new()));

        // Possibly need this for postgres.
        if is_first {
            nested_query.push_str(" (");
        } else {
            nested_query.push_str(" (");
        }

        let mut pg_param_offset = Some(pg_param_offset.unwrap_or(0));

        for (i, filter) in inputs.iter().enumerate() {
            //get the and and the or filters and handle recursively
            let and_filters = filter.as_document().unwrap().get("AND");
            let or_filters = filter.as_document().unwrap().get("OR");

            let mut initial_input = filter.clone().as_document().unwrap().clone();

            if initial_input.contains_key("AND") {
                initial_input.remove("AND");
            }
            if initial_input.contains_key("OR") {
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

            let (where_keys, where_values, _value_keys, _values, join_clauses) =
                SqlDataSource::get_key_data(
                    &query_input,
                    entity,
                    &ResolverType::FindOne,
                    &dialect,
                    &subgraph_config,
                    disable_eager_loading,
                )?;

            combined_join_clauses.0.extend(join_clauses.0);
            combined_where_values.extend(where_values.clone());

            let (parameterized_query, offset) = SqlDataSource::create_where_clause(
                &where_keys,
                dialect,
                pg_param_offset,
                &where_values,
            )?;

            pg_param_offset = Some(offset);

            nested_query.push_str(&parameterized_query);

            let is_first = i == 0;

            if and_filters.is_some() {
                let and_filters = and_filters.unwrap().as_array().unwrap();
                let has_more = if let Some(or_filters) = or_filters {
                    or_filters.as_array().unwrap().len() > 0
                } else {
                    false
                };
                let (and_query, and_where_values, and_join_clauses) =
                    SqlDataSource::create_nested_query_recursive(
                        is_first,
                        and_filters,
                        entity,
                        dialect,
                        FilterOperator::And,
                        has_more,
                        pg_param_offset,
                        subgraph_config,
                        Some(combined_join_clauses.clone()),
                        disable_eager_loading,
                    )?;

                combined_where_values.extend(and_where_values.clone());
                combined_join_clauses.0.extend(and_join_clauses.0);

                if let Some(and_query) = and_query {
                    nested_query.push_str(&and_query);
                };
            }

            if or_filters.is_some() {
                let or_filters = or_filters.unwrap().as_array().unwrap();
                let (or_query, or_where_values, or_join_clauses) =
                    SqlDataSource::create_nested_query_recursive(
                        is_first,
                        or_filters,
                        entity,
                        dialect,
                        FilterOperator::Or,
                        false,
                        pg_param_offset,
                        subgraph_config,
                        Some(combined_join_clauses.clone()),
                        disable_eager_loading,
                    )?;

                combined_where_values.extend(or_where_values.clone());
                combined_join_clauses.0.extend(or_join_clauses.0);

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

        let is_empty = nested_query.contains("()");
        let is_empty_and = nested_query.contains("AND") && nested_query.contains("()");
        if is_empty || nested_query.is_empty() || is_empty_and {
            return Ok((None, combined_where_values, combined_join_clauses));
        }

        // Filter duplicates from the join clauses.
        combined_join_clauses.0.sort();
        combined_join_clauses.0.dedup();

        debug!("Nested query: {}", nested_query);
        debug!("Combined Where Values: {:?}", combined_where_values);
        debug!("Combined Join Clauses: {:?}", combined_join_clauses);

        Ok((
            Some(nested_query),
            combined_where_values,
            combined_join_clauses,
        ))
    }
}
