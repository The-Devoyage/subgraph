use log::debug;

use crate::data_sources::sql::{PoolEnum, SqlQuery, SqlValueEnum};

use super::{ResponseRow, Services};

impl Services {
    pub async fn find_one(
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
    ) -> Result<ResponseRow, async_graphql::Error> {
        debug!("Executing Find One Query: {:?}", sql_query);
        match pool_enum {
            PoolEnum::MySql(pool) => {
                debug!("Executing MYSQL Query");
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(value) => {
                            query = query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            query = query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            query = query.bind(value);
                        }
                    }
                }

                let row = query.fetch_one(pool).await?;

                debug!("DB Row: {:?}", row);
                Ok(ResponseRow::MySql(row))
            }
            PoolEnum::Postgres(pool) => {
                debug!("Executing POSTGRES Query");
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(value) => {
                            query = query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            query = query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            query = query.bind(value);
                        }
                    }
                }

                let row = query.fetch_one(pool).await?;

                Ok(ResponseRow::Postgres(row))
            }
            PoolEnum::SqLite(pool) => {
                debug!("Executing SQLITE Query");
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(value) => {
                            query = query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            query = query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            query = query.bind(value);
                        }
                    }
                }

                let row = query.fetch_one(pool).await?;

                Ok(ResponseRow::SqLite(row))
            }
        }
    }
}
