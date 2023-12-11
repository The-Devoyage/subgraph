use std::str::FromStr;

use bson::Bson;
use log::{debug, error, trace};

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum, entities::ServiceEntityConfig, SubGraphConfig,
    },
    data_sources::sql::{create_query::JoinClauses, SqlDataSource, SqlValueEnum},
    utils::clean_string::clean_string,
};

impl SqlDataSource {
    /// Converts the types provided from the query input into the expected types for each data
    /// source.
    pub fn get_query_where_values(
        value: &Bson,
        dialect: &DialectEnum,
        parent_key: &str,
        as_type: Option<String>, // Passing the `as_type` allows the nested eager loaded  association
        // to be matched.
        subgraph_config: &SubGraphConfig,
        parent_alias: Option<String>,
    ) -> Result<(Vec<String>, Vec<SqlValueEnum>, JoinClauses), async_graphql::Error> {
        debug!("Getting Query Where Values");
        trace!("From Value: {:?}", value);
        trace!("Parent Key: {:?}", parent_key);
        let mut where_keys = Vec::new();
        let mut where_values = Vec::new();
        let mut join_clauses = JoinClauses(Vec::new());

        if value.as_array().is_some() {
            trace!("Parsing Values as Array");
            let value = value.as_array().unwrap();
            if value[0].as_document().is_some() {
                trace!("Receiving document type");
                let value = value[0].as_document().unwrap();
                for (k, value) in value.iter() {
                    let (wk, wv, jc) = SqlDataSource::get_query_where_values(
                        value,
                        dialect,
                        k,
                        None,
                        subgraph_config,
                        parent_alias.clone(),
                    )?;
                    for value in wv {
                        where_values.push(value);
                    }
                    for key in wk {
                        where_keys.push(key);
                    }
                    for clause in jc.0 {
                        join_clauses.0.push(clause);
                    }
                }
            }
            if value[0].as_str().is_some() {
                trace!("Receiving string type");
                // Check if all values are UUIDs
                let is_valid = value.iter().all(|x| {
                    let cleaned_value = clean_string(&x.to_string());
                    match uuid::Uuid::parse_str(&cleaned_value) {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                });

                if is_valid {
                    trace!("Parsing Values as UUIDs");
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
                            where_keys.push(parent_key.to_string());
                        }
                        _ => {
                            where_values.push(SqlValueEnum::UUIDList(values));
                            where_keys.push(parent_key.to_string());
                        }
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
                        trace!("Parsing Values as Dates");
                        let values = value
                            .iter()
                            .map(|x| {
                                let cleaned_value = clean_string(&x.to_string());
                                chrono::DateTime::from_str(&cleaned_value).unwrap()
                            })
                            .collect();
                        where_values.push(SqlValueEnum::DateTimeList(values));
                        where_keys.push(parent_key.to_string());
                    } else {
                        trace!("Parsing Values as Strings");
                        where_values.push(SqlValueEnum::StringList(
                            value.iter().map(|x| clean_string(&x.to_string())).collect(),
                        ));
                        where_keys.push(parent_key.to_string());
                    }
                }
            } else if value[0].as_i32().is_some() || value[0].as_i64().is_some() {
                let values = value.iter().map(|x| x.as_i32().unwrap()).collect();
                where_values.push(SqlValueEnum::IntList(values));
                where_keys.push(parent_key.to_string());
            } else if value[0].as_bool().is_some() {
                let values = value.iter().map(|x| x.as_bool().unwrap()).collect();
                where_values.push(SqlValueEnum::BoolList(values));
                where_keys.push(parent_key.to_string());
            }
        } else if value.as_document().is_some() {
            trace!("Parsing Nested Eager Field");
            let value = value.as_document().unwrap(); // This should be the nested input.
            let entity = match subgraph_config
                .clone()
                .get_entity(&as_type.clone().unwrap())
            {
                Some(entity) => entity,
                None => {
                    error!("Could not find entity with name: {:?}", as_type.clone());
                    return Err(async_graphql::Error::new(format!(
                        "Could not find entity with name: {}",
                        as_type.unwrap()
                    )));
                }
            };
            let field = ServiceEntityConfig::get_field(entity.clone(), parent_key.to_string())?;
            let where_key_prefix =
                SqlDataSource::get_where_key_prefix(&field, &entity, &subgraph_config)?;
            let join_clause = SqlDataSource::get_join_clause(
                &field,
                &entity,
                subgraph_config,
                parent_alias.clone(),
            )?;
            if join_clause.is_some() {
                join_clauses.0.push(join_clause.unwrap());
            }
            for (k, value) in value.iter() {
                let (where_key, where_value, combined_join_clauses) =
                    SqlDataSource::get_query_where_values(
                        value,
                        dialect,
                        k,
                        field.as_type.clone(),
                        subgraph_config,
                        parent_alias.clone(),
                    )?;
                for value in where_value {
                    where_values.push(value);
                }
                for key in where_key {
                    trace!("Additional Where Key: {:?}", key);
                    let key = format!("{}.{}", where_key_prefix, key);
                    where_keys.push(key);
                }
                for clause in combined_join_clauses.0 {
                    join_clauses.0.push(clause);
                }
            }
        } else {
            trace!("Parsing Value: {:?}", value);
            if value.as_str().is_some() {
                trace!("Parsing String: {:?}", value);
                let cleaned_value = clean_string(&value.to_string());
                match uuid::Uuid::parse_str(&cleaned_value) {
                    Ok(uuid) => {
                        trace!("Parsed UUID: {:?}", uuid);

                        // Sqlite does not support UUIDs
                        match dialect {
                            DialectEnum::SQLITE | DialectEnum::MYSQL => {
                                where_values.push(SqlValueEnum::String(cleaned_value));
                                where_keys.push(parent_key.to_string());
                            }
                            _ => {
                                where_values.push(SqlValueEnum::UUID(uuid));
                                where_keys.push(parent_key.to_string());
                            }
                        }
                    }
                    Err(_) => match chrono::DateTime::from_str(&cleaned_value) {
                        Ok(date) => {
                            trace!("Parsed Date: {:?}", date);
                            where_values.push(SqlValueEnum::DateTime(date));
                            where_keys.push(parent_key.to_string());
                        }
                        Err(_) => {
                            trace!("Parsed String: {:?}", cleaned_value);
                            where_values.push(SqlValueEnum::String(cleaned_value));
                            where_keys.push(parent_key.to_string());
                        }
                    },
                }
            } else if value.as_i32().is_some() {
                where_values.push(SqlValueEnum::Int(value.as_i32().unwrap()));
                where_keys.push(parent_key.to_string());
            } else if value.as_i64().is_some() {
                where_values.push(SqlValueEnum::Int(value.as_i64().unwrap() as i32));
                where_keys.push(parent_key.to_string());
            } else if value.as_bool().is_some() {
                where_values.push(SqlValueEnum::Bool(value.as_bool().unwrap()));
                where_keys.push(parent_key.to_string());
            } else if value.as_datetime().is_some() {
                where_values.push(SqlValueEnum::DateTime(
                    value.as_datetime().unwrap().to_chrono(),
                ));
                where_keys.push(parent_key.to_string());
            }
        };

        trace!("Completed Get Query Where Values");
        trace!("Where Values: {:?}", where_values);
        trace!("Where Keys: {:?}", where_keys);
        trace!("Join Clauses: {:?}", join_clauses);

        Ok((where_keys, where_values, join_clauses))
    }
}
