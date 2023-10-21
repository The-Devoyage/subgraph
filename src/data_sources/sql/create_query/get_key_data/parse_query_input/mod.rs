use bson::Bson;
use log::{debug, error};

use crate::{
    data_sources::sql::{SqlDataSource, SqlValueEnum},
    utils::clean_string::clean_string,
};

impl SqlDataSource {
    pub fn parse_query_input(
        value: &Bson,
        mut where_keys: Vec<String>,
        mut where_values: Vec<SqlValueEnum>,
    ) -> Result<(Vec<String>, Vec<SqlValueEnum>), async_graphql::Error> {
        debug!("Parsing Query Input: {:?}", value);
        let query_object = value.as_document();

        if query_object.is_none() {
            error!("Invalid Query Object: {:?}", value);
            return Err(async_graphql::Error::new("Invalid Query Object"));
        }

        let excluded_keys = vec!["OR".to_string(), "AND".to_string()];

        for (key, value) in query_object.unwrap().iter() {
            if excluded_keys.contains(&key) {
                continue;
            }

            where_keys.push(key.to_string());

            if value.as_array().is_some() {
                let value = value.as_array().unwrap();
                if value[0].as_str().is_some() {
                    let values = value
                        .iter()
                        .map(|x| x.as_str().unwrap().to_string())
                        .collect();
                    where_values.push(SqlValueEnum::StringList(values));
                } else if value[0].as_i32().is_some() {
                    let values = value.iter().map(|x| x.as_i32().unwrap()).collect();
                    where_values.push(SqlValueEnum::IntList(values));
                } else if value[0].as_bool().is_some() {
                    let values = value.iter().map(|x| x.as_bool().unwrap()).collect();
                    where_values.push(SqlValueEnum::BoolList(values));
                }
            } else {
                if value.as_str().is_some() {
                    let cleaned_value = clean_string(&value.to_string());
                    where_values.push(SqlValueEnum::String(cleaned_value));
                } else if value.as_i32().is_some() {
                    where_values.push(SqlValueEnum::Int(value.as_i32().unwrap()));
                } else if value.as_bool().is_some() {
                    where_values.push(SqlValueEnum::Bool(value.as_bool().unwrap()));
                }
            }
        }

        debug!("Where Keys: {:?}", where_keys);
        debug!("Where Values: {:?}", where_values);

        Ok((where_keys, where_values))
    }
}
