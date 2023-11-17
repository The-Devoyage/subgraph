use log::debug;

use crate::data_sources::sql::{PoolEnum, SqlQuery, SqlValueEnum};

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

                let row = query.fetch_optional(pool).await?;

                if row.is_none() {
                    return Ok(None);
                }

                Ok(Some(ResponseRow::Postgres(row.unwrap())))
            }
            PoolEnum::SqLite(pool) => {
                debug!("Executing SQLITE Query");
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

                let row = query.fetch_optional(pool).await?;

                if row.is_none() {
                    return Ok(None);
                }

                Ok(Some(ResponseRow::SqLite(row.unwrap())))
            }
        }
    }
}
