use bson::Document;
use log::error;

use crate::{
    configuration::subgraph::{data_sources::sql::DialectEnum, entities::ServiceEntityConfig},
    data_sources::sql::{SqlDataSource, SqlValueEnum},
};

use super::create_nested_query_recursive::FilterOperator;

impl SqlDataSource {
    pub fn create_find_one_query(
        entity: &ServiceEntityConfig,
        table_name: &str,
        dialect: &DialectEnum,
        input: &Document,
    ) -> Result<(String, Vec<SqlValueEnum>), async_graphql::Error> {
        let mut query = String::new();
        query.push_str("SELECT * FROM ");
        query.push_str(table_name);
        query.push_str(" WHERE ");

        let query_input = match input.get("query") {
            Some(query_input) => query_input,
            None => {
                error!("Invalid Query Object: {:?}", input);
                return Err(async_graphql::Error::new("Invalid Query Object"));
            }
        };

        let (nested_query, combined_where_values) = SqlDataSource::create_nested_query_recursive(
            true,
            &vec![query_input.clone()],
            entity,
            dialect,
            FilterOperator::And,
            false,
            None,
        )?;

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
