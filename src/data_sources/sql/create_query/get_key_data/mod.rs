use bson::Document;
use log::{debug, trace};

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum, entities::ServiceEntityConfig, SubGraphConfig,
    },
    data_sources::sql::{SqlDataSource, SqlValueEnum},
    resolver_type::ResolverType,
};

use super::JoinClauses;

mod parse_query_input;
mod parse_values_input;

impl SqlDataSource {
    /// Creates vectors of keys and values parsed from the user provided input.
    /// They persist order.
    /// Keys and values are assocciated with where clause and value clause.
    pub fn get_key_data(
        input_object: &Document,
        entity: &ServiceEntityConfig,
        resolver_type: &ResolverType,
        dialect: &DialectEnum,
        subgraph_config: &SubGraphConfig,
        disable_eager_loading: bool,
    ) -> Result<
        (
            Vec<String>,
            Vec<SqlValueEnum>,
            Vec<String>,
            Vec<SqlValueEnum>,
            JoinClauses,
        ),
        async_graphql::Error,
    > {
        debug!("Getting Key Data From Input: {:?}", input_object);
        let (mut where_keys, mut where_values, mut value_keys, mut values, mut join_clauses) = (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            JoinClauses(Vec::new()),
        );

        for (key, value) in input_object.iter() {
            if key == "values" {
                (where_keys, where_values, value_keys, values) = SqlDataSource::parse_values_input(
                    value,
                    where_keys,
                    where_values,
                    value_keys,
                    values,
                    entity,
                    resolver_type,
                    dialect,
                )?;
            } else if key == "query" {
                (where_keys, where_values, join_clauses) = SqlDataSource::parse_query_input(
                    value,
                    where_keys,
                    where_values,
                    dialect,
                    entity,
                    subgraph_config,
                    disable_eager_loading,
                )?;
            }
        }

        trace!("Completed Parsing Input Key Data");
        trace!("Where Keys: {:?}", where_keys);
        trace!("Where Values: {:?}", where_values);
        trace!("Value Keys: {:?}", value_keys);
        trace!("Values: {:?}", values);
        trace!("Join Clauses: {:?}", join_clauses);

        Ok((where_keys, where_values, value_keys, values, join_clauses))
    }
}
