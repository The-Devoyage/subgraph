use log::{debug, error};
use sqlx::{mysql::MySqlArguments, MySql, Row};

use crate::data_sources::{
    sql::{PoolEnum, SqlQuery, SqlValueEnum},
    TotalCount,
};

use super::{ResponseRow, Services};

impl Services {
    pub async fn find_many(
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
    ) -> Result<(Vec<Option<ResponseRow>>, TotalCount), async_graphql::Error> {
        debug!("Executing Find Many Query: {:?}", sql_query);

        match pool_enum {
            PoolEnum::MySql(pool) => {
                let mut query =
                    sqlx::query(&sql_query.query) as sqlx::query::Query<MySql, MySqlArguments>;
                let count_query_str = &sql_query.count_query.clone().unwrap();
                let mut count_query = sqlx::query(&count_query_str);

                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
                            for string in values {
                                query = query.bind(string);
                                count_query = count_query.bind(string);
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                query = query.bind(int);
                                count_query = count_query.bind(int);
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool);
                                count_query = count_query.bind(bool);
                            }
                        }
                        SqlValueEnum::UUID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid);
                                count_query = count_query.bind(uuid);
                            }
                        }
                        SqlValueEnum::DateTime(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::DateTimeList(values) => {
                            for datetime in values {
                                query = query.bind(datetime);
                                count_query = count_query.bind(datetime);
                            }
                        }
                    }
                }

                let rows = query.fetch_all(pool).await.map_err(|e| {
                    error!("Error executing query: {:?}", e);
                    async_graphql::Error::new("Error executing query.")
                })?;

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(Some(ResponseRow::MySql(row)));
                }

                let count = count_query.fetch_one(pool).await.map_err(|e| {
                    error!("Error executing query: {:?} \n Error: {:?}", sql_query, e);
                    async_graphql::Error::new("Error executing query.")
                })?;

                let total_count = count.try_get("total_count").unwrap_or(0);
                debug!("Total Count: {:?}", total_count);

                Ok((response_rows, TotalCount(total_count)))
            }
            PoolEnum::Postgres(pool) => {
                let mut query = sqlx::query(&sql_query.query);
                let count_query_str = &sql_query.count_query.clone().unwrap();
                let mut count_query = sqlx::query(&count_query_str);

                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
                            for string in values {
                                query = query.bind(string);
                                count_query = count_query.bind(string);
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                query = query.bind(int);
                                count_query = count_query.bind(int);
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool);
                                count_query = count_query.bind(bool);
                            }
                        }
                        SqlValueEnum::UUID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid);
                                count_query = count_query.bind(uuid);
                            }
                        }
                        SqlValueEnum::DateTime(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::DateTimeList(values) => {
                            for datetime in values {
                                query = query.bind(datetime);
                                count_query = count_query.bind(datetime);
                            }
                        }
                    }
                }

                let rows = query.fetch_all(pool).await.map_err(|e| {
                    error!("Error executing query: {:?} \n Error: {:?}", sql_query, e);
                    async_graphql::Error::new("Error executing query.")
                })?;

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(Some(ResponseRow::Postgres(row)));
                }

                let count = count_query.fetch_one(pool).await.map_err(|e| {
                    error!("Error executing query: {:?} \n Error: {:?}", sql_query, e);
                    async_graphql::Error::new("Error executing query.")
                })?;

                let total_count = count.try_get("total_count").unwrap_or(0);
                debug!("Total Count: {:?}", total_count);

                Ok((response_rows, TotalCount(total_count)))
            }
            PoolEnum::SqLite(pool) => {
                let mut query = sqlx::query(&sql_query.query);
                let count_query_str = &sql_query.count_query.clone().unwrap();
                let mut count_query = sqlx::query(&count_query_str);

                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
                            for string in values {
                                query = query.bind(string);
                                count_query = count_query.bind(string)
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                query = query.bind(int);
                                count_query = count_query.bind(int)
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool);
                                count_query = count_query.bind(bool)
                            }
                        }
                        SqlValueEnum::UUID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid);
                                count_query = count_query.bind(uuid)
                            }
                        }
                        SqlValueEnum::DateTime(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValueEnum::DateTimeList(values) => {
                            for datetime in values {
                                count_query = count_query.bind(datetime);
                                query = query.bind(datetime)
                            }
                        }
                    }
                }

                let rows = query.fetch_all(pool).await.map_err(|e| {
                    error!("Error executing query: {:?} \n Error: {:?}", sql_query, e);
                    async_graphql::Error::new("Error executing query.")
                })?;

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(Some(ResponseRow::SqLite(row)));
                }

                let count = count_query.fetch_one(pool).await.map_err(|e| {
                    error!("Error executing query: {:?} \n Error: {:?}", sql_query, e);
                    async_graphql::Error::new("Error executing query.")
                })?;

                let total_count = count.try_get("total_count").unwrap_or(0);
                debug!("Total Count: {:?}", total_count);

                Ok((response_rows, TotalCount(total_count)))
            }
        }
    }
}
