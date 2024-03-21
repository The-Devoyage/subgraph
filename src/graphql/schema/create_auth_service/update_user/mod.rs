use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::{sql::PoolEnum, DataSource},
    graphql::schema::ServiceSchema,
};
use bson::{doc, Regex};
use log::debug;
use mongodb::options::FindOneAndUpdateOptions;

use super::ServiceUser;

pub struct UpdateUserInput {
    pub identifier: String,
    pub registration_state: Option<String>,
    pub passkey: Option<String>,
    pub authentication_state: Option<String>,
}

impl ServiceSchema {
    fn create_set_options(update_user_input: UpdateUserInput) -> String {
        let mut set_options = String::new();
        if let Some(registration_state) = update_user_input.registration_state {
            set_options.push_str(&format!("registration_state = '{}'", registration_state));
        }
        if let Some(passkey) = update_user_input.passkey {
            if !set_options.is_empty() {
                set_options.push_str(", ");
            }
            set_options.push_str(&format!("passkey = '{}'", passkey));
        }
        if let Some(authentication_state) = update_user_input.authentication_state {
            if !set_options.is_empty() {
                set_options.push_str(", ");
            }
            set_options.push_str(&format!(
                "authentication_state = '{}'",
                authentication_state
            ));
        }

        set_options
    }

    pub async fn update_user(
        data_source: &DataSource,
        service_user: ServiceUser,
    ) -> Result<(), async_graphql::Error> {
        debug!("Updating user: {:?}", &service_user);

        let updated: Result<(), async_graphql::Error> = match &data_source {
            DataSource::Mongo(mongo_ds) => {
                let identifer_regex = Regex {
                    pattern: service_user.identifier.to_string(),
                    options: "i".to_string(),
                };
                let filter = doc! {
                    "identifier": identifer_regex
                };
                let mut update = doc! {
                    "$set": {
                        "registration_state": serde_json::to_string(&service_user.registration_state).unwrap(),
                        "passkey": serde_json::to_string(&service_user.passkey).unwrap(),
                    }
                };
                if let Some(authentication_state) = service_user.authentication_state {
                    update.insert(
                        "$set",
                        doc! {
                            "authentication_state": serde_json::to_string(&authentication_state).unwrap(),
                        },
                    );
                }
                let options = FindOneAndUpdateOptions::builder().upsert(true).build();
                let user = mongo_ds
                    .db
                    .collection::<ServiceUser>("subgraph_user")
                    .find_one_and_update(filter, update, options)
                    .await;

                match user {
                    Ok(user) => {
                        if let Some(_user) = user {
                            Ok(())
                        } else {
                            Err(async_graphql::Error::new(format!("Could not find user.")))
                        }
                    }
                    Err(e) => Err(async_graphql::Error::new(format!(
                        "Something went wrong when finding the user: {:?}",
                        e
                    ))),
                }
            }
            DataSource::SQL(sql_ds) => match sql_ds.config.dialect {
                DialectEnum::MYSQL => {
                    let set_query = ServiceSchema::create_set_options(UpdateUserInput {
                        identifier: service_user.identifier.clone(),
                        registration_state: Some(
                            serde_json::to_string(&service_user.registration_state)
                                .unwrap_or("".to_string())
                                .to_string(),
                        ),
                        passkey: Some(
                            serde_json::to_string(&service_user.passkey)
                                .unwrap_or("".to_string())
                                .to_string(),
                        ),
                        authentication_state: Some(
                            serde_json::to_string(&service_user.authentication_state)
                                .unwrap_or("".to_string())
                                .to_string(),
                        ),
                    });
                    let query_string = format!(
                        "UPDATE subgraph_user SET {} WHERE LOWER(identifier) = (?);",
                        set_query
                    );
                    let query = sqlx::query(&query_string).bind(service_user.identifier.clone());
                    let result = match sql_ds.pool.clone() {
                        PoolEnum::MySql(pool) => query.execute(&pool).await,
                        _ => panic!("Pool not supported."),
                    };

                    match result {
                        Ok(res) => {
                            if res.rows_affected() > 0 {
                                Ok(())
                            } else {
                                Err(async_graphql::Error::new(format!("Could not find user.")))
                            }
                        }
                        Err(error) => Err(async_graphql::Error::new(format!(
                            "Could not update user: {:?}",
                            error
                        ))),
                    }
                }
                DialectEnum::POSTGRES => {
                    let set_query = ServiceSchema::create_set_options(UpdateUserInput {
                        identifier: service_user.identifier.clone(),
                        registration_state: Some(
                            serde_json::to_string(&service_user.registration_state)
                                .unwrap_or("".to_string())
                                .to_string(),
                        ),
                        passkey: Some(
                            serde_json::to_string(&service_user.passkey)
                                .unwrap_or("".to_string())
                                .to_string(),
                        ),
                        authentication_state: Some(
                            serde_json::to_string(&service_user.authentication_state)
                                .unwrap_or("".to_string())
                                .to_string(),
                        ),
                    });
                    let query_string = format!(
                        "UPDATE subgraph_user SET {} WHERE LOWER(identifier) = LOWER($1) RETURNING *;",
                        set_query
                    );
                    let query = sqlx::query(&query_string).bind(service_user.identifier.clone());
                    let result = match sql_ds.pool.clone() {
                        PoolEnum::Postgres(pool) => query.fetch_one(&pool).await,
                        _ => panic!("Pool not supported."),
                    };

                    match result {
                        Ok(_) => Ok(()),
                        Err(error) => Err(async_graphql::Error::new(format!(
                            "Could not update user: {:?}",
                            error
                        ))),
                    }
                }
                DialectEnum::SQLITE => {
                    let set_query = ServiceSchema::create_set_options(UpdateUserInput {
                        identifier: service_user.identifier.clone(),
                        registration_state: Some(
                            serde_json::to_string(&service_user.registration_state)
                                .unwrap_or("".to_string())
                                .to_string(),
                        ),
                        passkey: Some(
                            serde_json::to_string(&service_user.passkey)
                                .unwrap_or("".to_string())
                                .to_string(),
                        ),
                        authentication_state: Some(
                            serde_json::to_string(&service_user.authentication_state)
                                .unwrap_or("".to_string())
                                .to_string(),
                        ),
                    });
                    let query_string = format!(
                        "UPDATE subgraph_user SET {} WHERE LOWER(identifier) = LOWER(?);",
                        set_query
                    );
                    let query = sqlx::query(&query_string).bind(service_user.identifier.clone());
                    let result = match sql_ds.pool.clone() {
                        PoolEnum::SqLite(pool) => query.execute(&pool).await,
                        _ => panic!("Pool not supported."),
                    };

                    match result {
                        Ok(res) => {
                            if res.rows_affected() > 0 {
                                Ok(())
                            } else {
                                Err(async_graphql::Error::new(format!("Could not find user.")))
                            }
                        }
                        Err(error) => Err(async_graphql::Error::new(format!(
                            "Could not update user: {:?}",
                            error
                        ))),
                    }
                }
            },
            _ => Err(async_graphql::Error::new(format!("Failed to find user."))),
        };

        debug!("Updated user");

        updated
    }
}
