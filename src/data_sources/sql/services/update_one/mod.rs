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

                let mut find_one_where_keys = Vec::new();
                let mut find_one_where_values = Vec::new();

                for key in &sql_query.where_keys {
                    find_one_where_keys.push(key.clone());
                    let index = sql_query
                        .value_keys
                        .iter()
                        .position(|x| *x.to_string() == key.to_string());

                    if index.is_none() {
                        let index = sql_query
                            .where_keys
                            .iter()
                            .position(|x| *x.to_string() == key.to_string())
                            .unwrap();
                        let value = sql_query.where_values.get(index).unwrap();
                        find_one_where_values.push(value.clone());
                    } else {
                        if let Some(value) = sql_query.values.get(index.unwrap()) {
                            find_one_where_values.push(value.clone());
                        }
                    }
                }

                for key in &sql_query.value_keys {
                    if !find_one_where_keys.contains(key) {
                        find_one_where_keys.push(key.clone());
                        let index = sql_query
                            .value_keys
                            .iter()
                            .position(|x| *x.to_string() == key.to_string())
                            .unwrap();
                        if let Some(value) = sql_query.values.get(index) {
                            find_one_where_values.push(value.clone());
                        }
                    }
                }

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
