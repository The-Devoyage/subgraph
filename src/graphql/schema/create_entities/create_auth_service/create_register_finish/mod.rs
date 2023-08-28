use async_graphql::{
    dynamic::{Field, FieldFuture, InputValue, TypeRef},
    Value,
};
use bson::doc;

use crate::{
    data_sources::{DataSource, DataSources},
    graphql::schema::ServiceSchemaBuilder,
};

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

                    let pub_key = match ctx.args.try_get("pub_key") {
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

                    let data_sources = ctx.data_unchecked::<DataSources>().clone();

                    let data_source = DataSources::get_data_source_by_name(
                        &data_sources,
                        &auth_config.data_source,
                    );

                    let user = match data_source.clone() {
                        DataSource::Mongo(mongo_ds) => {
                            let filter = doc! {
                                "identifier": identifier.clone()
                            };
                            let user = mongo_ds
                                .db
                                .collection::<ServiceUser>("subgraph_user")
                                .find_one(filter, None)
                                .await?;

                            match user {
                                Some(user) => user,
                                None => {
                                    return Err(async_graphql::Error::new(format!(
                                        "User not found."
                                    )))
                                }
                            }
                        }
                        _ => panic!("Data source not supported."),
                    };

                    let webauthn = ServiceSchemaBuilder::build_webauthn(&auth_config)?;
                    let pub_key = serde_json::from_str(&pub_key).unwrap();

                    let sk = webauthn
                        .finish_passkey_registration(&pub_key, &user.registration_state)
                        .map_err(|e| {
                            async_graphql::Error::new(format!(
                                "Failed to finish registration: {:?}",
                                e
                            ))
                        })?;

                    // Update user with sk
                    let filter = doc! {
                        "identifier": identifier
                    };
                    let update = doc! {
                        "$set": {
                            "sk": serde_json::to_string(&sk).unwrap()
                        }
                    };

                    match data_source.clone() {
                        DataSource::Mongo(mongo_ds) => {
                            mongo_ds
                                .db
                                .collection::<ServiceUser>("subgraph_user")
                                .update_one(filter, update, None)
                                .await?;
                        }
                        _ => panic!("Data source not supported."),
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
            "pub_key",
            TypeRef::named_nn(TypeRef::STRING),
        ));

        self.mutation = self.mutation.field(resolver);

        self
    }
}
