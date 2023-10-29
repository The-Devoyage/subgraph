use bson::Document;
use log::debug;

use crate::{
    configuration::subgraph::{data_sources::sql::DialectEnum, entities::ServiceEntityConfig},
    data_sources::sql::{PoolEnum, SqlDataSource, SqlQuery, SqlValueEnum},
};

use super::{ResponseRow, Services};

impl Services {
    pub async fn update_one(
        entity: &ServiceEntityConfig,
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
        dialect: DialectEnum,
    ) -> Result<Option<ResponseRow>, async_graphql::Error> {
        debug!("Executing Update One Query: {:?}", sql_query);

        match pool_enum {
            PoolEnum::MySql(pool) => {
                let mut update_query = sqlx::query(&sql_query.query);

                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for string in values {
                                update_query = update_query.bind(string)
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
                        SqlValueEnum::UUID(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::UUIDList(values) => {
                            for uuid in values {
                                update_query = update_query.bind(uuid)
                            }
                        }
                    }
                }

                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for string in values {
                                update_query = update_query.bind(string)
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
                        SqlValueEnum::UUID(v) => {
                            update_query = update_query.bind(v);
                        }
                        SqlValueEnum::UUIDList(values) => {
                            for uuid in values {
                                update_query = update_query.bind(uuid)
                            }
                        }
                    }
                }

                update_query.execute(pool).await?;

                let (find_one_where_keys, find_one_where_values) =
                    SqlDataSource::create_update_return_key_data(
                        &sql_query.where_keys,
                        &sql_query.where_values,
                        &sql_query.value_keys,
                        &sql_query.values,
                    )?;

                let mut input_document = Document::new();

                //for each key in find_one_where_keys, add the key and value to the input_document
                //TODO: Make sure this is a inputdocument.query
                for (index, key) in find_one_where_keys.iter().enumerate() {
                    match &find_one_where_values[index] {
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
                        _ => return Err(async_graphql::Error::from("Invalid Value Type")),
                    }
                }

                let (find_one_query_string, ..) = SqlDataSource::create_find_one_query(
                    entity,
                    &sql_query.table,
                    &dialect,
                    &input_document,
                )?;

                let mut find_one_query = sqlx::query(&find_one_query_string);

                for value in &find_one_where_values {
                    match value {
                        SqlValueEnum::String(v) => {
                            find_one_query = find_one_query.bind(v);
                        }
                        SqlValueEnum::Int(v) => {
                            find_one_query = find_one_query.bind(v);
                        }
                        SqlValueEnum::Bool(v) => {
                            find_one_query = find_one_query.bind(v);
                        }
                        SqlValueEnum::StringList(values) => {
                            for string in values {
                                find_one_query = find_one_query.bind(string)
                            }
                        }
                        SqlValueEnum::IntList(values) => {
                            for int in values {
                                find_one_query = find_one_query.bind(int)
                            }
                        }
                        SqlValueEnum::BoolList(values) => {
                            for bool in values {
                                find_one_query = find_one_query.bind(bool)
                            }
                        }
                        SqlValueEnum::UUID(v) => {
                            find_one_query = find_one_query.bind(v);
                        }
                        SqlValueEnum::UUIDList(values) => {
                            for uuid in values {
                                find_one_query = find_one_query.bind(uuid)
                            }
                        }
                    }
                }

                let find_one_result = find_one_query.fetch_one(pool).await?;

                Ok(Some(ResponseRow::MySql(find_one_result)))
            }
            _ => Err(async_graphql::Error::from("Update One Not Supported")),
        }
    }
}
