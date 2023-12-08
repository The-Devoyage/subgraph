use bson::Document;
use log::error;

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum, entities::ServiceEntityConfig, SubGraphConfig,
    },
    data_sources::sql::{SqlDataSource, SqlValueEnum},
};

use super::{create_nested_query_recursive::FilterOperator, JoinClauses};

impl SqlDataSource {
    pub fn create_find_one_query(
        entity: &ServiceEntityConfig,
        table_name: &str,
        dialect: &DialectEnum,
        input: &Document,
        subgraph_config: &SubGraphConfig,
        join_clauses: Option<JoinClauses>,
    ) -> Result<(String, Vec<SqlValueEnum>), async_graphql::Error> {
        let mut query = String::new();
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

        let query_input = match input.get("query") {
            Some(query_input) => query_input,
            None => {
                error!("Invalid Query Object: {:?}", input);
                return Err(async_graphql::Error::new("Invalid Query Object"));
            }
        };

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
            )?;

        for join_clause in combined_join_clauses.0 {
            query.push_str(&join_clause);
        }

        query.push_str(" WHERE ");

        if let Some(nested_query) = nested_query {
            query.push_str(&nested_query);
        } else {
            query.push_str("1 = 1");
        }

        if !query.ends_with(';') {
            query.push(';');
        }

        Ok((query, combined_where_values))
    }
}
