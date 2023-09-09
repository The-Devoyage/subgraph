use bson::doc;
use log::debug;
use sqlx::Row;

use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::{sql::PoolEnum, DataSource},
    graphql::schema::{create_entities::create_auth_service::ServiceUser, ServiceSchemaBuilder},
};

impl ServiceSchemaBuilder {
    pub async fn get_user(
        data_source: &DataSource,
        identifier: &str,
    ) -> Result<Option<ServiceUser>, async_graphql::Error> {
        debug!("Getting user, identifier: {:?}", &identifier);

        let user = match &data_source {
            DataSource::Mongo(mongo_ds) => {
                debug!("Getting User - Mongo Data Source");
                let filter = doc! {
                    "identifier": &identifier
                };

                let user = mongo_ds
                    .db
                    .collection::<ServiceUser>("subgraph_user")
                    .find_one(filter, None)
                    .await;

                user.map_err(|e| {
                    async_graphql::Error::new(format!("Failed to get user from mongo: {:?}", e))
                })
            }
            DataSource::SQL(sql_ds) => {
                debug!("SQL data source");
                let user = match sql_ds.config.dialect {
                    DialectEnum::MYSQL => {
                        let query =
                            sqlx::query("SELECT * FROM subgraph_user WHERE identifier = ?;")
                                .bind(&identifier);

                        let user = match sql_ds.pool.clone() {
                            PoolEnum::MySql(pool) => query.fetch_one(&pool).await,
                            _ => unreachable!(),
                        };
                        let user = user.map(|mysql_row| {
                            let identifier =
                                mysql_row.try_get("identifier").unwrap_or("").to_string();
                            let registration_state =
                                mysql_row.try_get("registration_state").unwrap_or("");
                            let pub_key = mysql_row.try_get("pub_key").unwrap_or("");

                            let user = ServiceUser {
                                identifier,
                                registration_state: serde_json::from_str(&registration_state)
                                    .expect("Failed to deserialize registration state"),
                                pub_key: serde_json::from_str(&pub_key)
                                    .expect("Failed to deserialize pub key"),
                            };
                            Some(user)
                        });
                        user.map_err(|e| {
                            async_graphql::Error::new(format!(
                                "Failed to get user from mysql: {:?}",
                                e
                            ))
                        })
                    }
                    DialectEnum::SQLITE => {
                        let query =
                            sqlx::query("SELECT * FROM subgraph_user WHERE identifier = ?;")
                                .bind(&identifier);

                        let user = match sql_ds.pool.clone() {
                            PoolEnum::SqLite(pool) => query.fetch_one(&pool).await,
                            _ => panic!("Pool not supported."),
                        }
                        .map(|sqlite_row| {
                            let identifier =
                                sqlite_row.try_get("identifier").unwrap_or("").to_string();
                            let registration_state =
                                sqlite_row.try_get("registration_state").unwrap_or("");
                            let pub_key = sqlite_row.try_get("pub_key").unwrap_or("");
                            let user = ServiceUser {
                                identifier,
                                registration_state: serde_json::from_str(&registration_state)
                                    .expect("Failed to deserialize registration state"),
                                pub_key: serde_json::from_str(&pub_key).unwrap_or(None),
                            };
                            Some(user)
                        })
                        .map_err(|e| {
                            async_graphql::Error::new(format!(
                                "Failed to get user from sqlite: {:?}",
                                e
                            ))
                        });

                        user
                    }
                    DialectEnum::POSTGRES => {
                        let query =
                            sqlx::query("SELECT * FROM subgraph_user WHERE identifier = $1;")
                                .bind(&identifier);

                        let user = match sql_ds.pool.clone() {
                            PoolEnum::Postgres(pool) => query.fetch_one(&pool).await,
                            _ => panic!("Pool not supported."),
                        }
                        .map(|pg_row| {
                            let identifier = pg_row.try_get("identifier").unwrap_or("").to_string();
                            let registration_state =
                                pg_row.try_get("registration_state").unwrap_or("");
                            let pub_key = pg_row.try_get("pub_key").unwrap_or("");
                            let user = ServiceUser {
                                identifier,
                                registration_state: serde_json::from_str(&registration_state)
                                    .expect("Failed to deserialize registration state"),
                                pub_key: serde_json::from_str(&pub_key)
                                    .expect("Failed to deserialize pub key"),
                            };
                            Some(user)
                        })
                        .map_err(|e| {
                            async_graphql::Error::new(format!(
                                "Failed to get user from postgres: {:?}",
                                e
                            ))
                        });

                        user
                    }
                };
                user
            }
            _ => panic!("Data Source not supported."),
        };

        debug!("User: {:?}", &user);

        user
    }
}
