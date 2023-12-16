use bson::Document;
use log::debug;

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum, entities::ServiceEntityConfig, SubGraphConfig,
    },
    data_sources::sql::{SqlDataSource, SqlValueEnum},
};

use super::create_nested_query_recursive::FilterOperator;

impl SqlDataSource {
    pub fn create_update_one_query(
        entity: &ServiceEntityConfig,
        table_name: &str,
        value_keys: &Vec<String>,
        dialect: &DialectEnum,
        input: &Document,
        subgraph_config: &SubGraphConfig,
    ) -> Result<(String, Vec<SqlValueEnum>), async_graphql::Error> {
        debug!("Creating Update One Query");

        let mut query = String::new();
        query.push_str("UPDATE ");
        query.push_str(table_name);
        query.push_str(" SET ");

        for i in 0..value_keys.len() {
            query.push_str(&value_keys[i]);
            query.push_str(" = ");
            query.push_str(SqlDataSource::get_placeholder(dialect, Some(i as i32)).as_str());
            if i != value_keys.len() - 1 {
                query.push_str(", ");
            }
        }

        // Offset used for postgres WHERE key placeholders, $1, $2
        let offset = Some(value_keys.len() as i32);

        query.push_str(" WHERE ");

        let query_input = input.get("query").unwrap();
        let (nested_query, combined_where_values, _combined_join_values) =
            SqlDataSource::create_nested_query_recursive(
                true,
                &vec![query_input.clone()],
                entity,
                dialect,
                FilterOperator::And,
                false,
                offset,
                subgraph_config,
                None,
                false,
            )?;

        if let Some(nested_query) = nested_query {
            query.push_str(nested_query.as_str());
        } else {
            return Err(async_graphql::Error::from("No filter provided"));
        }

        query.push_str(" LIMIT 1");

        if !query.ends_with(';') {
            query.push(';');
        }

        Ok((query, combined_where_values))
    }
}
