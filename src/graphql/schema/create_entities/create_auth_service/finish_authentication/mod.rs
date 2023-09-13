use async_graphql::{
    dynamic::{Field, FieldFuture, InputValue, TypeRef},
    Value,
};
use webauthn_rs::prelude::PublicKeyCredential;

use crate::{data_sources::DataSources, graphql::schema::ServiceSchemaBuilder};

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
            TypeRef::named_nn(TypeRef::STRING),
            move |ctx| {
                let auth_config = auth_config.clone();

                FieldFuture::new(async move {
                    let data_sources = ctx.data_unchecked::<DataSources>();
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

                    if user.is_none() {
                        return Err(async_graphql::Error::new(format!("User not found.")));
                    };

                    if user.clone().unwrap().passkey.is_none() {
                        return Err(async_graphql::Error::new(format!(
                            "User does not have a passkey."
                        )));
                    };

                    if user.clone().unwrap().authentication_state.is_none() {
                        return Err(async_graphql::Error::new(format!(
                            "User does not have an authentication state."
                        )));
                    };

                    let webauthn = ServiceSchemaBuilder::build_webauthn(&auth_config)?;

                    webauthn
                        .finish_passkey_authentication(
                            &pub_key,
                            &user.unwrap().authentication_state.unwrap(),
                        )
                        .map_err(|e| {
                            async_graphql::Error::new(format!(
                                "Failed to finish authentication: {:?}",
                                e
                            ))
                        })?;

                    Ok(Some(Value::from("Auth success")))
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
