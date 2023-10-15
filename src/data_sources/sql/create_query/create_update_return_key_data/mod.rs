use crate::data_sources::sql::{SqlDataSource, SqlValueEnum};
use log::debug;

impl SqlDataSource {
    /// Iterates over the original query's where k/v pairs and update value k/v pairs.
    /// It then generates new where k/v pairs by using the update value k/v pairs.
    /// In short, it swaps the new values with the old values for the new where object.
    pub fn create_update_return_key_data(
        sql_query_where_keys: &Vec<String>,
        sql_query_where_values: &Vec<SqlValueEnum>,
        sql_query_value_keys: &Vec<String>,
        sql_query_values: &Vec<SqlValueEnum>,
    ) -> (Vec<String>, Vec<SqlValueEnum>) {
        debug!("Creating Update Return Key Data");
        let mut where_keys = Vec::new();
        let mut where_values = Vec::new();

        for key in sql_query_where_keys {
            where_keys.push(key.clone());
            let index = sql_query_value_keys
                .iter()
                .position(|x| *x.to_string() == key.to_string());

            if index.is_none() {
                let index = sql_query_where_keys
                    .iter()
                    .position(|x| *x.to_string() == key.to_string())
                    .unwrap();
                let value = sql_query_where_values.get(index).unwrap();
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

        debug!("Update Return Key Data: {:?}", where_keys);
        debug!("Update Return Value Data: {:?}", where_values);
        (where_keys, where_values)
    }
}
