use async_graphql::{
    dynamic::{Field, FieldFuture, InputValue, TypeRef},
    Value,
};

use crate::{data_sources::DataSources, graphql::schema::ServiceSchemaBuilder};

use super::ServiceUser;

impl ServiceSchemaBuilder {
    pub fn create_authenticate_start(mut self) -> Self {
        let auth_config = match self.subgraph_config.service.auth.clone() {
            Some(auth) => auth,
            None => {
                panic!("Auth config not found.");
            }
        };

        let resolver = Field::new(
            "authenticate_start",
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

                    let data_sources = ctx.data_unchecked::<DataSources>();
                    let data_source = DataSources::get_data_source_by_name(
                        &data_sources,
                        &auth_config.data_source,
                    );

                    let user = ServiceSchemaBuilder::get_user(&data_source, &identifier).await;

                    let user = match user {
                        Ok(user) => user,
                        Err(e) => {
                            return Err(async_graphql::Error::new(format!(
                                "Failed to get user: {:?}",
                                e
                            )))
                        }
                    };

                    let user = match user {
                        Some(user) => user,
                        None => return Err(async_graphql::Error::new(format!("User not found."))),
                    };

                    if user.passkey.is_none() {
                        return Err(async_graphql::Error::new(format!(
                            "User does not have a passkey."
                        )));
                    };

                    let webauthn = ServiceSchemaBuilder::build_webauthn(&auth_config)?;

                    let (rcr, auth_state) = webauthn
                        .start_passkey_authentication(&vec![user.clone().passkey.unwrap()])
                        .map_err(|e| {
                            async_graphql::Error::new(format!(
                                "Failed to start authentication: {:?}",
                                e
                            ))
                        })?;

                    //Update the user property auth_state
                    let service_user = ServiceUser {
                        identifier: identifier.clone(),
                        registration_state: user.registration_state,
                        passkey: user.passkey,
                        authentication_state: Some(auth_state),
                        uuid: user.uuid,
                    };

                    let updated =
                        ServiceSchemaBuilder::update_user(&data_source, service_user).await;

                    match updated {
                        Ok(_) => {
                            let rcr_json = match serde_json::to_value(&rcr) {
                                Ok(rcr_json) => rcr_json,
                                Err(e) => {
                                    return Err(async_graphql::Error::new(format!(
                                        "Failed to serialize rcr: {:?}",
                                        e
                                    )))
                                }
                            };
                            let value = Value::from_json(rcr_json);
                            match value {
                                Ok(value) => Ok(Some(value)),
                                Err(e) => {
                                    return Err(async_graphql::Error::new(format!(
                                        "Failed to create value: {:?}",
                                        e
                                    )))
                                }
                            }
                        }
                        Err(e) => {
                            return Err(async_graphql::Error::new(format!(
                                "Failed to update user: {:?}",
                                e
                            )))
                        }
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
