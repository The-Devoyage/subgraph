use bson::doc;
use log::debug;

use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::{sql::PoolEnum, DataSource},
    graphql::schema::{create_entities::create_auth_service::ServiceUser, ServiceSchemaBuilder},
};

impl ServiceSchemaBuilder {
    pub async fn delete_user(
        data_source: &DataSource,
        identifier: &str,
    ) -> Result<(), async_graphql::Error> {
        debug!("Deleting user: {:?}", identifier);
        match &data_source {
            DataSource::Mongo(mongo_ds) => {
                let filter = doc! {
                    "identifier": &identifier
                };

                mongo_ds
                    .db
                    .collection::<ServiceUser>("subgraph_user")
                    .delete_one(filter, None)
                    .await
                    .map(|_| ())
                    .map_err(|e| {
                        async_graphql::Error::new(format!(
                            "Failed to delete user from mongo: {:?}",
                            e
                        ))
                    })
            }
            DataSource::SQL(sql_ds) => match sql_ds.config.dialect {
                DialectEnum::MYSQL => {
                    let query = sqlx::query("DELETE FROM subgraph_user WHERE identifier = ?;")
                        .bind(&identifier);
                    match sql_ds.pool.clone() {
                        PoolEnum::MySql(pool) => query.execute(&pool).await,
                        _ => unreachable!(),
                    }
                    .map(|_| ())
                    .map_err(|e| {
                        async_graphql::Error::new(format!(
                            "Failed to delete user from mysql: {:?}",
                            e
                        ))
                    })
                }
                DialectEnum::SQLITE => {
                    let query = sqlx::query("DELETE FROM subgraph_user WHERE identifier = ?;")
                        .bind(&identifier);

                    match sql_ds.pool.clone() {
                        PoolEnum::SqLite(pool) => query.execute(&pool).await,
                        _ => panic!("Pool not supported."),
                    }
                    .map(|_| ())
                    .map_err(|e| {
                        async_graphql::Error::new(format!(
                            "Failed to delete user from sqlite: {:?}",
                            e
                        ))
                    })
                }
                DialectEnum::POSTGRES => {
                    let query = sqlx::query("DELETE FROM subgraph_user WHERE identifier = $1;")
                        .bind(&identifier);
                    match sql_ds.pool.clone() {
                        PoolEnum::Postgres(pool) => query.execute(&pool).await,
                        _ => unreachable!(),
                    }
                    .map(|_| ())
                    .map_err(|e| {
                        async_graphql::Error::new(format!(
                            "Failed to delete user from postgres: {:?}",
                            e
                        ))
                    })
                }
            },
            _ => panic!("Data Source not supported."),
        }
    }
}
