use log::debug;

use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::sql::{PoolEnum, SqlDataSource, SqlQuery, SqlValueEnum},
};

use super::{ResponseRow, Services};

impl Services {
    pub async fn update_one(
        pool_enum: &PoolEnum,
        sql_query: &SqlQuery,
        dialect: DialectEnum,
    ) -> Result<ResponseRow, async_graphql::Error> {
        debug!("Executing Update One Query: {:?}", sql_query);

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

                let (find_one_where_keys, find_one_where_values) =
                    SqlDataSource::create_update_return_key_data(
                        &sql_query.where_keys,
                        &sql_query.where_values,
                        &sql_query.value_keys,
                        &sql_query.values,
                    );

                let find_one_query_string = SqlDataSource::create_find_one_query(
                    &sql_query.table,
                    &find_one_where_keys,
                    &dialect,
                );

                let mut find_one_query = sqlx::query(&find_one_query_string);

                for value in &find_one_where_values {
                    match value {
                        SqlValueEnum::String(value) => {
                            find_one_query = find_one_query.bind(value);
                        }
                        SqlValueEnum::Int(value) => {
                            find_one_query = find_one_query.bind(value);
                        }
                        SqlValueEnum::Bool(value) => {
                            find_one_query = find_one_query.bind(value);
                        }
                    }
                }

                let find_one_result = find_one_query.fetch_one(pool).await?;

                Ok(ResponseRow::MySql(find_one_result))
            }
            _ => Err(async_graphql::Error::from("Update One Not Supported")),
        }
    }
}
