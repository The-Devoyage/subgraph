use bson::Document;
use log::debug;

use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityField, ScalarOptions, ServiceEntity,
    },
    data_sources::sql::{SqlDataSource, SqlValueEnum},
    graphql::schema::ResolverType,
};

impl SqlDataSource {
    pub fn get_key_data(
        input_object: &Document,
        entity: &ServiceEntity,
        resolver_type: &ResolverType,
    ) -> (
        Vec<String>,
        Vec<SqlValueEnum>,
        Vec<String>,
        Vec<SqlValueEnum>,
    ) {
        debug!("Getting Key Data");
        let mut where_keys = vec![];
        let mut where_values = vec![];
        let mut value_keys = vec![];
        let mut values = vec![];

        for (key, value) in input_object.iter() {
            if key != "query" {
                debug!("Processing Key: {:?}", key);
                debug!("Processing Value: {:?}", value.to_string());

                let field = ServiceEntity::get_field(entity.clone(), key.to_string());

                if field.is_err() {
                    panic!("Field not found: {:?}", key);
                }

                let is_where_clause = match resolver_type {
                    ResolverType::FindOne | ResolverType::FindMany => true,
                    ResolverType::CreateOne
                    | ResolverType::UpdateOne
                    | ResolverType::UpdateMany => false,
                    _ => panic!("Invalid resolver type"),
                };
                let ServiceEntityField { scalar, .. } = field.unwrap();
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
                                    where_values
                                        .push(SqlValueEnum::String(value.unwrap().to_string()));
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
                    _ => {
                        panic!("Unsupported Scalar Type");
                    }
                }
            } else if key == "query" {
                debug!("Processing Where Query");
                let query_object = value.as_document().unwrap();

                for (key, value) in query_object.iter() {
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
                            where_values.push(SqlValueEnum::String(value.to_string()));
                        } else if value.as_i32().is_some() {
                            where_values.push(SqlValueEnum::Int(value.as_i32().unwrap()));
                        } else if value.as_bool().is_some() {
                            where_values.push(SqlValueEnum::Bool(value.as_bool().unwrap()));
                        }
                    }
                }
            }
        }

        debug!("Where Keys: {:?}", where_keys);
        debug!("Where Values: {:?}", where_values);
        debug!("Value Keys: {:?}", value_keys);
        debug!("Values: {:?}", values);

        (where_keys, where_values, value_keys, values)
    }
}
