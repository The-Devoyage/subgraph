use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum, entities::ServiceEntityConfig, SubGraphConfig,
    },
    data_sources::sql::{SqlDataSource, SqlValue},
    filter_operator::FilterOperator,
};
use bson::Document;
use log::debug;

impl SqlDataSource {
    pub fn create_update_many_query(
        entity: &ServiceEntityConfig,
        table_name: &str,
        value_keys: &Vec<String>,
        dialect: &DialectEnum,
        input: &Document,
        subgraph_config: &SubGraphConfig,
    ) -> Result<(String, Vec<SqlValue>, Vec<String>, String), async_graphql::Error> {
        debug!("Creating Update Many Query");

        let mut query = String::new();
        query.push_str("UPDATE ");
        query.push_str(table_name);

        let mut identifier_query = String::new();
        let primary_key_field = ServiceEntityConfig::get_primary_key_field(entity)?;
        identifier_query
            .push_str(format!("SELECT {} FROM {}", primary_key_field.name, table_name).as_str());

        let offset = Some(value_keys.len() as i32);
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
        identifier_query.push_str(" WHERE ");

        if let Some(nested_query) = nested_query {
            query.push_str(nested_query.as_str());
            identifier_query.push_str(nested_query.as_str());
        } else {
            return Err(async_graphql::Error::from("No filter provided"));
        }

        match dialect {
            DialectEnum::POSTGRES => {
                query.push_str(" RETURNING *");
            }
            _ => {}
        }

        if !query.ends_with(';') {
            query.push(';');
        }

        if !identifier_query.ends_with(';') {
            identifier_query.push(';');
        }

        debug!("Update Many Query: {}", query);
        Ok((
            query,
            combined_where_values,
            combined_where_keys,
            identifier_query,
        ))
    }
}
