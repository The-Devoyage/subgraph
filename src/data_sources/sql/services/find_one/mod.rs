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

                for value in &sql_query.where_values {
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

                let row = query.fetch_optional(pool).await?;

                debug!("DB Row: {:?}", row);

                if row.is_none() {
                    return Err(async_graphql::Error::from("Not Found"));
                }

                Ok(ResponseRow::MySql(row.unwrap()))
            }
            PoolEnum::Postgres(pool) => {
                debug!("Executing POSTGRES Query");
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.where_values {
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

                let row = query.fetch_optional(pool).await?;

                if row.is_none() {
                    return Err(async_graphql::Error::from("Not Found"));
                }

                Ok(ResponseRow::Postgres(row.unwrap()))
            }
            PoolEnum::SqLite(pool) => {
                debug!("Executing SQLITE Query");
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.where_values {
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

                let row = query.fetch_optional(pool).await?;

                if row.is_none() {
                    return Err(async_graphql::Error::from("Not Found"));
                }

                Ok(ResponseRow::SqLite(row.unwrap()))
            }
        }
    }
}
