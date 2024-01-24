use async_graphql::ErrorExtensions;
use log::{debug, error, trace};
use sqlx::{mysql::MySqlArguments, MySql, Row};

use crate::data_sources::{
    sql::{PoolEnum, SqlQuery, SqlValue},
    TotalCount,
};

use super::{ResponseRow, Services};

impl Services {
    pub async fn find_many(
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
        has_selection_set: &bool,
    ) -> Result<(Vec<Option<ResponseRow>>, TotalCount), async_graphql::Error> {
        debug!("Executing Find Many Query");
        trace!("{:?}", sql_query);

        match pool_enum {
            PoolEnum::MySql(pool) => {
                let mut query =
                    sqlx::query(&sql_query.query) as sqlx::query::Query<MySql, MySqlArguments>;
                let count_query_str = &sql_query.count_query.clone().unwrap();
                let mut count_query = sqlx::query(&count_query_str);

                for value in &sql_query.where_values {
                    match value {
                        SqlValue::String(v) | SqlValue::ObjectID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::Int(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::Bool(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::StringList(values) | SqlValue::ObjectIDList(values) => {
                            for string in values {
                                query = query.bind(string);
                                count_query = count_query.bind(string);
                            }
                        }
                        SqlValue::IntList(values) => {
                            for int in values {
                                query = query.bind(int);
                                count_query = count_query.bind(int);
                            }
                        }
                        SqlValue::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool);
                                count_query = count_query.bind(bool);
                            }
                        }
                        SqlValue::UUID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid);
                                count_query = count_query.bind(uuid);
                            }
                        }
                        SqlValue::DateTime(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::DateTimeList(values) => {
                            for datetime in values {
                                query = query.bind(datetime);
                                count_query = count_query.bind(datetime);
                            }
                        }
                    }
                }

                let rows = if *has_selection_set {
                    query.fetch_all(pool).await.map_err(|e| {
                        error!("Error executing query: {:?}", e);
                        async_graphql::Error::new("Error executing query.")
                            .extend_with(|_, err| err.set("cause", e.to_string()))
                    })?
                } else {
                    Vec::new()
                };

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(Some(ResponseRow::MySql(row)));
                }

                let count = count_query.fetch_one(pool).await.map_err(|e| {
                    error!("Error executing query: {:?} \n Error: {:?}", sql_query, e);
                    async_graphql::Error::new("Error executing query.")
                        .extend_with(|_, err| err.set("cause", e.to_string()))
                })?;

                let total_count = count.try_get("total_count").unwrap_or(0);
                trace!("Total Count: {:?}", total_count);

                Ok((response_rows, TotalCount(total_count)))
            }
            PoolEnum::Postgres(pool) => {
                let mut query = sqlx::query(&sql_query.query);
                let count_query_str = &sql_query.count_query.clone().unwrap();
                let mut count_query = sqlx::query(&count_query_str);

                for value in &sql_query.where_values {
                    match value {
                        SqlValue::String(v) | SqlValue::ObjectID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::Int(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::Bool(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::StringList(values) | SqlValue::ObjectIDList(values) => {
                            for string in values {
                                query = query.bind(string);
                                count_query = count_query.bind(string);
                            }
                        }
                        SqlValue::IntList(values) => {
                            for int in values {
                                query = query.bind(int);
                                count_query = count_query.bind(int);
                            }
                        }
                        SqlValue::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool);
                                count_query = count_query.bind(bool);
                            }
                        }
                        SqlValue::UUID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid);
                                count_query = count_query.bind(uuid);
                            }
                        }
                        SqlValue::DateTime(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::DateTimeList(values) => {
                            for datetime in values {
                                query = query.bind(datetime);
                                count_query = count_query.bind(datetime);
                            }
                        }
                    }
                }

                let rows = if *has_selection_set {
                    query.fetch_all(pool).await.map_err(|e| {
                        error!("Error executing query: {:?}", e);
                        async_graphql::Error::new("Error executing query.")
                            .extend_with(|_, err| err.set("cause", e.to_string()))
                    })?
                } else {
                    Vec::new()
                };

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(Some(ResponseRow::Postgres(row)));
                }

                let count = count_query.fetch_one(pool).await.map_err(|e| {
                    error!(
                        "Error executing count query: {:?} \n Error: {:?}",
                        sql_query, e
                    );
                    async_graphql::Error::new("Error executing query.")
                        .extend_with(|_, err| err.set("cause", e.to_string()))
                })?;

                let total_count = count.try_get("total_count").unwrap_or(0);
                trace!("Total Count: {:?}", total_count);

                Ok((response_rows, TotalCount(total_count)))
            }
            PoolEnum::SqLite(pool) => {
                let mut query = sqlx::query(&sql_query.query);
                let count_query_str = &sql_query.count_query.clone().unwrap();
                let mut count_query = sqlx::query(&count_query_str);

                for value in &sql_query.where_values {
                    match value {
                        SqlValue::String(v) | SqlValue::ObjectID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::Int(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::Bool(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::StringList(values) | SqlValue::ObjectIDList(values) => {
                            for string in values {
                                query = query.bind(string);
                                count_query = count_query.bind(string)
                            }
                        }
                        SqlValue::IntList(values) => {
                            for int in values {
                                query = query.bind(int);
                                count_query = count_query.bind(int)
                            }
                        }
                        SqlValue::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool);
                                count_query = count_query.bind(bool)
                            }
                        }
                        SqlValue::UUID(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid);
                                count_query = count_query.bind(uuid)
                            }
                        }
                        SqlValue::DateTime(v) => {
                            query = query.bind(v);
                            count_query = count_query.bind(v);
                        }
                        SqlValue::DateTimeList(values) => {
                            for datetime in values {
                                count_query = count_query.bind(datetime);
                                query = query.bind(datetime)
                            }
                        }
                    }
                }

                let rows = if *has_selection_set {
                    query.fetch_all(pool).await.map_err(|e| {
                        error!("Error executing query: {:?} \n Error: {:?}", sql_query, e);
                        async_graphql::Error::new("Error executing query.")
                            .extend_with(|_, err| err.set("cause", e.to_string()))
                    })?
                } else {
                    Vec::new()
                };

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(Some(ResponseRow::SqLite(row)));
                }

                let count = count_query.fetch_one(pool).await.map_err(|e| {
                    error!("Error executing query: {:?} \n Error: {:?}", sql_query, e);
                    async_graphql::Error::new("Error executing query.")
                        .extend_with(|_, err| err.set("cause", e.to_string()))
                })?;

                let total_count = count.try_get("total_count").unwrap_or(0);
                trace!("Total Count: {:?}", total_count);

                Ok((response_rows, TotalCount(total_count)))
            }
        }
    }
}
