use async_graphql::dynamic::FieldValue;
use bson::Document;
use log::debug;
use sqlx::{MySql, Pool, Postgres, Sqlite};

use crate::{
    configuration::subgraph::{
        data_sources::sql::{DialectEnum, SqlDataSourceConfig},
        entities::ServiceEntity,
    },
    graphql::schema::ResolverType,
};

use super::DataSource;

pub mod create_query;
pub mod services;

#[derive(Debug, Clone)]
pub struct SqlDataSource {
    pub pool: PoolEnum,
    pub config: SqlDataSourceConfig,
}

#[derive(Debug, Clone)]
pub struct TestEnum {
    pub pool: PoolEnum,
}

#[derive(Debug, Clone)]
pub enum PoolEnum {
    MySql(Pool<MySql>),
    Postgres(Pool<Postgres>),
    SqLite(Pool<Sqlite>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SqlValueEnum {
    String(String),
    Int(i32),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct SqlQuery {
    query: String,
    values: Vec<SqlValueEnum>,
    value_keys: Vec<String>,
    where_values: Vec<SqlValueEnum>,
    where_keys: Vec<String>,
    table: String,
}

impl SqlDataSource {
    pub async fn init(sql_data_source_config: &SqlDataSourceConfig) -> DataSource {
        debug!("Initializing SQL Data Source");

        let pool: PoolEnum = match sql_data_source_config.dialect {
            DialectEnum::SQLITE => {
                debug!("Creating SQLite Pool: {:?}", &sql_data_source_config.uri);
                let pool = sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(&sql_data_source_config.uri)
                    .await
                    .unwrap();
                PoolEnum::SqLite(pool)
            }
            DialectEnum::POSTGRES => {
                debug!("Creating Postgres Pool: {:?}", &sql_data_source_config.uri);
                let pool = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&sql_data_source_config.uri)
                    .await
                    .unwrap();
                PoolEnum::Postgres(pool)
            }
            DialectEnum::MYSQL => {
                debug!("Creating MySql Pool: {:?}", &sql_data_source_config.uri);
                let pool = sqlx::mysql::MySqlPoolOptions::new()
                    .max_connections(5)
                    .connect(&sql_data_source_config.uri)
                    .await
                    .unwrap();
                PoolEnum::MySql(pool)
            }
        };

        DataSource::SQL(SqlDataSource {
            pool,
            config: sql_data_source_config.clone(),
        })
    }

    pub async fn execute_operation<'a>(
        data_source: &DataSource,
        input: Document,
        entity: ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<FieldValue<'a>, async_graphql::Error> {
        debug!("Executing SQL Operation");

        let data_source = match data_source {
            DataSource::SQL(ds) => ds,
            _ => unreachable!(),
        };

        let entity_data_source = ServiceEntity::get_entity_data_source(&entity);

        let table;

        if entity_data_source.is_some() {
            if entity_data_source.clone().unwrap().table.is_some() {
                table = entity_data_source.unwrap().table.unwrap();
            } else {
                table = entity.name.to_string();
            }
        } else {
            table = entity.name.to_string();
        }

        let query = SqlDataSource::create_query(
            input,
            resolver_type,
            &table,
            data_source.config.dialect.clone(),
            &entity,
        );

        match resolver_type {
            ResolverType::FindOne => {
                let result = services::Services::find_one(&data_source.pool, &query).await?;
                Ok(FieldValue::owned_any(result))
            }
            ResolverType::FindMany => {
                let results = services::Services::find_many(&data_source.pool, &query).await?;
                Ok(FieldValue::list(
                    results.into_iter().map(|row| FieldValue::owned_any(row)),
                ))
            }
            ResolverType::CreateOne => {
                let result = services::Services::create_one(
                    &data_source.pool,
                    &query,
                    data_source.config.dialect.clone(),
                )
                .await?;
                Ok(FieldValue::owned_any(result))
            }
            ResolverType::UpdateOne => {
                let result = services::Services::update_one(
                    &data_source.pool,
                    &query,
                    data_source.config.dialect.clone(),
                )
                .await?;
                Ok(FieldValue::owned_any(result))
            }
            ResolverType::UpdateMany => {
                let results = services::Services::update_many(
                    &data_source.pool,
                    &query,
                    data_source.config.dialect.clone(),
                )
                .await?;
                Ok(FieldValue::list(
                    results.into_iter().map(|row| FieldValue::owned_any(row)),
                ))
            }
            _ => panic!("Invalid resolver type"),
        }
    }
}
