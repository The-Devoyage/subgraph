use bson::{doc, Regex};
use log::{debug, error, trace};
use sqlx::Row;

use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::{sql::PoolEnum, DataSource},
    graphql::schema::{create_auth_service::ServiceUser, ServiceSchema},
};

impl ServiceSchema {
    pub async fn get_user(
        data_source: &DataSource,
        identifier: &str,
    ) -> Result<Option<ServiceUser>, async_graphql::Error> {
        debug!("Get User");
        trace!("Identifier: {:?}", &identifier);

        let user = match &data_source {
            DataSource::Mongo(mongo_ds) => {
                trace!("Getting User - Mongo Data Source");

                let identifier_regex = Regex {
                    pattern: identifier.to_string(),
                    options: "i".to_string(),
                };

                let filter = doc! {
                    "identifier": identifier_regex
                };

                let user = mongo_ds
                    .db
                    .collection::<ServiceUser>("subgraph_user")
                    .find_one(filter, None)
                    .await;

                user.map_err(|e| {
                    error!("Failed to get user from mongo: {:?}", e);
                    async_graphql::Error::new(format!("Failed to get user from MongoDB: {:?}", e))
                })
            }
            DataSource::SQL(sql_ds) => {
                trace!("SQL data source");
                let user = match sql_ds.config.dialect {
                    DialectEnum::MYSQL => {
                        let query = sqlx::query(
                            "SELECT * FROM subgraph_user WHERE LOWER(identifier) = LOWER(?);",
                        )
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
                            let passkey = mysql_row.try_get("passkey").unwrap_or("");
                            let authentication_state =
                                mysql_row.try_get("authentication_state").unwrap_or("");
                            let uuid = mysql_row.try_get("uuid").unwrap_or("").to_string();

                            let user = ServiceUser {
                                uuid: uuid::Uuid::parse_str(&uuid).unwrap_or(uuid::Uuid::nil()),
                                identifier,
                                registration_state: serde_json::from_str(&registration_state)
                                    .expect("Failed to deserialize registration state"),
                                passkey: serde_json::from_str(&passkey).unwrap_or(None),
                                authentication_state: serde_json::from_str(&authentication_state)
                                    .unwrap_or(None),
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
                        let query = sqlx::query(
                            "SELECT * FROM subgraph_user WHERE LOWER(identifier) = LOWER(?);",
                        )
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
                            let passkey = sqlite_row.try_get("passkey").unwrap_or("");
                            let authentication_state =
                                sqlite_row.try_get("authentication_state").unwrap_or("");
                            let uuid = sqlite_row.try_get("uuid").unwrap_or("").to_string();
                            let user = ServiceUser {
                                uuid: uuid::Uuid::parse_str(&uuid).unwrap_or(uuid::Uuid::nil()),
                                identifier,
                                registration_state: serde_json::from_str(&registration_state)
                                    .expect("Failed to deserialize registration state"),
                                passkey: serde_json::from_str(&passkey).unwrap_or(None),
                                authentication_state: serde_json::from_str(&authentication_state)
                                    .unwrap_or(None),
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
                        let query = sqlx::query(
                            "SELECT * FROM subgraph_user WHERE LOWER(identifier) = ($1);",
                        )
                        .bind(&identifier);

                        let user = match sql_ds.pool.clone() {
                            PoolEnum::Postgres(pool) => query.fetch_one(&pool).await,
                            _ => panic!("Pool not supported."),
                        }
                        .map(|pg_row| {
                            let identifier = pg_row.try_get("identifier").unwrap_or("").to_string();
                            let registration_state =
                                pg_row.try_get("registration_state").unwrap_or("");
                            let passkey = pg_row.try_get("passkey").unwrap_or("");
                            let authentication_state =
                                pg_row.try_get("authentication_state").unwrap_or("");
                            let uuid = pg_row.try_get("uuid").unwrap_or(uuid::Uuid::nil());
                            let user = ServiceUser {
                                uuid,
                                identifier,
                                registration_state: serde_json::from_str(&registration_state)
                                    .expect("Failed to deserialize registration state"),
                                passkey: serde_json::from_str(&passkey).unwrap_or(None),
                                authentication_state: serde_json::from_str(&authentication_state)
                                    .unwrap_or(None),
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
            _ => {
                error!("Data Source not supported.");
                return Err(async_graphql::Error::new("Data Source not supported."));
            }
        };

        trace!("User: {:?}", &user);

        user
    }
}
