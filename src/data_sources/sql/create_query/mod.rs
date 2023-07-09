use bson::Document;
use log::debug;

use crate::{
    configuration::subgraph::{data_sources::sql::DialectEnum, entities::ServiceEntityConfig},
    graphql::schema::ResolverType,
};

use super::{SqlDataSource, SqlQuery};

pub mod create_create_one_query;
pub mod create_find_many_query;
pub mod create_find_one_query;
pub mod create_update_many_query;
pub mod create_update_one_query;
pub mod create_update_return_key_data;
pub mod create_where_clause;
pub mod get_key_data;
pub mod get_placeholder;

impl SqlDataSource {
    pub fn create_query(
        input: Document,
        resolver_type: ResolverType,
        table_name: &str,
        dialect: DialectEnum,
        entity: &ServiceEntityConfig,
    ) -> SqlQuery {
        debug!("Creating SQL Query");

        let (where_keys, where_values, value_keys, values) =
            SqlDataSource::get_key_data(&input, entity, &resolver_type);

        let query = match resolver_type {
            ResolverType::FindOne => SqlDataSource::create_find_one_query(
                table_name,
                &where_keys,
                &dialect,
                &where_values,
            ),
            ResolverType::FindMany => SqlDataSource::create_find_many_query(
                table_name,
                &where_keys,
                &dialect,
                &where_values,
            ),
            ResolverType::CreateOne => {
                SqlDataSource::create_create_one_query(table_name, &value_keys, &dialect)
            }
            ResolverType::UpdateOne => SqlDataSource::create_update_one_query(
                table_name,
                &value_keys,
                &dialect,
                &where_keys,
                &where_values,
            ),
            ResolverType::UpdateMany => SqlDataSource::create_update_many_query(
                table_name,
                &value_keys,
                &dialect,
                &where_keys,
                &where_values,
            ),
            _ => panic!("Invalid resolver type"),
        };

        let sql_query = SqlQuery {
            query,
            where_keys,
            where_values,
            value_keys,
            values,
            table: table_name.to_string(),
        };

        debug!("Query: {:?}", sql_query);

        sql_query
    }
}
