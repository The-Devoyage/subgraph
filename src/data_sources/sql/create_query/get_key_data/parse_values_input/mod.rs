use std::str::FromStr;

use bson::Bson;
use log::{debug, error};

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum,
        entities::{
            service_entity_field::ServiceEntityFieldConfig, ScalarOptions, ServiceEntityConfig,
        },
    },
    data_sources::sql::{SqlDataSource, SqlValueEnum},
    graphql::schema::ResolverType,
};

impl SqlDataSource {
    /// Converts the `input.values` struct provided by the client.
    pub fn parse_values_input(
        value: &Bson,
        mut where_keys: Vec<String>,
        mut where_values: Vec<SqlValueEnum>,
        mut value_keys: Vec<String>,
        mut values: Vec<SqlValueEnum>,
        entity: &ServiceEntityConfig,
        resolver_type: &ResolverType,
        dialect: &DialectEnum,
    ) -> Result<
        (
            Vec<String>,
            Vec<SqlValueEnum>,
            Vec<String>,
            Vec<SqlValueEnum>,
        ),
        async_graphql::Error,
    > {
        debug!("Parsing Values Input: {:?}", value);
        let values_object = value.as_document();

        if values_object.is_none() {
            return Err(async_graphql::Error::new("Invalid Values Object"));
        }

        for (key, value) in values_object.unwrap().iter() {
            debug!("Processing Key: {:?}", key);
            debug!("Processing Value: {:?}", value.to_string());

            let field = ServiceEntityConfig::get_field(entity.clone(), key.to_string());

            if field.is_err() {
                error!("Field {} does not exist on entity {}", key, entity.name);
                return Err(async_graphql::Error::new(format!(
                    "Field {} does not exist on entity {}",
                    key, entity.name
                )));
            }

            let is_where_clause = match resolver_type {
                ResolverType::FindOne | ResolverType::FindMany => true,
                ResolverType::CreateOne | ResolverType::UpdateOne | ResolverType::UpdateMany => {
                    false
                }
                _ => {
                    error!("Resolver type {:?} is not supported", resolver_type);
                    return Err(async_graphql::Error::new(format!(
                        "Resolver type {:?} is not supported",
                        resolver_type
                    )));
                }
            };
            let ServiceEntityFieldConfig { scalar, .. } = field.unwrap();
            let list = value.as_array().is_some();

            match scalar {
                ScalarOptions::String => {
                    if list {
                        let value = value.as_array();
                        if value.is_some() {
                            let key = key.to_string();
                            let values = value
                                .unwrap()
                                .iter()
                                .map(|x| x.as_str().unwrap().to_string())
                                .collect();
                            if is_where_clause {
                                where_keys.push(key);
                                where_values.push(SqlValueEnum::StringList(values));
                            }
                        }
                    } else {
                        let value = value.as_str();
                        if value.is_some() {
                            if is_where_clause {
                                where_keys.push(key.to_string());
                                where_values.push(SqlValueEnum::String(value.unwrap().to_string()));
                            } else {
                                value_keys.push(key.to_string());
                                values.push(SqlValueEnum::String(value.unwrap().to_string()));
                            }
                        }
                    }
                }
                ScalarOptions::Int => {
                    if list {
                        let value = value.as_array();
                        if value.is_some() {
                            let key = key.to_string();
                            let values =
                                value.unwrap().iter().map(|x| x.as_i32().unwrap()).collect();
                            if is_where_clause {
                                where_keys.push(key);
                                where_values.push(SqlValueEnum::IntList(values));
                            }
                        }
                    } else {
                        let value = value.as_i32();
                        if value.is_some() {
                            if is_where_clause {
                                where_keys.push(key.to_string());
                                where_values.push(SqlValueEnum::Int(value.unwrap()));
                            } else {
                                value_keys.push(key.to_string());
                                values.push(SqlValueEnum::Int(value.unwrap()));
                            }
                        }
                    }
                }
                ScalarOptions::Boolean => {
                    if list {
                        let value = value.as_array();
                        if value.is_some() {
                            let key = key.to_string();
                            let values = value
                                .unwrap()
                                .iter()
                                .map(|x| x.as_bool().unwrap())
                                .collect();
                            if is_where_clause {
                                where_keys.push(key);
                                where_values.push(SqlValueEnum::BoolList(values));
                            }
                        }
                    } else {
                        let value = value.as_bool();
                        if value.is_some() {
                            if is_where_clause {
                                where_keys.push(key.to_string());
                                where_values.push(SqlValueEnum::Bool(value.unwrap()));
                            } else {
                                value_keys.push(key.to_string());
                                values.push(SqlValueEnum::Bool(value.unwrap()));
                            }
                        }
                    }
                }
                ScalarOptions::UUID => {
                    if list {
                        let value = value.as_array();
                        if value.is_some() {
                            let key = key.to_string();
                            let values = value
                                .unwrap()
                                .iter()
                                .map(|x| {
                                    let x = x.as_str().unwrap_or("");
                                    let uuid = uuid::Uuid::parse_str(x);
                                    if uuid.is_ok() {
                                        uuid.unwrap()
                                    } else {
                                        uuid::Uuid::nil()
                                    }
                                })
                                .collect();
                            if is_where_clause {
                                where_keys.push(key);
                                where_values.push(SqlValueEnum::UUIDList(values));
                            }
                        }
                    } else {
                        let value = uuid::Uuid::parse_str(value.as_str().unwrap())
                            .map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
                        if is_where_clause {
                            where_keys.push(key.to_string());
                            where_values.push(SqlValueEnum::UUID(value));
                        } else {
                            value_keys.push(key.to_string());
                            // If SQLITE Dialect, push as string
                            match dialect {
                                DialectEnum::SQLITE => {
                                    values.push(SqlValueEnum::String(value.to_string()));
                                }
                                _ => {
                                    values.push(SqlValueEnum::UUID(value));
                                }
                            }
                        }
                    }
                }
                ScalarOptions::DateTime => {
                    if list {
                        let value = value.as_array();
                        if value.is_some() {
                            let key = key.to_string();
                            //check that all values are valid date times
                            let is_dates_valid = value.unwrap().iter().all(|x| {
                                let x = x.as_str().unwrap_or("");
                                let date_time = chrono::DateTime::<chrono::Utc>::from_str(x);
                                date_time.is_ok()
                            });
                            if !is_dates_valid {
                                return Err(async_graphql::Error::new(
                                    "Invalid DateTime String in Vector",
                                ));
                            }
                            let values = value
                                .unwrap()
                                .iter()
                                .map(|x| {
                                    let x = x.as_str().unwrap_or("");
                                    let date_time = chrono::DateTime::from_str(x);
                                    date_time.unwrap()
                                })
                                .collect();
                            if is_where_clause {
                                where_keys.push(key);
                                where_values.push(SqlValueEnum::DateTimeList(values));
                            }
                        }
                    } else {
                        let date_time = match value.as_str() {
                            Some(dt) => dt,
                            None => {
                                return Err(async_graphql::Error::new("Invalid DateTime String"))
                            }
                        };
                        let date_time = match chrono::DateTime::from_str(date_time) {
                            Ok(dt) => dt,
                            Err(_) => {
                                return Err(async_graphql::Error::new("Failed to parse DateTime"))
                            }
                        };
                        if is_where_clause {
                            where_keys.push(key.to_string());
                            where_values.push(SqlValueEnum::DateTime(date_time));
                        } else {
                            value_keys.push(key.to_string());
                            values.push(SqlValueEnum::DateTime(date_time));
                        }
                    }
                }
                _ => {
                    error!("Unsupported Scalar Type");
                    return Err(async_graphql::Error::new("Unsupported Scalar Type"));
                }
            }
        }

        debug!("Where Keys: {:?}", where_keys);
        debug!("Where Values: {:?}", where_values);
        debug!("Value Keys: {:?}", value_keys);
        debug!("Values: {:?}", values);

        Ok((where_keys, where_values, value_keys, values))
    }
}
