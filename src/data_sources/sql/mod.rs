use std::{path::Path, str::FromStr};

use async_graphql::dynamic::FieldValue;
use bson::Document;
use log::{debug, error, info};
use sqlx::{sqlite::SqliteConnectOptions, MySql, Pool, Postgres, Sqlite};

use crate::{
    cli_args::CliArgs,
    configuration::subgraph::{
        data_sources::sql::{DialectEnum, SqlDataSourceConfig},
        entities::ServiceEntityConfig,
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
    StringList(Vec<String>),
    IntList(Vec<i32>),
    BoolList(Vec<bool>),
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
    pub async fn init(sql_data_source_config: &SqlDataSourceConfig, args: &CliArgs) -> DataSource {
        debug!("Initializing SQL Data Source");

        let pool: PoolEnum = match sql_data_source_config.dialect {
            DialectEnum::SQLITE => {
                debug!("Creating SQLite Pool: {:?}", &sql_data_source_config.uri);

                let options = if let Some(extensions) = &sql_data_source_config.sqlite_extensions {
                    debug!("Creating SQLite Pool with Extensions: {:?}", &extensions);
                    let mut options = SqliteConnectOptions::from_str(&sql_data_source_config.uri)
                        .expect("Failed to create SqliteConnectOptions with extensions");
                    for extension in extensions {
                        options = options.extension(extension.clone());
                    }
                    options
                } else {
                    SqliteConnectOptions::from_str(&sql_data_source_config.uri)
                        .expect("Failed to create SqliteConnectOptions")
                };

                let pool = sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect_with(options)
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

        if let Some(migrate) = &args.migrate {
            if migrate == "run" {
                let path = sql_data_source_config.migrations_path.clone();
                if path.is_some() {
                    let path = path.unwrap();
                    debug!("Running Migrations: {:?}", &path);

                    let migration = sqlx::migrate::Migrator::new(Path::new(&path)).await;
                    match migration {
                        Ok(migration) => match &pool {
                            PoolEnum::MySql(pool) => {
                                let migration_completed = migration.run(pool).await;
                                match migration_completed {
                                    Ok(_) => {
                                        info!("Migration Complete");
                                    }
                                    Err(e) => {
                                        error!("MySQL Migration Error: {:?}", e);
                                    }
                                }
                            }
                            PoolEnum::Postgres(pool) => {
                                let completed = migration.run(pool).await;
                                match completed {
                                    Ok(_) => {
                                        info!("Migration Complete");
                                    }
                                    Err(e) => {
                                        error!("Postgres Migration Error: {:?}", e);
                                    }
                                }
                            }
                            PoolEnum::SqLite(pool) => {
                                let completed = migration.run(pool).await;
                                match completed {
                                    Ok(_) => {
                                        info!("Migration Complete");
                                    }
                                    Err(e) => {
                                        error!("SQLITE Migration Error: {:?}", e);
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            debug!("Error: {:?}", e);
                        }
                    }
                }
            } else if migrate == "revert" {
            }
        }

        DataSource::SQL(SqlDataSource {
            pool,
            config: sql_data_source_config.clone(),
        })
    }

    pub async fn execute_operation<'a>(
        data_source: &DataSource,
        input: Document,
        entity: ServiceEntityConfig,
        resolver_type: ResolverType,
    ) -> Result<FieldValue<'a>, async_graphql::Error> {
        debug!("Executing SQL Operation");

        let data_source = match data_source {
            DataSource::SQL(ds) => ds,
            _ => unreachable!(),
        };

        let entity_data_source = ServiceEntityConfig::get_entity_data_source(&entity);

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
        )?;

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
                    &entity,
                    &data_source.pool,
                    &query,
                    data_source.config.dialect.clone(),
                )
                .await?;
                Ok(FieldValue::owned_any(result))
            }
            ResolverType::UpdateOne => {
                let result = services::Services::update_one(
                    &entity,
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
