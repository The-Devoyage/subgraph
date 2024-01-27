use bson::Document;
use log::debug;

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum, entities::ServiceEntityConfig, SubGraphConfig,
    },
    data_sources::sql::SqlDataSource,
    filter_operator::FilterOperator,
    sql_value::SqlValue,
};

impl SqlDataSource {
    pub fn create_update_one_query(
        entity: &ServiceEntityConfig,
        table_name: &str,
        value_keys: &Vec<String>,
        dialect: &DialectEnum,
        input: &Document,
        subgraph_config: &SubGraphConfig,
    ) -> Result<(String, Vec<SqlValue>, Vec<String>, String), async_graphql::Error> {
        debug!("Creating Update One Query");

        let mut query = String::new();
        query.push_str("UPDATE ");
        query.push_str(table_name);
        query.push_str(" SET ");

        let mut identifier_query = String::new();
        let primary_key_field = ServiceEntityConfig::get_primary_key_field(entity)?;
        identifier_query
            .push_str(format!("SELECT {} FROM {}", primary_key_field.name, table_name).as_str());

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
        identifier_query.push_str(" WHERE ");

        let query_input = input.get("query").unwrap();
        let (
            nested_query,
            combined_where_values,
            _combined_join_values,
            combined_where_keys,
            _offset,
        ) = SqlDataSource::create_nested_query_recursive(
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
            identifier_query.push_str(nested_query.as_str());
        } else {
            return Err(async_graphql::Error::from("No filter provided"));
        }

        query.push_str(" LIMIT 1");

        if !query.ends_with(';') {
            query.push(';');
        }

        if !identifier_query.ends_with(';') {
            identifier_query.push(';');
        }

        Ok((
            query,
            combined_where_values,
            combined_where_keys,
            identifier_query,
        ))
    }
}
