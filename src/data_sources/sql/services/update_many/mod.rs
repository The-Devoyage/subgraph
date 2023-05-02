use log::debug;

use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::sql::{PoolEnum, SqlDataSource, SqlQuery, SqlValueEnum},
};

use super::{ResponseRow, Services};

impl Services {
    pub async fn update_many(
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
        dialect: DialectEnum,
    ) -> Result<Vec<ResponseRow>, async_graphql::Error> {
        debug!("Update Many SQL Data Source");

        match pool_enum {
            PoolEnum::MySql(pool) => {
                let mut update_query = sqlx::query(&sql_query.query);
                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            update_query = update_query.bind(value);
                        }
                    }
                }
                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            update_query = update_query.bind(value);
                        }
                    }
                }

                update_query.execute(pool).await?;

                let (find_many_where_keys, find_many_where_values) =
                    SqlDataSource::create_update_return_key_data(
                        &sql_query.where_keys,
                        &sql_query.where_values,
                        &sql_query.value_keys,
                        &sql_query.values,
                    );
                let find_many_query_string = SqlDataSource::create_find_many_query(
                    &sql_query.table,
                    &find_many_where_keys,
                    &dialect,
                );
                let mut find_many_query = sqlx::query(&find_many_query_string);

                for value in &find_many_where_values {
                    match value {
                        SqlValueEnum::String(value) => {
                            find_many_query = find_many_query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            find_many_query = find_many_query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            find_many_query = find_many_query.bind(value);
                        }
                    }
                }

                let rows = find_many_query.fetch_all(pool).await?;

                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(ResponseRow::MySql(row));
                }
                Ok(response_rows)
            }
            PoolEnum::Postgres(pool) => {
                let mut update_query = sqlx::query(&sql_query.query);
                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            update_query = update_query.bind(value);
                        }
                    }
                }
                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            update_query = update_query.bind(value);
                        }
                    }
                }

                let rows = update_query.fetch_all(pool).await?;
                let mut response_rows = Vec::new();
                for row in rows {
                    response_rows.push(ResponseRow::Postgres(row));
                }
                Ok(response_rows)
            }
            PoolEnum::SqLite(pool) => {
                let mut update_query = sqlx::query(&sql_query.query);
                for value in &sql_query.values {
                    match value {
                        SqlValueEnum::String(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            update_query = update_query.bind(value);
                        }
                    }
                }
                for value in &sql_query.where_values {
                    match value {
                        SqlValueEnum::String(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            update_query = update_query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            update_query = update_query.bind(value);
                        }
                    }
                }

                update_query.execute(pool).await?;

                let (find_many_where_keys, find_many_where_values) =
                    SqlDataSource::create_update_return_key_data(
                        &sql_query.where_keys,
                        &sql_query.where_values,
                        &sql_query.value_keys,
                        &sql_query.values,
                    );

                let find_many_query_string = SqlDataSource::create_find_many_query(
                    &sql_query.table,
                    &find_many_where_keys,
                    &dialect,
                );

                let mut find_many_query = sqlx::query(&find_many_query_string);

                for value in &find_many_where_values {
                    match value {
                        SqlValueEnum::String(value) => {
                            find_many_query = find_many_query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            find_many_query = find_many_query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            find_many_query = find_many_query.bind(value);
                        }
                    }
                }

                let rows = find_many_query.fetch_all(pool).await?;

                let mut response_rows = Vec::new();

                for row in rows {
                    response_rows.push(ResponseRow::SqLite(row));
                }

                Ok(response_rows)
            }
        }
    }
}
