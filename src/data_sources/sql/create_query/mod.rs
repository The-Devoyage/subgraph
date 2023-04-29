use async_graphql::dynamic::{ObjectAccessor, ValueAccessor};
use log::debug;

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum,
        entities::{ScalarOptions, ServiceEntity},
    },
    data_sources::sql::SqlValueEnum,
    graphql::schema::ResolverType,
};

use super::{SqlDataSource, SqlQuery};

impl SqlDataSource {
    pub fn create_query(
        input: &ValueAccessor<'_>,
        resolver_type: ResolverType,
        table_name: &str,
        dialect: DialectEnum,
        entity: &ServiceEntity,
    ) -> SqlQuery {
        debug!("Creating SQL Query");

        let input_object = input.object().unwrap();

        let (where_keys, where_values, value_keys, values) =
            SqlDataSource::get_key_data(&input_object, entity, &resolver_type);

        let query = match resolver_type {
            ResolverType::FindOne => {
                SqlDataSource::create_find_one_query(table_name, &where_keys, &dialect)
            }
            ResolverType::FindMany => {
                SqlDataSource::create_find_many_query(table_name, &where_keys, &dialect)
            }
            ResolverType::CreateOne => {
                SqlDataSource::create_create_one_query(table_name, &value_keys, &dialect)
            }
            ResolverType::UpdateOne => SqlDataSource::create_update_one_query(
                table_name,
                &value_keys,
                &dialect,
                &where_keys,
            ),
        };

        let sql_query = SqlQuery {
            query,
            where_keys,
            where_values,
            value_keys,
            values,
            table: table_name.to_string(),
        };

        debug!("Query: {:?}", sql_query);

        sql_query
    }

    pub fn get_key_data(
        input_object: &ObjectAccessor,
        entity: &ServiceEntity,
        resolver_type: &ResolverType,
    ) -> (
        Vec<String>,
        Vec<SqlValueEnum>,
        Vec<String>,
        Vec<SqlValueEnum>,
    ) {
        let mut where_keys = vec![];
        let mut where_values = vec![];
        let mut value_keys = vec![];
        let mut values = vec![];

        for (key, value) in input_object.iter() {
            if key != "query" {
                debug!("Processing Key: {:?}", key);
                debug!("Processing Value: {:?}", value.string());

                let field = ServiceEntity::get_field(entity, key);

                if field.is_none() {
                    panic!("Field not found: {:?}", key);
                }

                let is_where_clause = match resolver_type {
                    ResolverType::FindOne | ResolverType::FindMany => true,
                    ResolverType::CreateOne | ResolverType::UpdateOne => false,
                };

                match field.unwrap().scalar {
                    ScalarOptions::String => {
                        if is_where_clause {
                            where_keys.push(key.to_string());
                            where_values
                                .push(SqlValueEnum::String(value.string().unwrap().to_string()));
                        } else {
                            value_keys.push(key.to_string());
                            values.push(SqlValueEnum::String(value.string().unwrap().to_string()));
                        }
                    }
                    ScalarOptions::Int => {
                        if is_where_clause {
                            where_keys.push(key.to_string());
                            where_values.push(SqlValueEnum::Int(value.i64().unwrap() as i32));
                        } else {
                            value_keys.push(key.to_string());
                            values.push(SqlValueEnum::Int(value.i64().unwrap() as i32));
                        }
                    }
                    ScalarOptions::Boolean => {
                        if is_where_clause {
                            where_keys.push(key.to_string());
                            where_values.push(SqlValueEnum::Bool(value.boolean().unwrap()));
                        } else {
                            value_keys.push(key.to_string());
                            values.push(SqlValueEnum::Bool(value.boolean().unwrap()));
                        }
                    }
                    _ => {
                        panic!("Unsupported Scalar Type");
                    }
                }
            } else if key == "query" {
                debug!("Processing Where Query");
                let query_object = value.object().unwrap();

                for (key, value) in query_object.iter() {
                    where_keys.push(key.to_string());
                    if value.string().is_ok() {
                        where_values
                            .push(SqlValueEnum::String(value.string().unwrap().to_string()));
                    } else if value.i64().is_ok() {
                        where_values.push(SqlValueEnum::Int(value.i64().unwrap() as i32));
                    } else if value.boolean().is_ok() {
                        where_values.push(SqlValueEnum::Bool(value.boolean().unwrap()));
                    }
                }
            }
        }

        (where_keys, where_values, value_keys, values)
    }

    pub fn get_placeholder(dialect: &DialectEnum, index: Option<i32>) -> String {
        match dialect {
            DialectEnum::POSTGRES => "$".to_string() + &(index.unwrap() + 1).to_string(),
            DialectEnum::MYSQL | DialectEnum::SQLITE => "?".to_string(),
        }
    }

    pub fn create_where_clause(
        where_keys: &Vec<String>,
        dialect: &DialectEnum,
        offset: Option<i32>,
    ) -> String {
        let parameterized_query = if !where_keys.is_empty() {
            let mut query = String::new();
            query.push_str(" WHERE ");

            for i in 0..where_keys.len() {
                query.push_str(&where_keys[i]);
                query.push_str(" = ");
                let index = if offset.is_some() {
                    Some(i as i32 + offset.unwrap())
                } else {
                    Some(0)
                };
                query.push_str(&SqlDataSource::get_placeholder(dialect, index));
                if i != where_keys.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            query
        } else {
            String::new()
        };
        parameterized_query
    }

    pub fn create_find_one_query(
        table_name: &str,
        keys: &Vec<String>,
        dialect: &DialectEnum,
    ) -> String {
        let mut query = String::new();
        query.push_str("SELECT * FROM ");
        query.push_str(table_name);

        let parameterized_query = SqlDataSource::create_where_clause(keys, dialect, None);
        query.push_str(&parameterized_query);

        if !query.ends_with(';') {
            query.push(';');
        }

        query
    }

    pub fn create_find_many_query(
        table_name: &str,
        where_keys: &Vec<String>,
        dialect: &DialectEnum,
    ) -> String {
        let mut query = String::new();
        query.push_str("SELECT * FROM ");
        query.push_str(table_name);

        let parameterized_query = SqlDataSource::create_where_clause(where_keys, dialect, None);
        query.push_str(&parameterized_query);

        if !query.ends_with(';') {
            query.push(';');
        }

        query
    }

    pub fn create_create_one_query(
        table_name: &str,
        value_keys: &Vec<String>,
        dialect: &DialectEnum,
    ) -> String {
        let mut query = String::new();
        query.push_str("INSERT INTO ");
        query.push_str(table_name);
        query.push_str(" (");

        for i in 0..value_keys.len() {
            query.push_str(&value_keys[i]);
            if i != value_keys.len() - 1 {
                query.push_str(", ");
            }
        }

        query.push_str(") VALUES (");
        for i in 0..value_keys.len() {
            query.push_str(SqlDataSource::get_placeholder(dialect, Some(i as i32)).as_str());
            if i != value_keys.len() - 1 {
                query.push_str(", ");
            }
        }
        query.push_str(")");

        match dialect {
            DialectEnum::POSTGRES => {
                query.push_str(" RETURNING *");
            }
            _ => {}
        }

        if !query.ends_with(';') {
            query.push(';');
        }
        query
    }

    pub fn create_update_one_query(
        table_name: &str,
        value_keys: &Vec<String>,
        dialect: &DialectEnum,
        where_keys: &Vec<String>,
    ) -> String {
        let mut query = String::new();
        query.push_str("UPDATE ");
        query.push_str(table_name);
        query.push_str(" SET ");

        for i in 0..value_keys.len() {
            query.push_str(&value_keys[i]);
            query.push_str(" = ");
            query.push_str(SqlDataSource::get_placeholder(dialect, Some(i as i32)).as_str());
            if i != value_keys.len() - 1 {
                query.push_str(", ");
            }
        }

        let parameterized_query = SqlDataSource::create_where_clause(where_keys, dialect, None);
        query.push_str(&parameterized_query);

        query.push_str(" LIMIT 1");

        if !query.ends_with(';') {
            query.push(';');
        }

        query
    }
}
