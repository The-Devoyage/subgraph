use log::{debug, error, trace, warn};

use crate::{
    data_sources::sql::{PoolEnum, SqlQuery},
    sql_value::SqlValue,
};

use super::{ResponseRow, Services};

impl Services {
    pub async fn find_one(
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
    ) -> Result<Option<ResponseRow>, async_graphql::Error> {
        debug!("Executing Find One Query: {:?}", sql_query);
        match pool_enum {
            PoolEnum::MySql(pool) => {
                debug!("Executing MYSQL Query");
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.where_values {
                    match value {
                        SqlValue::String(v) | SqlValue::ObjectID(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::Int(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::Bool(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::StringList(values) | SqlValue::ObjectIDList(values) => {
                            for string in values {
                                query = query.bind(string)
                            }
                        }
                        SqlValue::IntList(values) => {
                            for int in values {
                                query = query.bind(int)
                            }
                        }
                        SqlValue::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool)
                            }
                        }
                        SqlValue::UUID(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid)
                            }
                        }
                        SqlValue::DateTime(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::DateTimeList(values) => {
                            for datetime in values {
                                query = query.bind(datetime)
                            }
                        }
                    }
                }

                let row = query.fetch_optional(pool).await?;

                debug!("DB Row: {:?}", row);

                if row.is_none() {
                    return Ok(None);
                }

                Ok(Some(ResponseRow::MySql(row.unwrap())))
            }
            PoolEnum::Postgres(pool) => {
                debug!("Executing POSTGRES Query");
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.where_values {
                    match value {
                        SqlValue::String(v) | SqlValue::ObjectID(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::Int(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::Bool(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::StringList(values) | SqlValue::ObjectIDList(values) => {
                            for string in values {
                                query = query.bind(string)
                            }
                        }
                        SqlValue::IntList(values) => {
                            for int in values {
                                query = query.bind(int)
                            }
                        }
                        SqlValue::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool)
                            }
                        }
                        SqlValue::UUID(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid)
                            }
                        }
                        SqlValue::DateTime(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::DateTimeList(values) => {
                            for datetime in values {
                                query = query.bind(datetime)
                            }
                        }
                    }
                }

                let row = query.fetch_optional(pool).await?;

                if row.is_none() {
                    return Ok(None);
                }

                Ok(Some(ResponseRow::Postgres(row.unwrap())))
            }
            PoolEnum::SqLite(pool) => {
                debug!("Executing SQLITE Query: {:?}", sql_query.query);
                let mut query = sqlx::query(&sql_query.query);

                for value in &sql_query.where_values {
                    match value {
                        SqlValue::String(v) | SqlValue::ObjectID(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::Int(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::Bool(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::StringList(values) | SqlValue::ObjectIDList(values) => {
                            for string in values {
                                query = query.bind(string)
                            }
                        }
                        SqlValue::IntList(values) => {
                            for int in values {
                                query = query.bind(int)
                            }
                        }
                        SqlValue::BoolList(values) => {
                            for bool in values {
                                query = query.bind(bool)
                            }
                        }
                        SqlValue::UUID(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::UUIDList(values) => {
                            for uuid in values {
                                query = query.bind(uuid)
                            }
                        }
                        SqlValue::DateTime(v) => {
                            query = query.bind(v);
                        }
                        SqlValue::DateTimeList(values) => {
                            for datetime in values {
                                query = query.bind(datetime)
                            }
                        }
                    }
                }

                let row = query.fetch_optional(pool).await.map_err(|e| {
                    error!("Sqlite Find One Error: {:?}", e);
                    async_graphql::Error::new(format!("Error finding one"))
                })?;

                if row.is_none() {
                    warn!("No row found: {:?}", sql_query);
                    return Ok(None);
                }
                trace!("Row Found: {:?}", row.is_some());
                Ok(Some(ResponseRow::SqLite(row.unwrap())))
            }
        }
    }
}
