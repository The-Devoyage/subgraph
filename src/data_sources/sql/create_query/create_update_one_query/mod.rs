use bson::Document;
use log::debug;

use crate::{
    configuration::subgraph::{data_sources::sql::DialectEnum, entities::ServiceEntityConfig},
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

        query.push_str(" WHERE ");

        let (nested_query, combined_where_values) = SqlDataSource::create_nested_query_recursive(
            true,
            &vec![input.clone().into()],
            entity,
            dialect,
            FilterOperator::And,
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
