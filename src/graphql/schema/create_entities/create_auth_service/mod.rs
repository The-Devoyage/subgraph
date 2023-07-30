use async_graphql::{
    dynamic::{Field, FieldFuture, FieldValue, InputValue, Object, TypeRef},
    Value,
};
use json::object;
use log::debug;
use reqwest::Url;
use webauthn_rs::prelude::{CreationChallengeResponse, WebauthnBuilder};

use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityFieldConfig, ScalarOptions, ServiceEntityConfig,
    },
    graphql::entity::ServiceEntity,
};

use super::ServiceSchemaBuilder;

impl ServiceSchemaBuilder {
    pub fn create_auth_service(mut self) -> Self {
        self = self.create_register_start();
        self
    }

    pub fn create_register_start(mut self) -> Self {
        let resolver = Field::new("register_start", TypeRef::named_nn("ccr"), move |ctx| {
            FieldFuture::new(async move {
                //Create webauthn object.
                let rp_id = "localhost";
                let rp_origin = Url::parse("http://localhost:8080").expect("Invalid URL");
                let webauthn = WebauthnBuilder::new(rp_id, &rp_origin)
                    .expect("Failed to create Webauthn object")
                    .rp_name("Subgraph RP NAME")
                    .build()
                    .expect("Failed to build Webauthn object");
                debug!("Webauthn object created: {:?}", webauthn);
                //Check if user exists in database.
                //If registered, return error.
                //If not exists, then create user.

                //create challenge
                let (ccr, reg_state) = webauthn.start_passkey_registration(
                    uuid::Uuid::new_v4(),
                    "nickisyourfan@gmail.com",
                    "nickisyourfan@gmail.com",
                    None,
                )?;

                debug!("Challenge created: {:?}", ccr);
                debug!("Registration state created: {:?}", reg_state);

                let string = serde_json::to_string(&ccr).expect("Failed to serialize ccr");

                let json = json::parse(&string).expect("Failed to parse ccr");

                debug!("Converted ccr to json: {:?}", json);

                Ok(Some(FieldValue::owned_any(json)))
            })
        })
        .argument(InputValue::new(
            "identifier",
            TypeRef::named_nn(TypeRef::STRING),
        ));

        self.query = self.query.field(resolver);

        let ccr_entity = self.create_ccr_entity();

        let ccr_typedefs = ServiceEntity::new(
            self.data_sources.clone(),
            ccr_entity.clone(),
            "ccr".to_string(),
            ccr_entity.fields,
            self.subgraph_config.clone(),
        )
        .build();

        // let resolver_fields = Object::new(entity_response).field(Field::new(
        //     "ccr",
        //     TypeRef::named_nn("ccr"),
        //     move |ctx| {
        //         FieldFuture::new(async move {
        //             // let ccr = match ctx
        //             //     .parent_value
        //             //     .try_downcast_ref::<CreationChallengeResponse>()
        //             // {
        //             //     Ok(ccr) => serde_json::to_string(ccr),
        //             //     Err(_) => {
        //             //         return Err(async_graphql::Error::new("Failed to resolve challenge."))
        //             //     }
        //             // };
        //             //
        //             let ccr = ctx.parent_value.as_value();

        //             if ccr.is_some() {
        //                 let ccr = ccr.unwrap();
        //                 // debug!("Ccr: {:?}", ccr);
        //                 return Ok(Some(FieldValue::owned_any(ccr.clone())));
        //             } else {
        //                 return Err(async_graphql::Error::new("Failed to resolve challenge."));
        //             }

        //             // let ccr = match ccr {
        //             //     Ok(ccr) => json::parse(&ccr),
        //             //     Err(_) => {
        //             //         return Err(async_graphql::Error::new("Failed to resolve challenge."))
        //             //     }
        //             // };

        //             // let string = serde_json::to_string(&ccr).expect("Failed to serialize ccr");

        //             // let json = json::parse(&string).expect("Failed to parse ccr");

        //             // debug!("Converted ccr to json: {:?}", json);

        //             // Ok(Some(Value::into_json(ccr)))
        //         })
        //     },
        // ));

        self = self.register_types(ccr_typedefs);

        self
    }

    fn create_ccr_entity(&self) -> ServiceEntityConfig {
        ServiceEntityConfig {
            name: "ccr".to_string(),
            data_source: None,
            guards: None,
            fields: vec![ServiceEntityFieldConfig {
                name: "public_key".to_string(),
                scalar: ScalarOptions::Object,
                required: Some(true),
                list: Some(false),
                fields: Some(vec![
                    ServiceEntityFieldConfig {
                        name: "rp".to_string(),
                        scalar: ScalarOptions::Object,
                        required: Some(true),
                        list: Some(false),
                        fields: Some(vec![
                            ServiceEntityFieldConfig {
                                name: "id".to_string(),
                                scalar: ScalarOptions::String,
                                required: Some(true),
                                list: Some(false),
                                fields: None,
                                guards: None,
                                exclude_from_input: None,
                                exclude_from_output: None,
                                as_type: None,
                                join_on: None,
                            },
                            ServiceEntityFieldConfig {
                                name: "name".to_string(),
                                scalar: ScalarOptions::String,
                                required: Some(true),
                                list: Some(false),
                                fields: None,
                                guards: None,
                                exclude_from_input: None,
                                exclude_from_output: None,
                                as_type: None,
                                join_on: None,
                            },
                        ]),
                        guards: None,
                        exclude_from_input: None,
                        exclude_from_output: None,
                        as_type: None,
                        join_on: None,
                    },
                    ServiceEntityFieldConfig {
                        name: "user".to_string(),
                        scalar: ScalarOptions::Object,
                        required: Some(true),
                        list: Some(false),
                        as_type: None,
                        guards: None,
                        exclude_from_input: None,
                        exclude_from_output: None,
                        join_on: None,
                        fields: Some(vec![
                            ServiceEntityFieldConfig {
                                name: "id".to_string(),
                                scalar: ScalarOptions::String,
                                required: Some(true),
                                list: Some(false),
                                fields: None,
                                guards: None,
                                exclude_from_input: None,
                                exclude_from_output: None,
                                as_type: None,
                                join_on: None,
                            },
                            ServiceEntityFieldConfig {
                                name: "name".to_string(),
                                scalar: ScalarOptions::String,
                                required: Some(true),
                                list: Some(false),
                                fields: None,
                                guards: None,
                                exclude_from_input: None,
                                exclude_from_output: None,
                                as_type: None,
                                join_on: None,
                            },
                            ServiceEntityFieldConfig {
                                name: "displayName".to_string(),
                                scalar: ScalarOptions::String,
                                required: Some(true),
                                list: Some(false),
                                fields: None,
                                guards: None,
                                exclude_from_input: None,
                                exclude_from_output: None,
                                as_type: None,
                                join_on: None,
                            },
                        ]),
                    },
                    ServiceEntityFieldConfig {
                        name: "challenge".to_string(),
                        scalar: ScalarOptions::String,
                        required: Some(true),
                        list: Some(false),
                        fields: None,
                        guards: None,
                        exclude_from_input: None,
                        exclude_from_output: None,
                        as_type: None,
                        join_on: None,
                    },
                    ServiceEntityFieldConfig {
                        name: "pubKeyCredParams".to_string(),
                        scalar: ScalarOptions::Object,
                        required: Some(true),
                        list: Some(true),
                        fields: Some(vec![
                            ServiceEntityFieldConfig {
                                name: "alg".to_string(),
                                scalar: ScalarOptions::Int,
                                required: Some(true),
                                list: Some(false),
                                fields: None,
                                guards: None,
                                exclude_from_input: None,
                                exclude_from_output: None,
                                as_type: None,
                                join_on: None,
                            },
                            ServiceEntityFieldConfig {
                                name: "type".to_string(),
                                scalar: ScalarOptions::String,
                                required: Some(true),
                                list: Some(false),
                                fields: None,
                                guards: None,
                                exclude_from_input: None,
                                exclude_from_output: None,
                                as_type: None,
                                join_on: None,
                            },
                        ]),
                        guards: None,
                        exclude_from_input: None,
                        exclude_from_output: None,
                        as_type: None,
                        join_on: None,
                    },
                    ServiceEntityFieldConfig {
                        name: "timeout".to_string(),
                        scalar: ScalarOptions::Int,
                        required: Some(true),
                        list: Some(false),
                        fields: None,
                        guards: None,
                        exclude_from_input: None,
                        exclude_from_output: None,
                        as_type: None,
                        join_on: None,
                    },
                    ServiceEntityFieldConfig {
                        name: "attestation".to_string(),
                        scalar: ScalarOptions::String,
                        required: Some(true),
                        list: Some(false),
                        fields: None,
                        guards: None,
                        exclude_from_input: None,
                        exclude_from_output: None,
                        as_type: None,
                        join_on: None,
                    },
                    ServiceEntityFieldConfig {
                        name: "authenticatorSelection".to_string(),
                        scalar: ScalarOptions::Object,
                        required: Some(true),
                        list: Some(false),
                        guards: None,
                        exclude_from_input: None,
                        exclude_from_output: None,
                        as_type: None,
                        join_on: None,
                        fields: Some(vec![
                            ServiceEntityFieldConfig {
                                name: "requireResidentKey".to_string(),
                                scalar: ScalarOptions::Boolean,
                                required: Some(true),
                                list: Some(false),
                                fields: None,
                                guards: None,
                                exclude_from_input: None,
                                exclude_from_output: None,
                                as_type: None,
                                join_on: None,
                            },
                            ServiceEntityFieldConfig {
                                name: "userVerification".to_string(),
                                scalar: ScalarOptions::String,
                                required: Some(true),
                                list: Some(false),
                                fields: None,
                                guards: None,
                                exclude_from_input: None,
                                exclude_from_output: None,
                                as_type: None,
                                join_on: None,
                            },
                        ]),
                    },
                    ServiceEntityFieldConfig {
                        name: "extensions".to_string(),
                        scalar: ScalarOptions::Object,
                        required: Some(true),
                        list: Some(false),
                        guards: None,
                        exclude_from_input: None,
                        exclude_from_output: None,
                        as_type: None,
                        join_on: None,
                        fields: Some(vec![
                            ServiceEntityFieldConfig {
                                name: "uvm".to_string(),
                                scalar: ScalarOptions::Boolean,
                                required: Some(true),
                                list: Some(false),
                                fields: None,
                                guards: None,
                                exclude_from_input: None,
                                exclude_from_output: None,
                                as_type: None,
                                join_on: None,
                            },
                            ServiceEntityFieldConfig {
                                name: "credProps".to_string(),
                                scalar: ScalarOptions::Boolean,
                                required: Some(true),
                                list: Some(false),
                                fields: None,
                                guards: None,
                                exclude_from_input: None,
                                exclude_from_output: None,
                                as_type: None,
                                join_on: None,
                            },
                        ]),
                    },
                ]),
                guards: None,
                exclude_from_input: None,
                exclude_from_output: None,
                as_type: None,
                join_on: None,
            }],
        }
    }
}
