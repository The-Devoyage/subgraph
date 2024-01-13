use log::{debug, error, trace};
use sqlx::Row;

use crate::{
    configuration::subgraph::entities::ServiceEntityConfig,
    data_sources::sql::{PoolEnum, SqlQuery, SqlValueEnum},
    utils::clean_string::{clean_string, CleanOptions},
};

use super::{ResponseRow, Services};

impl Services {
    pub async fn update_many(
        entity: &ServiceEntityConfig,
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
    ) -> Result<Vec<Option<ResponseRow>>, async_graphql::Error> {
        debug!("Update Many SQL Data Source");

        let clean_options = CleanOptions {
            newline: Some(false),
            quotes: Some(true),
        };
        match pool_enum {
            PoolEnum::MySql(pool) => {
                let identifier_query = match &sql_query.identifier_query {
                    Some(query) => query,
                    None => {
                        error!(
                            "Identifier query not found for update_many on {}",
                            entity.name
                        );
                        return Err(async_graphql::Error::new(format!(
                            "Identifier query not found for update_many on {}",
                            entity.name
                        )));
                    }
                };

                let mut identifier_query = sqlx::query(&identifier_query);

                let mut update_query = sqlx::query(&sql_query.query);
                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            let v = clean_string(v, Some(clean_options.clone()));
                            update_query = update_query.bind(v.clone());
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
                            for v in values {
                                let v = clean_string(v, Some(clean_options.clone()));
                                update_query = update_query.bind(v.clone());
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                update_query = update_query.bind(int);
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                update_query = update_query.bind(bool);
                            }
                        }
                        SqlValueEnum::UUID(uuid) => update_query = update_query.bind(uuid),
                        SqlValueEnum::UUIDList(uuids) => {
                            for uuid in uuids {
                                update_query = update_query.bind(uuid);
                            }
                        }
                        SqlValueEnum::DateTime(date_time) => {
                            update_query = update_query.bind(date_time);
                        }
                        SqlValueEnum::DateTimeList(date_times) => {
                            for date_time in date_times {
                                update_query = update_query.bind(date_time);
                            }
                        }
                    }
                }
                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            let v = clean_string(v, Some(clean_options.clone()));
                            update_query = update_query.bind(v.clone());
                            identifier_query = identifier_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                            identifier_query = identifier_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                            identifier_query = identifier_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
                            for v in values {
                                let v = clean_string(v, Some(clean_options.clone()));
                                update_query = update_query.bind(v.clone());
                                identifier_query = identifier_query.bind(v);
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                update_query = update_query.bind(int);
                                identifier_query = identifier_query.bind(int);
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                update_query = update_query.bind(bool);
                                identifier_query = identifier_query.bind(bool);
                            }
                        }
                        SqlValueEnum::UUID(uuid) => update_query = update_query.bind(uuid),
                        SqlValueEnum::UUIDList(uuids) => {
                            for uuid in uuids {
                                update_query = update_query.bind(uuid);
                                identifier_query = identifier_query.bind(uuid);
                            }
                        }
                        SqlValueEnum::DateTime(date_time) => {
                            update_query = update_query.bind(date_time);
                            identifier_query = identifier_query.bind(date_time);
                        }
                        SqlValueEnum::DateTimeList(date_times) => {
                            for date_time in date_times {
                                update_query = update_query.bind(date_time);
                                identifier_query = identifier_query.bind(date_time);
                            }
                        }
                    }
                }

                let identifier_results = identifier_query.fetch_all(pool).await.map_err(|e| {
                    error!("Error executing identifier query: {}", e);
                    e
                })?;

                update_query.execute(pool).await.map_err(|e| {
                    error!("Error executing update many query: {}", e);
                    e
                })?;

                let mut ids = Vec::new();
                for row in identifier_results {
                    trace!("Row: {:?}", row);
                    let identifier: i64 = row.try_get("id").map_err(|e| {
                        error!("Error getting primary key from row: {}", e);
                        e
                    })?;
                    ids.push(identifier);
                }

                trace!("Identifiers: {:?}", ids);

                if ids.is_empty() {
                    return Ok(Vec::new());
                }

                let query = format!(
                    "SELECT * FROM {} WHERE id IN ({});",
                    sql_query.table,
                    ids.iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                );
                trace!("Query: {}", query);

                let response_query = sqlx::query(&query);

                let rows = response_query.fetch_all(pool).await.map_err(|e| {
                    error!("Error finding data: {}", e);
                    e
                })?;

                let mut response_rows = Vec::new();

                for row in rows {
                    response_rows.push(Some(ResponseRow::MySql(row)));
                }

                Ok(response_rows)
            }
            PoolEnum::Postgres(pool) => {
                let mut update_query = sqlx::query(&sql_query.query);
                debug!("PG VALUES: {:?}", sql_query);
                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            let v = clean_string(v, Some(clean_options.clone()));
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
                            for v in values {
                                let v = clean_string(v, Some(clean_options.clone()));
                                update_query = update_query.bind(v)
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                update_query = update_query.bind(int)
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                update_query = update_query.bind(bool)
                            }
                        }
                        SqlValueEnum::UUID(uuid) => update_query = update_query.bind(uuid),
                        SqlValueEnum::UUIDList(uuids) => {
                            for uuid in uuids {
                                update_query = update_query.bind(uuid)
                            }
                        }
                        SqlValueEnum::DateTime(date_time) => {
                            update_query = update_query.bind(date_time)
                        }
                        SqlValueEnum::DateTimeList(date_times) => {
                            for date_time in date_times {
                                update_query = update_query.bind(date_time)
                            }
                        }
                    }
                }
                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            let v = clean_string(v, Some(clean_options.clone()));
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
                            for v in values {
                                let v = clean_string(v, Some(clean_options.clone()));
                                update_query = update_query.bind(v)
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                update_query = update_query.bind(int)
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                update_query = update_query.bind(bool)
                            }
                        }
                        SqlValueEnum::UUID(uuid) => update_query = update_query.bind(uuid),
                        SqlValueEnum::UUIDList(uuids) => {
                            for uuid in uuids {
                                update_query = update_query.bind(uuid)
                            }
                        }
                        SqlValueEnum::DateTime(date_time) => {
                            update_query = update_query.bind(date_time)
                        }
                        SqlValueEnum::DateTimeList(date_times) => {
                            for date_time in date_times {
                                update_query = update_query.bind(date_time)
                            }
                        }
                    }
                }

                let rows = update_query.fetch_all(pool).await.map_err(|e| {
                    error!("Error: {:?}", e);
                    e
                })?;

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(Some(ResponseRow::Postgres(row)));
                }
                Ok(response_rows)
            }
            PoolEnum::SqLite(pool) => {
                let identifier_query = match &sql_query.identifier_query {
                    Some(identifier_query) => identifier_query,
                    None => {
                        return Err(async_graphql::Error::new(
                            "No identifier query provided for SQLite",
                        ))
                    }
                };
                let mut identifier_query = sqlx::query(identifier_query);
                let mut update_query = sqlx::query(&sql_query.query);

                //Bind the values first, example: SET title = ?
                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            let v = clean_string(v, Some(clean_options.clone()));
                            update_query = update_query.bind(v.clone());
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v.clone());
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
                            for v in values {
                                let v = clean_string(v, Some(clean_options.clone()));
                                update_query = update_query.bind(v.clone());
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                update_query = update_query.bind(int);
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                update_query = update_query.bind(bool);
                            }
                        }
                        SqlValueEnum::UUID(uuid) => update_query = update_query.bind(uuid),
                        SqlValueEnum::UUIDList(uuids) => {
                            for uuid in uuids {
                                update_query = update_query.bind(uuid);
                            }
                        }
                        SqlValueEnum::DateTime(date_time) => {
                            update_query = update_query.bind(date_time);
                        }
                        SqlValueEnum::DateTimeList(date_times) => {
                            for date_time in date_times {
                                update_query = update_query.bind(date_time);
                            }
                        }
                    }
                }

                // Bind the where values, example: WHERE id = ?
                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(v) | SqlValueEnum::ObjectID(v) => {
                            let v = clean_string(v, Some(clean_options.clone()));
                            update_query = update_query.bind(v.clone());
                            identifier_query = identifier_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                            identifier_query = identifier_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                            identifier_query = identifier_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) | SqlValueEnum::ObjectIDList(values) => {
                            for v in values {
                                update_query = update_query.bind(v);
                                identifier_query = identifier_query.bind(v);
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                update_query = update_query.bind(int);
                                identifier_query = identifier_query.bind(int);
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                update_query = update_query.bind(bool);
                                identifier_query = identifier_query.bind(bool);
                            }
                        }
                        SqlValueEnum::UUID(uuid) => update_query = update_query.bind(uuid),
                        SqlValueEnum::UUIDList(uuids) => {
                            for uuid in uuids {
                                update_query = update_query.bind(uuid);
                                identifier_query = identifier_query.bind(uuid);
                            }
                        }
                        SqlValueEnum::DateTime(date_time) => {
                            update_query = update_query.bind(date_time);
                            identifier_query = identifier_query.bind(date_time);
                        }
                        SqlValueEnum::DateTimeList(date_times) => {
                            for date_time in date_times {
                                update_query = update_query.bind(date_time);
                                identifier_query = identifier_query.bind(date_time);
                            }
                        }
                    }
                }

                // Construct a query to get the updated data
                let identifer_results = identifier_query.fetch_all(pool).await.map_err(|e| {
                    error!("Error: {:?}", e);
                    e
                })?;

                update_query.execute(pool).await.map_err(|e| {
                    error!("Error: {:?}", e);
                    e
                })?;

                let mut ids = vec![];
                for row in identifer_results {
                    let id: i64 = row.try_get("id")?;
                    ids.push(id);
                }

                if ids.is_empty() {
                    return Ok(vec![]);
                }

                let query = format!(
                    "SELECT * FROM {} WHERE id IN ({})",
                    sql_query.table,
                    ids.iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                );

                let response_query = sqlx::query(&query);

                let rows = response_query.fetch_all(pool).await.map_err(|e| {
                    error!("Error finding data: {}", e);
                    e
                })?;

                let mut response_rows = Vec::new();

                for row in rows {
                    response_rows.push(Some(ResponseRow::SqLite(row)));
                }

                Ok(response_rows)
            }
        }
    }
}
