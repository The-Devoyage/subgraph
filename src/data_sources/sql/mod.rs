use std::{path::Path, str::FromStr};

use async_graphql::dynamic::FieldValue;
use bson::{to_document, Document};
use log::{debug, error, info, trace};
use sqlx::{sqlite::SqliteConnectOptions, MySql, Pool, Postgres, Sqlite};

use crate::{
    cli_args::CliArgs,
    configuration::subgraph::{
        data_sources::sql::{DialectEnum, SqlDataSourceConfig},
        entities::ServiceEntityConfig,
        SubGraphConfig,
    },
    graphql::{
        entity::create_return_types::{ResolverResponse, ResolverResponseMeta},
        schema::create_auth_service::TokenData,
    },
    resolver_type::ResolverType,
};

use super::DataSource;

pub mod create_query;
pub mod services;

#[derive(Debug, Clone)]
pub struct SqlDataSource {
    pub pool: PoolEnum,
    pub config: SqlDataSourceConfig,
    pub subgraph_config: SubGraphConfig,
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
    UUID(uuid::Uuid),
    UUIDList(Vec<uuid::Uuid>),
    DateTime(chrono::DateTime<chrono::Utc>),
    DateTimeList(Vec<chrono::DateTime<chrono::Utc>>),
    ObjectID(String),
    ObjectIDList(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct SqlQuery {
    query: String,
    count_query: Option<String>,
    identifier_query: Option<String>,
    values: Vec<SqlValueEnum>,
    where_values: Vec<SqlValueEnum>,
    table: String,
}

impl SqlDataSource {
    pub async fn init(
        sql_data_source_config: &SqlDataSourceConfig,
        args: &CliArgs,
        subgraph_config: SubGraphConfig,
    ) -> DataSource {
        debug!("Initializing SQL Data Source");

        // Create the pool
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
                    info!("Running Migrations: {:?}", &path);

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
                            error!("Migrations Failed: {:?}", e);
                        }
                    }
                }
            } else if migrate == "revert" {
                //TODO:
            }
        }

        DataSource::SQL(SqlDataSource {
            pool,
            config: sql_data_source_config.clone(),
            subgraph_config,
        })
    }

    pub async fn execute_operation<'a>(
        data_source: &DataSource,
        input: Document,
        entity: ServiceEntityConfig,
        resolver_type: ResolverType,
        subgraph_config: &SubGraphConfig,
        token_data: &Option<TokenData>,
    ) -> Result<Option<FieldValue<'a>>, async_graphql::Error> {
        debug!("Executing SQL Operation");

        let data_source = match data_source {
            DataSource::SQL(ds) => ds,
            _ => unreachable!(),
        };

        let entity_data_source = ServiceEntityConfig::get_entity_data_source(&entity);

        let table;

        // If the entity has a data source, use that table name
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
            input.clone(),
            resolver_type,
            &table,
            data_source.config.dialect.clone(),
            &entity,
            &subgraph_config,
        )?;

        let user_uuid = if token_data.is_some() {
            Some(token_data.as_ref().unwrap().user_uuid.to_string())
        } else {
            None
        };

        // Return the result from the database as a FieldValue
        match resolver_type {
            ResolverType::FindOne => {
                let result = services::Services::find_one(&data_source.pool, &query).await?;
                let res = ResolverResponse {
                    data: vec![FieldValue::owned_any(result)],
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: 1,
                        total_count: 1,
                        page: 1,
                        total_pages: 1,
                        user_uuid,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::FindMany => {
                let (entities, total_count) =
                    services::Services::find_many(&data_source.pool, &query).await?;
                let count = entities.len();
                let opts_doc = if input.clone().get("opts").is_some() {
                    trace!("opts: {:?}", input.get("opts").unwrap());
                    to_document(input.get("opts").unwrap()).unwrap()
                } else {
                    let mut d = Document::new();
                    d.insert("per_page", 10);
                    d.insert("page", 1);
                    trace!("created opts: {:?}", d);
                    d
                };
                trace!("opts_doc: {:?}", opts_doc);
                let page = opts_doc
                    .get_i64("page")
                    .unwrap_or(opts_doc.get_i32("page").unwrap_or(1) as i64);
                let per_page = opts_doc
                    .get_i64("per_page")
                    .unwrap_or(opts_doc.get_i32("per_page").unwrap_or(10) as i64);
                trace!("per_page: {:?}", per_page);
                let res = ResolverResponse {
                    data: entities
                        .into_iter()
                        .map(|row| FieldValue::owned_any(row))
                        .collect(),
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: count as i64,
                        total_count: total_count.0,
                        page,
                        total_pages: if per_page == -1 {
                            1
                        } else {
                            (total_count.0 / per_page) + 1
                        },
                        user_uuid,
                    },
                };

                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::CreateOne => {
                let result = services::Services::create_one(
                    &entity,
                    &data_source.pool,
                    &query,
                    data_source.config.dialect.clone(),
                    &subgraph_config,
                )
                .await?;

                let res = ResolverResponse {
                    data: vec![FieldValue::owned_any(result)],
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: 1,
                        total_count: 1,
                        page: 1,
                        total_pages: 1,
                        user_uuid,
                    },
                };

                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::UpdateOne => {
                let result =
                    services::Services::update_one(&entity, &data_source.pool, &query).await?;
                let res = ResolverResponse {
                    data: vec![FieldValue::owned_any(result)],
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: 1,
                        total_count: 1,
                        page: 1,
                        total_pages: 1,
                        user_uuid,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::UpdateMany => {
                let results =
                    services::Services::update_many(&entity, &data_source.pool, &query).await?;
                let count = results.len();
                let res = ResolverResponse {
                    data: results
                        .into_iter()
                        .map(|row| FieldValue::owned_any(row))
                        .collect(),
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: count as i64,
                        total_count: count as i64,
                        page: 1,
                        total_pages: 1,
                        user_uuid,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            _ => panic!("Invalid resolver type"),
        }
    }
}
