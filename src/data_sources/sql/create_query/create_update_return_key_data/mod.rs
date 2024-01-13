use crate::data_sources::sql::{SqlDataSource, SqlValueEnum};
use log::{debug, trace};

impl SqlDataSource {
    /// Iterates over the original query's where k/v pairs and update value k/v pairs.
    /// It then generates new where k/v pairs by using the update value k/v pairs.
    /// In short, it swaps the new values with the old values for the new where object.
    pub fn create_update_return_key_data(
        sql_query_where_keys: &Vec<String>,
        sql_query_where_values: &Vec<SqlValueEnum>,
        sql_query_value_keys: &Vec<String>,
        sql_query_values: &Vec<SqlValueEnum>,
    ) -> Result<(Vec<String>, Vec<SqlValueEnum>), async_graphql::Error> {
        debug!("Creating Update Return Key Data");
        trace!("sql_query_where_keys: {:?}", sql_query_where_keys);
        trace!("sql_query_where_values: {:?}", sql_query_where_values);
        trace!("sql_query_value_keys: {:?}", sql_query_value_keys);
        trace!("sql_query_values: {:?}", sql_query_values);
        let mut where_keys = Vec::new();
        let mut where_values = Vec::new();

        for key in sql_query_where_keys {
            if key.contains(".") {
                let split_key: Vec<&str> = key.split(".").collect();
                let key = split_key[1].to_string();
                where_keys.push(key.clone());
            } else {
                where_keys.push(key.clone());
            }

            let index = sql_query_value_keys
                .iter()
                .position(|x| *x.to_string() == key.to_string());

            if index.is_none() {
                let index = sql_query_where_keys
                    .iter()
                    .position(|x| *x.to_string() == key.to_string())
                    .unwrap();

                let value = sql_query_where_values.get(index).map_or_else(
                    || {
                        return Err(async_graphql::Error::new(format!(
                            "Could not find value for key: {}",
                            key
                        )));
                    },
                    |x| Ok(x),
                )?;

                where_values.push(value.clone());
            } else {
                if let Some(value) = sql_query_values.get(index.unwrap()) {
                    where_values.push(value.clone());
                }
            }
        }

        for key in sql_query_value_keys {
            if !where_keys.contains(key) {
                where_keys.push(key.clone());
                let index = sql_query_value_keys
                    .iter()
                    .position(|x| *x.to_string() == key.to_string())
                    .unwrap();
                if let Some(value) = sql_query_values.get(index) {
                    where_values.push(value.clone());
                }
            }
        }

        trace!("Update Return Key Data: {:?}", where_keys);
        trace!("Update Return Value Data: {:?}", where_values);
        Ok((where_keys, where_values))
    }
}
