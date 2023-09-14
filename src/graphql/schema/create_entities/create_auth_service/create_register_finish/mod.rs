use async_graphql::{
    dynamic::{Field, FieldFuture, InputValue, TypeRef},
    Value,
};
use log::error;
use webauthn_rs::prelude::RegisterPublicKeyCredential;

use crate::{data_sources::DataSources, graphql::schema::ServiceSchemaBuilder};

use super::ServiceUser;

impl ServiceSchemaBuilder {
    pub fn create_register_finish(mut self) -> Self {
        let auth_config = match self.subgraph_config.service.auth.clone() {
            Some(auth) => auth,
            None => {
                panic!("Auth config not found.");
            }
        };

        let resolver = Field::new(
            "register_finish",
            TypeRef::named_nn(TypeRef::BOOLEAN),
            move |ctx| {
                let auth_config = auth_config.clone();

                FieldFuture::new(async move {
                    let data_sources = ctx.data_unchecked::<DataSources>().clone();

                    let data_source = DataSources::get_data_source_by_name(
                        &data_sources,
                        &auth_config.data_source,
                    );

                    let identifier = match ctx.args.try_get("identifier") {
                        Ok(input) => input
                            .deserialize::<String>()
                            .expect("Failed to deserialize."),
                        Err(e) => {
                            return Err(async_graphql::Error::new(format!(
                                "Failed to get input: {:?}",
                                e
                            )));
                        }
                    };

                    let pub_key = match ctx.args.try_get("public_key") {
                        Ok(input) => input
                            .deserialize::<String>()
                            .expect("Failed to deserialize."),
                        Err(e) => {
                            ServiceSchemaBuilder::delete_user(&data_source, &identifier).await?;
                            return Err(async_graphql::Error::new(format!(
                                "Failed to get input: {:?}",
                                e
                            )));
                        }
                    };

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

                    if user.is_none() {
                        error!("User Not Found: {:?}", &identifier);
                        return Err(async_graphql::Error::new(format!(
                            "User Not Found: {:?}",
                            &identifier
                        )));
                    }

                    let webauthn = match ServiceSchemaBuilder::build_webauthn(&auth_config) {
                        Ok(w) => Ok(w),
                        Err(e) => {
                            ServiceSchemaBuilder::delete_user(&data_source, &identifier).await?;
                            Err(async_graphql::Error::new(format!(
                                "something went wrong when building webauthn: {:?}",
                                e
                            )))
                        }
                    }?;

                    let pub_key: Result<RegisterPublicKeyCredential, async_graphql::Error> =
                        serde_json::from_str(&pub_key).map_err(|e| {
                            async_graphql::Error::new(format!("Failed to deserialize: {:?}", e))
                        });

                    let pub_key = match pub_key {
                        Ok(pk) => pk,
                        Err(error) => {
                            ServiceSchemaBuilder::delete_user(&data_source, &identifier).await?;
                            return Err(error);
                        }
                    };

                    let passkey = webauthn
                        .finish_passkey_registration(
                            &pub_key,
                            &user.clone().unwrap().registration_state,
                        )
                        .map_err(|e| {
                            async_graphql::Error::new(format!(
                                "Failed to finish registration: {:?}",
                                e
                            ))
                        });
                    let passkey = match passkey {
                        Ok(pk) => pk,
                        Err(error) => {
                            ServiceSchemaBuilder::delete_user(&data_source, &identifier).await?;
                            return Err(error);
                        }
                    };

                    // Update user with sk
                    let service_user = ServiceUser {
                        identifier: identifier.clone(),
                        registration_state: user.clone().unwrap().registration_state,
                        passkey: Some(passkey),
                        authentication_state: None,
                        id: user.unwrap().id,
                    };

                    let updated =
                        ServiceSchemaBuilder::update_user(&data_source, service_user).await;
                    match updated {
                        Ok(_) => (),
                        Err(e) => {
                            ServiceSchemaBuilder::delete_user(&data_source, &identifier).await?;
                            return Err(e);
                        }
                    };

                    Ok(Some(Value::from(true)))
                })
            },
        )
        .argument(InputValue::new(
            "identifier",
            TypeRef::named_nn(TypeRef::STRING),
        ))
        .argument(InputValue::new(
            "public_key",
            TypeRef::named_nn(TypeRef::STRING),
        ));

        self.mutation = self.mutation.field(resolver);
        self
    }
}
