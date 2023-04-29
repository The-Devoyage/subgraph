use log::debug;

use crate::data_sources::sql::{PoolEnum, SqlQuery, SqlValueEnum};

use super::{ResponseRow, Services};

impl Services {
    pub async fn find_many(
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
    ) -> Result<Vec<ResponseRow>, async_graphql::Error> {
        debug!("Executing Find Many Query: {:?}", sql_query);

        match pool_enum {
            PoolEnum::MySql(pool) => {
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

                let rows = query.fetch_all(pool).await?;

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(ResponseRow::MySql(row));
                }
                Ok(response_rows)
            }
            PoolEnum::Postgres(pool) => {
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

                let rows = query.fetch_all(pool).await?;

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(ResponseRow::Postgres(row));
                }
                Ok(response_rows)
            }
            PoolEnum::SqLite(pool) => {
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

                let rows = query.fetch_all(pool).await?;

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(ResponseRow::SqLite(row));
                }
                Ok(response_rows)
            }
        }
    }
}
