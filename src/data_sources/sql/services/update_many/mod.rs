use bson::{doc, Document};
use log::{debug, error};

use crate::{
    configuration::subgraph::{data_sources::sql::DialectEnum, entities::ServiceEntityConfig},
    data_sources::sql::{PoolEnum, SqlDataSource, SqlQuery, SqlValueEnum},
    utils::clean_string::clean_string,
};

use super::{ResponseRow, Services};

impl Services {
    pub async fn update_many(
        entity: &ServiceEntityConfig,
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
        dialect: DialectEnum,
    ) -> Result<Vec<Option<ResponseRow>>, async_graphql::Error> {
        debug!("Update Many SQL Data Source");

        match pool_enum {
            PoolEnum::MySql(pool) => {
                let mut update_query = sqlx::query(&sql_query.query);
                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(v) => {
                            let v = clean_string(v);
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for v in values {
                                let v = clean_string(v);
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
                        SqlValueEnum::String(v) => {
                            let v = clean_string(v);
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for v in values {
                                let v = clean_string(v);
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

                update_query.execute(pool).await.map_err(|e| {
                    error!("Error executing update many query: {}", e);
                    e
                })?;

                let (find_many_where_keys, find_many_where_values) =
                    SqlDataSource::create_update_return_key_data(
                        &sql_query.where_keys,
                        &sql_query.where_values,
                        &sql_query.value_keys,
                        &sql_query.values,
                    )?;
                let mut input_document = Document::new();

                for (index, key) in find_many_where_keys.iter().enumerate() {
                    match &find_many_where_values[index] {
                        SqlValueEnum::String(v) => input_document.insert(key, v),
                        SqlValueEnum::Int(v) => input_document.insert(key, v),
                        SqlValueEnum::Bool(v) => input_document.insert(key, v),
                        SqlValueEnum::UUID(v) => input_document.insert(key, v.to_string()),
                        SqlValueEnum::DateTime(v) => input_document.insert(key, v),
                        _ => return Err(async_graphql::Error::from("Invalid Value Type")),
                    };
                }

                let query_input = doc! {
                    "query": input_document
                };

                let (find_many_query_string, ..) = SqlDataSource::create_find_many_query(
                    entity,
                    &sql_query.table,
                    &dialect,
                    &query_input,
                )?;

                let mut find_many_query = sqlx::query(&find_many_query_string);

                for value in &find_many_where_values {
                    match value {
                        SqlValueEnum::String(v) => {
                            let v = clean_string(v);
                            find_many_query = find_many_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            find_many_query = find_many_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            find_many_query = find_many_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for v in values {
                                let v = clean_string(v);
                                find_many_query = find_many_query.bind(v)
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                find_many_query = find_many_query.bind(int)
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                find_many_query = find_many_query.bind(bool)
                            }
                        }
                        SqlValueEnum::UUID(uuid) => find_many_query = find_many_query.bind(uuid),
                        SqlValueEnum::UUIDList(uuids) => {
                            for uuid in uuids {
                                find_many_query = find_many_query.bind(uuid)
                            }
                        }
                        SqlValueEnum::DateTime(date_time) => {
                            find_many_query = find_many_query.bind(date_time)
                        }
                        SqlValueEnum::DateTimeList(date_times) => {
                            for date_time in date_times {
                                find_many_query = find_many_query.bind(date_time)
                            }
                        }
                    }
                }

                let rows = find_many_query.fetch_all(pool).await.map_err(|e| {
                    error!("Error executing find many query: {}", e);
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
                        SqlValueEnum::String(v) => {
                            let v = clean_string(v);
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for v in values {
                                let v = clean_string(v);
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
                        SqlValueEnum::String(v) => {
                            let v = clean_string(v);
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for v in values {
                                let v = clean_string(v);
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
                let mut update_query = sqlx::query(&sql_query.query);

                //Bind the values first, example: SET title = ?
                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(v) => {
                            let v = clean_string(v);
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for v in values {
                                let v = clean_string(v);
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

                // Bind the where values, example: WHERE id = ?
                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(v) => {
                            let v = clean_string(v);
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for v in values {
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

                update_query
                    .execute(pool)
                    .await
                    .map_err(|e| {
                        error!("Error updating data: {}", e);
                        e
                    })
                    .map_err(|e| {
                        error!("Error: {:?}", e);
                        e
                    })?;

                let (find_many_where_keys, find_many_where_values) =
                    SqlDataSource::create_update_return_key_data(
                        &sql_query.where_keys,
                        &sql_query.where_values,
                        &sql_query.value_keys,
                        &sql_query.values,
                    )?;

                // Create the input document to find the newly updated data
                let mut input_document = Document::new();

                for (index, key) in find_many_where_keys.iter().enumerate() {
                    match &find_many_where_values[index] {
                        SqlValueEnum::String(v) => {
                            input_document.insert(key, v);
                        }
                        SqlValueEnum::Int(v) => {
                            input_document.insert(key, v);
                        }
                        SqlValueEnum::Bool(v) => {
                            input_document.insert(key, v);
                        }
                        SqlValueEnum::UUID(v) => {
                            input_document.insert(key, v.to_string());
                        }
                        SqlValueEnum::DateTime(v) => {
                            input_document.insert(key, v);
                        }
                        _ => return Err(async_graphql::Error::new("Invalid value type")),
                    }
                }

                let query_doc = doc! {
                    "query": input_document
                };

                debug!("Query document: {:?}", query_doc);

                let (find_many_query_string, ..) = SqlDataSource::create_find_many_query(
                    entity,
                    &sql_query.table,
                    &dialect,
                    &query_doc,
                )?;

                let mut find_many_query = sqlx::query(&find_many_query_string);

                for value in &find_many_where_values {
                    match value {
                        SqlValueEnum::String(v) => {
                            let v = clean_string(v);
                            find_many_query = find_many_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            find_many_query = find_many_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            find_many_query = find_many_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for v in values {
                                find_many_query = find_many_query.bind(v)
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                find_many_query = find_many_query.bind(int)
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                find_many_query = find_many_query.bind(bool)
                            }
                        }
                        SqlValueEnum::UUID(uuid) => find_many_query = find_many_query.bind(uuid),
                        SqlValueEnum::UUIDList(uuids) => {
                            for uuid in uuids {
                                find_many_query = find_many_query.bind(uuid)
                            }
                        }
                        SqlValueEnum::DateTime(date_time) => {
                            find_many_query = find_many_query.bind(date_time)
                        }
                        SqlValueEnum::DateTimeList(date_times) => {
                            for date_time in date_times {
                                find_many_query = find_many_query.bind(date_time)
                            }
                        }
                    }
                }

                let rows = find_many_query.fetch_all(pool).await.map_err(|e| {
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
