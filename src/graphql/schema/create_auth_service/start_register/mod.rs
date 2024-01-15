use async_graphql::{
    dynamic::{Field, FieldFuture, InputValue, TypeRef},
    Value,
};
use bson::{doc, Bson};
use log::{debug, error};

use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::{sql::PoolEnum, DataSource, DataSources},
    graphql::schema::ServiceSchema,
};

impl ServiceSchema {
    pub fn create_register_start(mut self) -> Self {
        let auth_config = match self.subgraph_config.service.auth.clone() {
            Some(auth) => auth,
            None => {
                panic!("Auth config not found.");
            }
        };

        let resolver = Field::new(
            "register_start",
            TypeRef::named_nn(TypeRef::STRING),
            move |ctx| {
                let auth_config = auth_config.clone();

                FieldFuture::new(async move {
                    let identifier = match ctx.args.try_get("identifier") {
                        Ok(input) => input
                            .deserialize::<String>()
                            .expect("Failed to deserialize."),
                        Err(e) => {
                            return Err(async_graphql::Error::new(format!(
                                "Failed to get input: {:?}",
                                e
                            )))
                        }
                    };

                    //match name of data source to the auth_config.data_source string
                    let data_sources = ctx.data_unchecked::<DataSources>();
                    let data_source = DataSources::get_data_source_by_name(
                        &data_sources,
                        &auth_config.data_source,
                    );

                    // Check if user exists. If previous register, reject, else delete the user.
                    let user = ServiceSchema::get_user(&data_source, &identifier).await; 

                    if !user.is_err() && user.clone().unwrap().clone().is_some() {
                        if user.clone().unwrap().unwrap().passkey.is_some() {
                            error!("User already exists: {:?}", &identifier);
                            return Err(async_graphql::Error::new(format!(
                                "User already exists: {:?}",
                                &identifier
                            )));
                        }else {
                            ServiceSchema::delete_user(&data_source, &identifier).await?;
                        }
                    }

                    debug!("Creating webauthn service");

                    let webauthn = ServiceSchema::build_webauthn(&auth_config)?;

                    let user_uuid = uuid::Uuid::new_v4();

                    let (ccr, reg_state) = webauthn.start_passkey_registration(
                        user_uuid.clone(),
                        &identifier,
                        &identifier,
                        None,
                    )?;

                    let reg_state = match serde_json::to_string(&reg_state) {
                        Ok(reg_state) => reg_state,
                        Err(e) => {
                            return Err(async_graphql::Error::new(format!(
                                "Failed to serialize registration state: {}",
                                e
                            )))
                        }
                    };

                    let user_uuid_string = user_uuid.to_string();

                    // Save registration state to database
                    match &data_source {
                        DataSource::Mongo(mongo_ds) => {
                            let user = doc! {
                                "uuid": user_uuid_string.to_string(),
                                "identifier": identifier.clone(),
                                "registration_state": &reg_state,
                                "authentication_state": Bson::Null,
                                "passkey": Bson::Null,
                            };

                            mongo_ds
                                .db
                                .collection("subgraph_user")
                                .insert_one(user, None)
                                .await?;
                        }
                        DataSource::SQL(sql_ds) => {
                            match sql_ds.config.dialect {
                                DialectEnum::MYSQL => {
                                    let query = sqlx::query("INSERT INTO subgraph_user (uuid, identifier, registration_state) VALUES (?, ?, ?);")
                                        .bind(&user_uuid_string)
                                        .bind(&identifier)
                                        .bind(&reg_state);
                                    match sql_ds.pool.clone() {
                                        PoolEnum::MySql(pool) => {
                                            query.execute(&pool).await?;
                                        }
                                        _ => unreachable!(),
                                    };
                                }
                                DialectEnum::SQLITE => {
                                    let query = sqlx::query("INSERT INTO subgraph_user (uuid, identifier, registration_state) VALUES (?, ?, ?);")
                                        .bind(&user_uuid_string)
                                        .bind(&identifier)
                                        .bind(&reg_state);
                                    match sql_ds.pool.clone() {
                                        PoolEnum::SqLite(pool) => {
                                            query.execute(&pool).await?;
                                        }
                                        _ => unreachable!(),
                                    };
                                }
                                DialectEnum::POSTGRES => {
                                    let query = sqlx::query("INSERT INTO subgraph_user (uuid, identifier, registration_state) VALUES ($1, $2, $3);")
                                        .bind(&user_uuid)
                                        .bind(&identifier)
                                        .bind(&reg_state);
                                    match sql_ds.pool.clone() {
                                        PoolEnum::Postgres(pool) => {
                                            query.execute(&pool).await?;
                                        }
                                        _ => unreachable!(),
                                    };
                                }
                            };
                        }
                        _ => panic!("Data Source not supported."),
                    };

                    debug!("Challenge created: {:?}", ccr);
                    debug!("Registration state created: {:?}", reg_state);

                    let json = match serde_json::to_value(&ccr) {
                        Ok(json) => json,
                        Err(e) => {
                            return Err(async_graphql::Error::new(format!(
                                "Failed to serialize challenge: {}",
                                e
                            )))
                        }
                    };

                    let value = Value::from_json(json);

                    match value {
                        Ok(value) => Ok(Some(value)),
                        Err(_) => Err(async_graphql::Error::new("Failed to resolve challenge.")),
                    }
                })
            },
        )
        .argument(InputValue::new(
            "identifier",
            TypeRef::named_nn(TypeRef::STRING),
        ));

        self.mutation = self.mutation.field(resolver);
        self
    }
}
