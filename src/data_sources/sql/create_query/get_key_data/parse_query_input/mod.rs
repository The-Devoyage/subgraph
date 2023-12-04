use std::str::FromStr;

use bson::Bson;
use log::{debug, error};

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum, entities::ServiceEntityConfig, SubGraphConfig,
    },
    data_sources::sql::{create_query::JoinClauses, SqlDataSource, SqlValueEnum},
    utils::clean_string::clean_string,
};

impl SqlDataSource {
    /// Creates vectors of keys and values parsed from the user provided input.
    pub fn parse_query_input(
        value: &Bson,
        mut where_keys: Vec<String>,
        mut where_values: Vec<SqlValueEnum>,
        dialect: &DialectEnum,
        entity: &ServiceEntityConfig,
        subgraph_config: &SubGraphConfig,
    ) -> Result<(Vec<String>, Vec<SqlValueEnum>, JoinClauses), async_graphql::Error> {
        debug!("Parsing Query Input: {:?}", value);
        let query_object = value.as_document();
        let mut join_clauses = JoinClauses(Vec::new());

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

            debug!("Parsing Query Key: {:?}", key);
            let field = ServiceEntityConfig::get_field(entity.clone(), key.to_string())?;

            // If database table is specified, use the table name as the prefix in the search
            // query: SELECT * FROM todo WHERE todo_table.id = $1;
            let mut where_key_prefix = if entity.data_source.is_some() {
                let ds = entity.data_source.clone().unwrap();
                if ds.table.is_some() {
                    ds.table.unwrap()
                } else {
                    entity.clone().name
                }
            } else {
                entity.clone().name
            };

            // If the field is a eager loaded field, get the correct prefix
            if field.eager.is_some() {
                if field.as_type.is_none() {
                    error!("As type required for eager loading: {:?}", field);
                    return Err(async_graphql::Error::new(format!(
                        "As type required for eager loading: {:?}",
                        field,
                    )));
                }
                where_key_prefix = field.as_type.clone().unwrap();

                // Create the join clauses, to be used later.
                let join_clause = format!(
                    " JOIN {} ON {}.{} = {}.{} ",
                    field.as_type.clone().unwrap(),
                    field.as_type.unwrap(),
                    field.join_on.clone().unwrap(),
                    entity.name.clone(),
                    field.join_from.unwrap_or(field.name)
                );
                join_clauses.0.push(join_clause);
            }

            // If the field is eager loaded, we can assume it is a object with many fields. Iterate
            // over the fields and return the keys.
            // Else, just return the key as is.
            if field.eager.is_some() {
                let eager_input = value.as_document();
                if eager_input.is_none() {
                    let where_key = format!("{}.{}", where_key_prefix, field.join_on.unwrap());
                    where_keys.push(where_key);
                    let parsed_where_values = SqlDataSource::get_query_where_values(value, dialect);
                    for value in parsed_where_values {
                        where_values.push(value);
                    }
                    continue;
                }
                for (key, nested_value) in eager_input.unwrap().iter() {
                    let where_key = format!("{}.{}", where_key_prefix, key.to_string());
                    where_keys.push(where_key);
                    let parsed_where_values =
                        SqlDataSource::get_query_where_values(nested_value, dialect);
                    for value in parsed_where_values {
                        where_values.push(value);
                    }
                }
            } else {
                let where_key = format!("{}.{}", where_key_prefix, key.to_string());
                where_keys.push(where_key);
                let parsed_where_values = SqlDataSource::get_query_where_values(value, dialect);
                for value in parsed_where_values {
                    where_values.push(value);
                }
            }
        }

        debug!("Where Keys: {:?}", where_keys);
        debug!("Where Values: {:?}", where_values);

        Ok((where_keys, where_values, join_clauses))
    }

    pub fn get_query_where_values(value: &Bson, dialect: &DialectEnum) -> Vec<SqlValueEnum> {
        debug!("Getting Query Where Values: {:?}", value);
        let mut where_values = Vec::new();

        if value.as_array().is_some() {
            debug!("Parsing Values as Array");
            let value = value.as_array().unwrap();
            if value[0].as_document().is_some() {
                debug!("Receiving document type");
                let value = value[0].as_document().unwrap();
                for (_key, value) in value.iter() {
                    let where_value = SqlDataSource::get_query_where_values(value, dialect);
                    for value in where_value {
                        where_values.push(value);
                    }
                }
            }
            if value[0].as_str().is_some() {
                debug!("Receiving string type");
                // Check if all values are UUIDs
                let is_valid = value.iter().all(|x| {
                    let cleaned_value = clean_string(&x.to_string());
                    match uuid::Uuid::parse_str(&cleaned_value) {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                });

                if is_valid {
                    debug!("Parsing Values as UUIDs");
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
                        debug!("Parsing Values as Dates");
                        let values = value
                            .iter()
                            .map(|x| {
                                let cleaned_value = clean_string(&x.to_string());
                                chrono::DateTime::from_str(&cleaned_value).unwrap()
                            })
                            .collect();
                        where_values.push(SqlValueEnum::DateTimeList(values));
                    } else {
                        debug!("Parsing Values as Strings");
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
            debug!("Parsing Value: {:?}", value);
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
                where_values.push(SqlValueEnum::Int(value.as_i64().unwrap() as i32));
            } else if value.as_bool().is_some() {
                where_values.push(SqlValueEnum::Bool(value.as_bool().unwrap()));
            } else if value.as_datetime().is_some() {
                where_values.push(SqlValueEnum::DateTime(
                    value.as_datetime().unwrap().to_chrono(),
                ));
            }
        };

        debug!("Parsed Query Where Values: {:?}", where_values);
        where_values
    }
}
