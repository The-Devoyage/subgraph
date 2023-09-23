use async_graphql::dynamic::{Object, Scalar, Schema, SchemaBuilder};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use biscuit_auth::{KeyPair, PrivateKey, PublicKey};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::{configuration::subgraph::SubGraphConfig, data_sources::DataSources};

pub mod create_entities;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum ResolverType {
    FindOne,
    FindMany,
    CreateOne,
    UpdateOne,
    UpdateMany,
    InternalType,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ExcludeFromInput {
    FindOne,
    FindMany,
    CreateOne,
    UpdateOne,
    UpdateMany,
    UpdateOneQuery,
    UpdateManyQuery,
    All,
}

pub struct ServiceSchemaBuilder {
    pub subgraph_config: SubGraphConfig,
    pub schema_builder: SchemaBuilder,
    pub query: Object,
    pub mutation: Object,
    pub data_sources: DataSources,
}

impl ServiceSchemaBuilder {
    pub fn new(subgraph_config: SubGraphConfig, data_sources: DataSources) -> Self {
        info!("Creating Service Schema");

        let key_pair;
        if subgraph_config.service.auth.is_some() {
            let auth = subgraph_config.service.auth.clone().unwrap();
            let b64_private_key = auth.private_key;

            if b64_private_key.is_some() {
                debug!("Using provided key pair");
                let bytes_private_key = &general_purpose::URL_SAFE_NO_PAD
                    .decode(b64_private_key.unwrap())
                    .unwrap();

                let private_key = PrivateKey::from_bytes(bytes_private_key);

                key_pair = Some(KeyPair::from(&private_key.unwrap()));
            } else {
                key_pair = Some(KeyPair::new());
            }
        } else {
            key_pair = None;
        }

        ServiceSchemaBuilder {
            subgraph_config,
            schema_builder: Schema::build("Query", Some("Mutation"), None)
                .data(data_sources.clone())
                .data(key_pair)
                .enable_federation(),
            query: Object::new("Query").extends(),
            mutation: Object::new("Mutation"),
            data_sources,
        }
    }

    pub fn build(mut self) -> Schema {
        info!("Building Schema");

        let object_id = Scalar::new("ObjectID");

        self = self.create_entities();

        let schema = self
            .schema_builder
            .register(object_id)
            .register(self.query)
            .register(self.mutation)
            .finish();

        debug!("Schema Created: {:?}", schema);

        match schema {
            Ok(sch) => sch,
            Err(err) => {
                error!("{}", err);
                panic!("Failed to build schema.")
            }
        }
    }
}
