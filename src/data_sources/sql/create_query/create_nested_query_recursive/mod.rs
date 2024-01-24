use bson::{doc, Bson};
use log::{debug, trace};

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum, entities::ServiceEntityConfig, SubGraphConfig,
    },
    data_sources::sql::{SqlDataSource, SqlValue},
    filter_operator::FilterOperator,
    resolver_type::ResolverType,
};

use super::JoinClauses;

impl SqlDataSource {
    pub fn create_nested_query_recursive(
        inputs: &Vec<Bson>,
        entity: &ServiceEntityConfig,
        dialect: &DialectEnum,
        filter_by_operator: FilterOperator,
        has_more: bool,
        pg_param_offset: Option<i32>,
        subgraph_config: &SubGraphConfig,
        join_clauses: Option<JoinClauses>,
        disable_eager_loading: bool,
    ) -> Result<
        (
            Option<String>,
            Vec<SqlValue>,
            JoinClauses,
            Vec<String>,
            Option<i32>,
        ),
        async_graphql::Error,
    > {
        debug!("Creating Recursive Nested Query");
        trace!("Initial Inputs: {:?}", inputs);
        trace!("Initial Join Clauses: {:?}", join_clauses);
        trace!("Pg Param Offset: {:?}", pg_param_offset);
        let mut nested_query = String::new();
        let mut combined_where_values = vec![];
        let mut combined_where_keys = vec![];
        let mut combined_join_clauses = join_clauses.unwrap_or(JoinClauses(Vec::new()));

        nested_query.push_str(" (");

        let mut pg_param_offset = Some(pg_param_offset.unwrap_or(0));
        trace!("Pg Param Offset Init: {:?}", pg_param_offset);

        for (i, filter) in inputs.iter().enumerate() {
            //get the filters to handle recursively
            let mut recursive_filters = vec![];
            for filter_operator in FilterOperator::list() {
                let filter = filter.as_document().unwrap().get(filter_operator.as_str());
                recursive_filters.push((filter_operator, filter));
            }

            let mut initial_input = filter.clone().as_document().unwrap().clone();
            let mut is_nested = false;

            // Remove the and/or/like filters from the initial_input.
            // These are handled recursively.
            for filter_operator in FilterOperator::list() {
                if initial_input.contains_key(filter_operator.as_str()) {
                    initial_input.remove(filter_operator.as_str());
                    is_nested = true;
                }
            }

            // Nest inside a "query" property for recursive calls.
            let query_input = doc! { "query": initial_input };

            // Handle the initial filter.
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
            combined_where_keys.extend(where_keys.clone());

            let (parameterized_query, offset) = SqlDataSource::create_where_clause(
                &where_keys,
                dialect,
                pg_param_offset,
                &where_values,
                filter_by_operator.clone(),
            )?;

            pg_param_offset = Some(offset);

            nested_query.push_str(&parameterized_query);

            if is_nested && i == 0 && !has_more && nested_query != " (" {
                nested_query.push_str(" AND ");
            }

            for (i, recursive_filter) in recursive_filters.iter().enumerate() {
                if recursive_filter.1.is_none() {
                    continue;
                }

                let filters = match recursive_filter.0 {
                    FilterOperator::And | FilterOperator::Or => recursive_filter
                        .1
                        .clone()
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .clone(),
                    _ => {
                        let doc = recursive_filter.1.clone().unwrap().as_document().unwrap();
                        let mut array = vec![];
                        array.push(Bson::Document(doc.clone()));
                        array
                    }
                };
                let is_last = i == recursive_filters.len() - 1;
                let has_more = if !is_last && recursive_filters[i + 1].1.is_some() {
                    true
                } else {
                    false
                };
                let (
                    recursive_query,
                    recursive_where_values,
                    recursive_join_clauses,
                    recursive_where_keys,
                    offset,
                ) = SqlDataSource::create_nested_query_recursive(
                    &filters,
                    entity,
                    dialect,
                    recursive_filter.0.clone(),
                    has_more,
                    pg_param_offset,
                    subgraph_config,
                    Some(combined_join_clauses.clone()),
                    disable_eager_loading,
                )?;

                combined_where_values.extend(recursive_where_values);
                combined_where_keys.extend(recursive_where_keys);
                combined_join_clauses.0.extend(recursive_join_clauses.0);
                if offset.is_some() {
                    pg_param_offset = offset;
                }

                if recursive_query.is_some() {
                    nested_query.push_str(&recursive_query.unwrap());
                }
            }

            if i != inputs.len() - 1 {
                match filter_by_operator {
                    FilterOperator::And => nested_query.push_str(" AND "),
                    FilterOperator::Or => nested_query.push_str(" OR "),
                    _ => (),
                }
            }
        }

        nested_query.push_str(")");

        if has_more {
            nested_query.push_str(" AND ");
        }

        let is_empty = nested_query.contains("()");
        let is_empty_and =
            nested_query.contains(FilterOperator::And.as_str()) && nested_query.contains("()");
        if is_empty || nested_query.is_empty() || is_empty_and {
            return Ok((
                None,
                combined_where_values,
                combined_join_clauses,
                combined_where_keys,
                pg_param_offset,
            ));
        }

        // Filter duplicates from the join clauses.
        combined_join_clauses.0.sort();
        combined_join_clauses.0.dedup();

        trace!("Nested query: {}", nested_query);
        trace!("Combined Where Values: {:?}", combined_where_values);
        trace!("Combined Join Clauses: {:?}", combined_join_clauses);
        trace!("Combined Where Keys: {:?}", combined_where_keys);

        Ok((
            Some(nested_query),
            combined_where_values,
            combined_join_clauses,
            combined_where_keys,
            pg_param_offset,
        ))
    }
}
