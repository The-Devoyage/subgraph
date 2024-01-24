use log::{debug, error};
use sqlx::Row;

use crate::{
    configuration::subgraph::entities::ServiceEntityConfig,
    data_sources::sql::{PoolEnum, SqlQuery, SqlValue},
};

use super::{ResponseRow, Services};

impl Services {
    pub async fn update_one(
        entity: &ServiceEntityConfig,
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
    ) -> Result<Option<ResponseRow>, async_graphql::Error> {
        debug!("Executing Update One Query: {:?}", sql_query);

        match pool_enum {
            PoolEnum::MySql(pool) => {
                let identifier_query = match &sql_query.identifier_query {
                    Some(query) => query,
                    None => {
                        error!("No identifier query found for entity: {}", entity.name);
                        return Err(async_graphql::Error::new(format!(
                            "No identifier query found for entity: {}",
                            entity.name
                        )));
                    }
                };
                let mut identifier_query = sqlx::query(&identifier_query);
                let mut update_query = sqlx::query(&sql_query.query);

                for value in &sql_query.values {
                    match value {
                        SqlValue::String(v) | SqlValue::ObjectID(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValue::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValue::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValue::StringList(values) | SqlValue::ObjectIDList(values) => {
                            for string in values {
                                update_query = update_query.bind(string)
                            }
                        }
                        SqlValue::IntList(values) => {
                            for int in values {
                                update_query = update_query.bind(int)
                            }
                        }
                        SqlValue::BoolList(values) => {
                            for bool in values {
                                update_query = update_query.bind(bool)
                            }
                        }
                        SqlValue::UUID(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValue::UUIDList(values) => {
                            for uuid in values {
                                update_query = update_query.bind(uuid)
                            }
                        }
                        SqlValue::DateTime(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValue::DateTimeList(values) => {
                            for datetime in values {
                                update_query = update_query.bind(datetime)
                            }
                        }
                    }
                }

                for value in &sql_query.where_values {
                    match value {
                        SqlValue::String(v) | SqlValue::ObjectID(v) => {
                            update_query = update_query.bind(v);
                            identifier_query = identifier_query.bind(v);
                        }
                        SqlValue::Int(v) => {
                            update_query = update_query.bind(v);
                            identifier_query = identifier_query.bind(v);
                        }
                        SqlValue::Bool(v) => {
                            update_query = update_query.bind(v);
                            identifier_query = identifier_query.bind(v);
                        }
                        SqlValue::StringList(values) | SqlValue::ObjectIDList(values) => {
                            for string in values {
                                update_query = update_query.bind(string);
                                identifier_query = identifier_query.bind(string);
                            }
                        }
                        SqlValue::IntList(values) => {
                            for int in values {
                                update_query = update_query.bind(int);
                                identifier_query = identifier_query.bind(int);
                            }
                        }
                        SqlValue::BoolList(values) => {
                            for bool in values {
                                update_query = update_query.bind(bool);
                                identifier_query = identifier_query.bind(bool);
                            }
                        }
                        SqlValue::UUID(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValue::UUIDList(values) => {
                            for uuid in values {
                                update_query = update_query.bind(uuid);
                                identifier_query = identifier_query.bind(uuid);
                            }
                        }
                        SqlValue::DateTime(v) => {
                            update_query = update_query.bind(v);
                            identifier_query = identifier_query.bind(v);
                        }
                        SqlValue::DateTimeList(values) => {
                            for datetime in values {
                                update_query = update_query.bind(datetime);
                                identifier_query = identifier_query.bind(datetime);
                            }
                        }
                    }
                }

                let identifier_results = identifier_query.fetch_all(pool).await?;

                if identifier_results.len() == 0 {
                    error!("No results found for entity: {}", entity.name);
                    return Err(async_graphql::Error::new(format!(
                        "No results found for entity: {}",
                        entity.name
                    )));
                }

                if identifier_results.len() > 1 {
                    error!("Multiple results found for entity: {}", entity.name);
                    return Err(async_graphql::Error::new(format!(
                        "Multiple results found for entity: {}",
                        entity.name
                    )));
                }

                let id: i64 = match identifier_results[0].get("id") {
                    Some(id) => id,
                    None => {
                        error!("No primary key found for entity: {}", entity.name);
                        return Err(async_graphql::Error::new(format!(
                            "No primary key found for entity: {}",
                            entity.name
                        )));
                    }
                };

                update_query.execute(pool).await?;

                let response_query = format!(
                    "SELECT * FROM {} WHERE id = {}",
                    sql_query.table,
                    id.to_string()
                );
                let response_query = sqlx::query(&response_query);

                let find_one_result = response_query.fetch_one(pool).await?;

                Ok(Some(ResponseRow::MySql(find_one_result)))
            }
            _ => Err(async_graphql::Error::from("Update One Not Supported")),
        }
    }
}
