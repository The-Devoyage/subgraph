use bson::Document;
use log::{debug, trace};

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum, entities::ServiceEntityConfig, SubGraphConfig,
    },
    data_sources::sql::{SqlDataSource, SqlValueEnum},
};

use super::{create_nested_query_recursive::FilterOperator, JoinClauses};

impl SqlDataSource {
    pub fn create_find_many_query(
        entity: &ServiceEntityConfig,
        table_name: &str,
        dialect: &DialectEnum,
        input: &Document,
        subgraph_config: &SubGraphConfig,
        join_clauses: Option<JoinClauses>,
        disable_eager_loading: bool,
    ) -> Result<(String, Vec<SqlValueEnum>, String), async_graphql::Error> {
        debug!("Creating Find Many Query");

        let mut query = String::new();
        let mut count_query = String::new();
        let entity_table_name = if let Some(entity_ds) = entity.data_source.clone() {
            if entity_ds.table.is_some() {
                entity_ds.table.unwrap()
            } else {
                entity.name.clone()
            }
        } else {
            entity.name.clone()
        };
        let select_statement = format!("SELECT {}.* FROM ", entity_table_name);
        query.push_str(&select_statement);
        query.push_str(table_name);

        let count_statement = format!("SELECT COUNT(*) as total_count FROM {}", table_name);
        count_query.push_str(&count_statement);

        let query_input = input.get("query").unwrap();
        let (nested_query, combined_where_values, combined_join_clauses) =
            SqlDataSource::create_nested_query_recursive(
                true,
                &vec![query_input.clone()],
                entity,
                dialect,
                FilterOperator::And,
                false,
                None,
                subgraph_config,
                join_clauses,
                disable_eager_loading,
            )?;

        for join_clause in combined_join_clauses.0 {
            trace!("Adding Join Clause: {}", join_clause);
            query.push_str(&join_clause);
            count_query.push_str(&join_clause);
        }

        query.push_str(" WHERE ");
        count_query.push_str(" WHERE ");

        if let Some(nested_query) = nested_query {
            query.push_str(&nested_query);
            count_query.push_str(&nested_query);
        } else {
            query.push_str("1=1");
            count_query.push_str("1=1");
        }

        let opts_input = input.get("opts");
        let mut per_page = 10;
        let mut page = 1;

        if let Some(opts_input) = opts_input {
            let opts = opts_input.as_document().unwrap();
            if let Some(per_page_input) = opts.get("per_page") {
                per_page = per_page_input.as_i32().unwrap();
            }
            if let Some(page_input) = opts.get("page") {
                page = page_input.as_i32().unwrap();
            }
        }

        let offset = (page - 1) * per_page;
        query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

        if !query.ends_with(';') {
            query.push(';');
        }

        if !count_query.ends_with(';') {
            count_query.push(';');
        }

        Ok((query, combined_where_values, count_query))
    }
}
