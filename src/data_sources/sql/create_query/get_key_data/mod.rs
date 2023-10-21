use bson::Document;
use log::debug;

use crate::{
    configuration::subgraph::entities::ServiceEntityConfig,
    data_sources::sql::{SqlDataSource, SqlValueEnum},
    graphql::schema::ResolverType,
};

mod parse_query_input;
mod parse_values_input;

impl SqlDataSource {
    /// Creates vectors of keys and values. They persist order.
    /// Keys and values are assocciated with where clause and value clause.
    pub fn get_key_data(
        input_object: &Document,
        entity: &ServiceEntityConfig,
        resolver_type: &ResolverType,
    ) -> Result<
        (
            Vec<String>,
            Vec<SqlValueEnum>,
            Vec<String>,
            Vec<SqlValueEnum>,
        ),
        async_graphql::Error,
    > {
        debug!("Getting Key Data From Input: {:?}", input_object);
        let (mut where_keys, mut where_values, mut value_keys, mut values) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());

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
                )?;
            } else if key == "query" {
                (where_keys, where_values) =
                    SqlDataSource::parse_query_input(value, where_keys, where_values)?;
            }
        }

        debug!("Where Keys: {:?}", where_keys);
        debug!("Where Values: {:?}", where_values);
        debug!("Value Keys: {:?}", value_keys);
        debug!("Values: {:?}", values);

        Ok((where_keys, where_values, value_keys, values))
    }
}
