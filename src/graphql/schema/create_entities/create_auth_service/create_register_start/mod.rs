use async_graphql::{
    dynamic::{Field, FieldFuture, InputValue, TypeRef},
    Value,
};
use bson::doc;
use log::{debug, error};

use crate::{
    data_sources::{DataSource, DataSources},
    graphql::schema::{create_entities::create_auth_service::ServiceUser, ServiceSchemaBuilder},
};

impl ServiceSchemaBuilder {
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

                    // Check if user exists. If exists then reject.
                    match &data_source {
                        DataSource::Mongo(mongo_ds) => {
                            debug!("Mongo data source");
                            let filter = doc! {
                                "identifier": &identifier
                            };

                            let user = mongo_ds
                                .db
                                .collection::<ServiceUser>("subgraph_user")
                                .find_one(filter, None)
                                .await;

                            match user {
                                Ok(user) => {
                                    debug!("User: {:?}", user);

                                    if user.is_some() {
                                        error!("User already exists: {:?}", user);
                                        return Err(async_graphql::Error::new(format!(
                                            "User already exists: {:?}",
                                            user
                                        )));
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to find user: {:?}", e);
                                    return Err(async_graphql::Error::new(format!(
                                        "Failed to find user: {:?}",
                                        e
                                    )));
                                }
                            };
                        }
                        _ => panic!("Data Source not supported."),
                    };

                    debug!("Creating webauthn service");

                    let webauthn = ServiceSchemaBuilder::build_webauthn(&auth_config)?;

                    let (ccr, reg_state) = webauthn.start_passkey_registration(
                        uuid::Uuid::new_v4(),
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

                    // Save registration state to database
                    match data_source {
                        DataSource::Mongo(mongo_ds) => {
                            let user = doc! {
                                "identifier": identifier,
                                "registration_state": &reg_state
                            };

                            mongo_ds
                                .db
                                .collection("subgraph_user")
                                .insert_one(user, None)
                                .await?;
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
