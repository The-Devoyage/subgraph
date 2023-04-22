use async_graphql::dynamic::ValueAccessor;
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

        let mut keys = vec![];
        let mut values = vec![];

        let input_object = input.object().unwrap();

        for (key, value) in input_object.iter() {
            if key != "query" {
                debug!("Processing Key: {:?}", key);
                keys.push(key.to_string());
                debug!("Processing Value: {:?}", value.string());

                let field = ServiceEntity::get_field(entity, key);

                if field.is_none() {
                    panic!("Field not found: {:?}", key);
                }

                match field.unwrap().scalar {
                    ScalarOptions::String => {
                        values.push(SqlValueEnum::String(value.string().unwrap().to_string()));
                    }
                    ScalarOptions::Int => {
                        values.push(SqlValueEnum::Int(value.i64().unwrap() as i32));
                    }
                    ScalarOptions::Boolean => {
                        values.push(SqlValueEnum::Bool(value.boolean().unwrap()));
                    }
                    _ => {
                        panic!("Unsupported Scalar Type");
                    }
                }
            }
        }

        let query = match resolver_type {
            ResolverType::FindOne => {
                SqlDataSource::create_find_one_query(table_name, keys, &dialect)
            }
            ResolverType::FindMany => {
                SqlDataSource::create_find_many_query(table_name, keys, &dialect)
            }
            ResolverType::CreateOne => {
                SqlDataSource::create_create_one_query(table_name, keys, &dialect)
            }
            ResolverType::UpdateOne => {
                let query_object = input_object.get("query");

                if !query_object.is_some() {
                    panic!("Update One Missing Query");
                }

                let mut query_keys = vec![];

                for (key, value) in query_object.unwrap().object().unwrap().iter() {
                    query_keys.push(key.to_string());
                    if value.string().is_ok() {
                        values.push(SqlValueEnum::String(value.string().unwrap().to_string()));
                    } else if value.i64().is_ok() {
                        values.push(SqlValueEnum::Int(value.i64().unwrap() as i32));
                    } else if value.boolean().is_ok() {
                        values.push(SqlValueEnum::Bool(value.boolean().unwrap()));
                    }
                }

                SqlDataSource::create_update_one_query(table_name, keys, &dialect, query_keys)
            }
        };

        let sql_query = SqlQuery {
            query,
            values,
            table: table_name.to_string(),
        };

        debug!("Query: {:?}", sql_query);

        sql_query
    }

    pub fn get_placeholder(dialect: &DialectEnum, index: Option<i32>) -> String {
        match dialect {
            DialectEnum::POSTGRES => "$".to_string() + &(index.unwrap() + 1).to_string(),
            DialectEnum::MYSQL | DialectEnum::SQLITE => "?".to_string(),
        }
    }

    pub fn create_paramaterized_search_query(keys: Vec<String>, dialect: &DialectEnum) -> String {
        let parameterized_query = if !keys.is_empty() {
            let mut query = String::new();
            query.push_str(" WHERE ");

            for i in 0..keys.len() {
                query.push_str(&keys[i]);
                query.push_str(" = ");
                query.push_str(&SqlDataSource::get_placeholder(dialect, Some(i as i32)));
                if i != keys.len() - 1 {
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
        keys: Vec<String>,
        dialect: &DialectEnum,
    ) -> String {
        let mut query = String::new();
        query.push_str("SELECT * FROM ");
        query.push_str(table_name);

        let parameterized_query = SqlDataSource::create_paramaterized_search_query(keys, dialect);
        query.push_str(&parameterized_query);

        if !query.ends_with(';') {
            query.push(';');
        }

        query
    }

    pub fn create_find_many_query(
        table_name: &str,
        keys: Vec<String>,
        dialect: &DialectEnum,
    ) -> String {
        let mut query = String::new();
        query.push_str("SELECT * FROM ");
        query.push_str(table_name);

        let parameterized_query = SqlDataSource::create_paramaterized_search_query(keys, dialect);
        query.push_str(&parameterized_query);

        if !query.ends_with(';') {
            query.push(';');
        }

        query
    }

    pub fn create_create_one_query(
        table_name: &str,
        keys: Vec<String>,
        dialect: &DialectEnum,
    ) -> String {
        let mut query = String::new();
        query.push_str("INSERT INTO ");
        query.push_str(table_name);
        query.push_str(" (");

        for i in 0..keys.len() {
            query.push_str(&keys[i]);
            if i != keys.len() - 1 {
                query.push_str(", ");
            }
        }

        query.push_str(") VALUES (");

        for i in 0..keys.len() {
            query.push_str(SqlDataSource::get_placeholder(dialect, None).as_str());
            if i != keys.len() - 1 {
                query.push_str(", ");
            }
        }

        query.push_str(")");
        if !query.ends_with(';') {
            query.push(';');
        }
        query
    }

    pub fn create_update_one_query(
        table_name: &str,
        keys: Vec<String>,
        dialect: &DialectEnum,
        query_keys: Vec<String>,
    ) -> String {
        let mut query = String::new();
        query.push_str("UPDATE ");
        query.push_str(table_name);
        query.push_str(" SET ");

        for i in 0..keys.len() {
            query.push_str(&keys[i]);
            query.push_str(" = ");
            query.push_str("?");
            if i != keys.len() - 1 {
                query.push_str(", ");
            }
        }

        let parameterized_query =
            SqlDataSource::create_paramaterized_search_query(query_keys, dialect);
        query.push_str(&parameterized_query);

        query.push_str(" LIMIT 1");

        if !query.ends_with(';') {
            query.push(';');
        }

        query
    }
}
