use log::debug;

use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::sql::{PoolEnum, SqlDataSource, SqlQuery, SqlValueEnum},
};

use super::{ResponseRow, Services};

impl Services {
    pub async fn create_one(
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
        dialect: DialectEnum,
    ) -> Result<ResponseRow, async_graphql::Error> {
        debug!("Executing Create One Query: {:?}", sql_query);

        match pool_enum {
            PoolEnum::MySql(pool) => {
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for string in values {
                                query = query.bind(string)
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                query = query.bind(int)
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool)
                            }
                        }
                    }
                }

                let last_inserted_id = query.execute(pool).await?.last_insert_id();

                let find_one_query = SqlDataSource::create_find_one_query(
                    &sql_query.table,
                    &vec!["id".to_string()],
                    &dialect,
                    &vec![SqlValueEnum::Int(last_inserted_id as i32)],
                );

                let result = sqlx::query(&find_one_query)
                    .bind(last_inserted_id)
                    .fetch_one(pool)
                    .await?;

                Ok(ResponseRow::MySql(result))
            }
            PoolEnum::Postgres(pool) => {
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for string in values {
                                query = query.bind(string)
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                query = query.bind(int)
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool)
                            }
                        }
                    }
                }

                let result = query.fetch_one(pool).await?;

                Ok(ResponseRow::Postgres(result))
            }
            PoolEnum::SqLite(pool) => {
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for string in values {
                                query = query.bind(string)
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                query = query.bind(int)
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool)
                            }
                        }
                    }
                }

                let last_inserted_rowid = query.execute(pool).await?.last_insert_rowid();

                let find_one_query = SqlDataSource::create_find_one_query(
                    &sql_query.table,
                    &vec!["id".to_string()],
                    &dialect,
                    &vec![SqlValueEnum::Int(last_inserted_rowid as i32)],
                );

                let result = sqlx::query(&find_one_query)
                    .bind(last_inserted_rowid)
                    .fetch_one(pool)
                    .await?;

                Ok(ResponseRow::SqLite(result))
            }
        }
    }
}
