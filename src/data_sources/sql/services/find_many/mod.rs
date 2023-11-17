use log::debug;
use sqlx::{mysql::MySqlArguments, MySql};

use crate::data_sources::sql::{PoolEnum, SqlQuery, SqlValueEnum};

use super::{ResponseRow, Services};

impl Services {
    pub async fn find_many(
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
    ) -> Result<Vec<Option<ResponseRow>>, async_graphql::Error> {
        debug!("Executing Find Many Query: {:?}", sql_query);

        match pool_enum {
            PoolEnum::MySql(pool) => {
                let mut query =
                    sqlx::query(&sql_query.query) as sqlx::query::Query<MySql, MySqlArguments>;

                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
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
                        SqlValueEnum::UUID(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid)
                            }
                        }
                        SqlValueEnum::DateTime(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::DateTimeList(values) => {
                            for datetime in values {
                                query = query.bind(datetime)
                            }
                        }
                    }
                }

                let rows = query.fetch_all(pool).await?;

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(Some(ResponseRow::MySql(row)));
                }
                Ok(response_rows)
            }
            PoolEnum::Postgres(pool) => {
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
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
                        SqlValueEnum::UUID(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid)
                            }
                        }
                        SqlValueEnum::DateTime(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::DateTimeList(values) => {
                            for datetime in values {
                                query = query.bind(datetime)
                            }
                        }
                    }
                }

                let rows = query.fetch_all(pool).await?;

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(Some(ResponseRow::Postgres(row)));
                }
                Ok(response_rows)
            }
            PoolEnum::SqLite(pool) => {
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
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
                        SqlValueEnum::UUID(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid)
                            }
                        }
                        SqlValueEnum::DateTime(v) => {
                            query = query.bind(v);
                        }
                        SqlValueEnum::DateTimeList(values) => {
                            for datetime in values {
                                query = query.bind(datetime)
                            }
                        }
                    }
                }

                let rows = query.fetch_all(pool).await?;

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(Some(ResponseRow::SqLite(row)));
                }
                Ok(response_rows)
            }
        }
    }
}
