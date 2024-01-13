use bson::Document;
use log::debug;

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum, entities::ServiceEntityConfig, SubGraphConfig,
    },
    graphql::schema::ResolverType,
};

use super::{SqlDataSource, SqlQuery};

pub mod create_create_one_query;
pub mod create_find_many_query;
pub mod create_find_one_query;
pub mod create_nested_query_recursive;
pub mod create_update_many_query;
pub mod create_update_one_query;
pub mod create_update_return_key_data;
pub mod create_where_clause;
pub mod get_key_data;
pub mod get_placeholder;

#[derive(Debug, Clone)]
pub struct JoinClauses(pub Vec<String>);

impl SqlDataSource {
    pub fn create_query(
        input: Document,
        resolver_type: ResolverType,
        table_name: &str,
        dialect: DialectEnum,
        entity: &ServiceEntityConfig,
        subgraph_config: &SubGraphConfig,
    ) -> Result<SqlQuery, async_graphql::Error> {
        debug!("Creating SQL Query");

        let (mut where_keys, mut where_values, value_keys, values, join_clauses) =
            SqlDataSource::get_key_data(
                &input,
                entity,
                &resolver_type,
                &dialect,
                &subgraph_config,
                false,
            )?;
        let mut count_query = None;
        let mut identifier_query = None;

        // Generate the query string and get the where values.
        let query = match resolver_type {
            ResolverType::FindOne => {
                let (query_string, combined_where_values, combined_where_keys) =
                    SqlDataSource::create_find_one_query(
                        &entity,
                        table_name,
                        &dialect,
                        &input,
                        subgraph_config,
                        Some(join_clauses),
                    )?;
                where_values = combined_where_values;
                where_keys = combined_where_keys;
                query_string
            }
            ResolverType::FindMany => {
                let (query_string, combined_where_values, count_q, combined_where_keys) =
                    SqlDataSource::create_find_many_query(
                        &entity,
                        table_name,
                        &dialect,
                        &input,
                        subgraph_config,
                        Some(join_clauses),
                        false,
                    )?;
                where_values = combined_where_values;
                where_keys = combined_where_keys;
                count_query = Some(count_q);
                query_string
            }
            ResolverType::CreateOne => {
                SqlDataSource::create_create_one_query(table_name, &value_keys, &dialect)?
            }
            ResolverType::UpdateOne => {
                let (query_string, combined_where_value, combined_where_keys) =
                    SqlDataSource::create_update_one_query(
                        &entity,
                        table_name,
                        &value_keys,
                        &dialect,
                        &input,
                        subgraph_config,
                    )?;
                where_values = combined_where_value;
                where_keys = combined_where_keys;
                query_string
            }
            ResolverType::UpdateMany => {
                let (query_string, combined_where_value, combined_where_keys, identifier_q) =
                    SqlDataSource::create_update_many_query(
                        &entity,
                        table_name,
                        &value_keys,
                        &dialect,
                        &input,
                        subgraph_config,
                    )?;
                where_values = combined_where_value;
                where_keys = combined_where_keys;
                identifier_query = Some(identifier_q);
                query_string
            }
            _ => panic!("Invalid resolver type"),
        };

        let sql_query = SqlQuery {
            query,
            count_query,
            identifier_query,
            where_keys,
            where_values,
            value_keys,
            values,
            table: table_name.to_string(),
        };

        debug!("Query: {:?}", sql_query);

        Ok(sql_query)
    }
}
