use async_graphql::{
    dynamic::{Field, FieldFuture, FieldValue, InputValue, Object, TypeRef},
    Value,
};
use biscuit_auth::{Biscuit, KeyPair};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::PublicKeyCredential;

use crate::{data_sources::DataSources, graphql::schema::ServiceSchemaBuilder};

use super::ID;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthenticateSuccess {
    pub token: String,
    pub user_id: String,
}

impl ServiceSchemaBuilder {
    pub fn create_authenticate_finish(mut self) -> Self {
        let auth_config = match self.subgraph_config.service.auth.clone() {
            Some(auth) => auth,
            None => {
                panic!("Auth config not found.");
            }
        };

        let resolver = Field::new(
            "authenticate_finish",
            TypeRef::named_nn("authenticate_success"),
            move |ctx| {
                let auth_config = auth_config.clone();

                FieldFuture::new(async move {
                    let data_sources = ctx.data_unchecked::<DataSources>();
                    let data_source = DataSources::get_data_source_by_name(
                        &data_sources,
                        &auth_config.data_source,
                    );
                    let key_pair = match ctx.data_unchecked::<Option<KeyPair>>() {
                        Some(key_pair) => key_pair,
                        None => {
                            return Err(async_graphql::Error::new(format!(
                                "Failed to get key pair."
                            )));
                        }
                    };

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
                            return Err(async_graphql::Error::new(format!(
                                "Failed to get input: {:?}",
                                e
                            )));
                        }
                    };
                    let pub_key: Result<PublicKeyCredential, async_graphql::Error> =
                        serde_json::from_str(&pub_key).map_err(|e| {
                            async_graphql::Error::new(format!("Failed to deserialize: {:?}", e))
                        });
                    let pub_key = match pub_key {
                        Ok(pk) => pk,
                        Err(error) => {
                            return Err(error);
                        }
                    };

                    let user = ServiceSchemaBuilder::get_user(&data_source, &identifier).await?;

                    let user = match user {
                        Some(user) => {
                            if user.passkey.is_none() {
                                return Err(async_graphql::Error::new(format!(
                                    "User does not have a passkey."
                                )));
                            };

                            if user.authentication_state.is_none() {
                                return Err(async_graphql::Error::new(format!(
                                    "User does not have an authentication state."
                                )));
                            };
                            user
                        }
                        None => {
                            return Err(async_graphql::Error::new(format!("User not found.")));
                        }
                    };

                    let webauthn = ServiceSchemaBuilder::build_webauthn(&auth_config)?;

                    webauthn
                        .finish_passkey_authentication(
                            &pub_key,
                            &user.authentication_state.unwrap(),
                        )
                        .map_err(|e| {
                            async_graphql::Error::new(format!(
                                "Failed to finish authentication: {:?}",
                                e
                            ))
                        })?;

                    let user_id = match user.id.clone() {
                        ID::Int(id) => id.to_string(),
                        ID::String(id) => id,
                    };

                    debug!("User id: {}", user_id);

                    // Expires in one hour.
                    let expires_at = chrono::Utc::now()
                        .checked_add_signed(chrono::Duration::seconds(3600 as i64))
                        .expect("Failed to add duration.")
                        .to_string();

                    let mut biscuit = Biscuit::builder();
                    biscuit
                        .add_fact(format!("user(\"{}\", {})", identifier, user_id).as_str())
                        .map_err(|e| {
                            async_graphql::Error::new(format!("Failed to add fact: {:?}", e))
                        })?;
                    let biscuit = biscuit.build(key_pair).map_err(|e| {
                        async_graphql::Error::new(format!("Failed to build biscuit: {:?}", e))
                    })?;
                    let base64 = biscuit.to_base64().map_err(|e| {
                        async_graphql::Error::new(format!("Failed to convert to base64: {:?}", e))
                    })?;

                    let response_value = serde_json::to_value(AuthenticateSuccess {
                        token: base64.clone(),
                        user_id: user.id.clone().to_string(),
                    })
                    .map_err(|e| {
                        async_graphql::Error::new(format!("Failed to serialize: {:?}", e))
                    })?;

                    Ok(Some(FieldValue::owned_any(response_value)))
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

        let authentication_success = Object::new("authenticate_success")
            .field(Field::new(
                "token",
                TypeRef::named_nn(TypeRef::STRING),
                move |ctx| {
                    FieldFuture::new(async move {
                        let parent_value = ctx
                            .parent_value
                            .try_downcast_ref::<serde_json::Value>()
                            .map_err(|e| {
                                error!("Failed to downcast: {:?}", e);
                                async_graphql::Error::new(format!("Failed to downcast: {:?}", e))
                            })?;
                        let token = parent_value["token"].as_str().ok_or_else(|| {
                            error!("Failed to get token.");
                            async_graphql::Error::new(format!("Failed to get token."))
                        })?;

                        Ok(Some(Value::from(token.clone())))
                    })
                },
            ))
            .field(Field::new(
                "user_id",
                TypeRef::named_nn(TypeRef::STRING),
                move |ctx| {
                    FieldFuture::new(async move {
                        let parent_value = ctx
                            .parent_value
                            .try_downcast_ref::<serde_json::Value>()
                            .map_err(|e| {
                                error!("Failed to downcast: {:?}", e);
                                async_graphql::Error::new(format!("Failed to downcast: {:?}", e))
                            })?;
                        let user_id = parent_value["user_id"].as_str().ok_or_else(|| {
                            error!("Failed to get user_id.");
                            async_graphql::Error::new(format!("Failed to get user_id."))
                        })?;

                        Ok(Some(Value::from(user_id.clone())))
                    })
                },
            ));

        self = self.register_types(vec![authentication_success]);
        self.mutation = self.mutation.field(resolver);
        self
    }
}
