use std::str::FromStr;

use bson::Bson;
use log::{debug, error};

use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::sql::{SqlDataSource, SqlValueEnum},
    utils::clean_string::clean_string,
};

impl SqlDataSource {
    /// Creates vectors of keys and values parsed from the user provided input.
    pub fn parse_query_input(
        value: &Bson,
        mut where_keys: Vec<String>,
        mut where_values: Vec<SqlValueEnum>,
        dialect: &DialectEnum,
    ) -> Result<(Vec<String>, Vec<SqlValueEnum>), async_graphql::Error> {
        debug!("Parsing Query Input: {:?}", value);
        let query_object = value.as_document();

        if query_object.is_none() {
            error!("Invalid Query Object: {:?}", value);
            return Err(async_graphql::Error::new("Invalid Query Object"));
        }

        let excluded_keys = vec!["OR".to_string(), "AND".to_string()];

        // Iterate through the query object and create a vector of keys and values
        for (key, value) in query_object.unwrap().iter() {
            if excluded_keys.contains(&key) {
                continue;
            }

            where_keys.push(key.to_string());

            // HACK: This should be more efficient. Rather than checking each value, we should
            // already know what type of value we are expecting based on the key.
            if value.as_array().is_some() {
                let value = value.as_array().unwrap();
                if value[0].as_str().is_some() {
                    // Check if all values are UUIDs
                    let is_valid = value.iter().all(|x| {
                        let cleaned_value = clean_string(&x.to_string());
                        match uuid::Uuid::parse_str(&cleaned_value) {
                            Ok(_) => true,
                            Err(_) => false,
                        }
                    });

                    if is_valid {
                        let values = value
                            .iter()
                            .map(|x| {
                                let cleaned_value = clean_string(&x.to_string());
                                uuid::Uuid::parse_str(&cleaned_value).unwrap()
                            })
                            .collect::<Vec<uuid::Uuid>>();

                        // If dialect is SQLITE, use strings, as SQLITE does not support UUIDs with
                        // SQLX
                        match dialect {
                            DialectEnum::SQLITE => {
                                where_values.push(SqlValueEnum::StringList(
                                    values.iter().map(|x| x.to_string()).collect(),
                                ));
                            }
                            _ => where_values.push(SqlValueEnum::UUIDList(values)),
                        }
                    } else {
                        let is_valid_dates = value.iter().all(|x| {
                            let cleaned_value = clean_string(&x.to_string());
                            match chrono::DateTime::<chrono::Utc>::from_str(&cleaned_value) {
                                Ok(_) => true,
                                Err(_) => false,
                            }
                        });
                        if is_valid_dates {
                            let values = value
                                .iter()
                                .map(|x| {
                                    let cleaned_value = clean_string(&x.to_string());
                                    chrono::DateTime::from_str(&cleaned_value).unwrap()
                                })
                                .collect();
                            where_values.push(SqlValueEnum::DateTimeList(values));
                        } else {
                            where_values.push(SqlValueEnum::StringList(
                                value.iter().map(|x| clean_string(&x.to_string())).collect(),
                            ));
                        }
                    }
                } else if value[0].as_i32().is_some() || value[0].as_i64().is_some() {
                    let values = value.iter().map(|x| x.as_i32().unwrap()).collect();
                    where_values.push(SqlValueEnum::IntList(values));
                } else if value[0].as_bool().is_some() {
                    let values = value.iter().map(|x| x.as_bool().unwrap()).collect();
                    where_values.push(SqlValueEnum::BoolList(values));
                }
            } else {
                if value.as_str().is_some() {
                    debug!("Parsing String: {:?}", value);
                    let cleaned_value = clean_string(&value.to_string());
                    match uuid::Uuid::parse_str(&cleaned_value) {
                        Ok(uuid) => {
                            debug!("Parsed UUID: {:?}", uuid);

                            // Sqlite does not support UUIDs
                            match dialect {
                                DialectEnum::SQLITE | DialectEnum::MYSQL => {
                                    where_values.push(SqlValueEnum::String(cleaned_value));
                                    continue;
                                }
                                _ => where_values.push(SqlValueEnum::UUID(uuid)),
                            }
                        }
                        Err(_) => match chrono::DateTime::from_str(&cleaned_value) {
                            Ok(date) => {
                                debug!("Parsed Date: {:?}", date);
                                where_values.push(SqlValueEnum::DateTime(date));
                            }
                            Err(_) => {
                                debug!("Parsed String: {:?}", cleaned_value);
                                where_values.push(SqlValueEnum::String(cleaned_value));
                            }
                        },
                    }
                } else if value.as_i32().is_some() {
                    where_values.push(SqlValueEnum::Int(value.as_i32().unwrap()));
                } else if value.as_i64().is_some() {
                    // HACK: Should support Int and BitInt
                    where_values.push(SqlValueEnum::Int(value.as_i64().unwrap() as i32));
                } else if value.as_bool().is_some() {
                    where_values.push(SqlValueEnum::Bool(value.as_bool().unwrap()));
                } else if value.as_datetime().is_some() {
                    where_values.push(SqlValueEnum::DateTime(
                        value.as_datetime().unwrap().to_chrono(),
                    ));
                }
            }
        }

        debug!("Where Keys: {:?}", where_keys);
        debug!("Where Values: {:?}", where_values);

        Ok((where_keys, where_values))
    }
}
