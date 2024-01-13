use std::fmt::Display;

use async_graphql::dynamic::{Object, Scalar, Schema, SchemaBuilder};
use base64::{engine::general_purpose, Engine as _};
use biscuit_auth::{KeyPair, PrivateKey};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::{configuration::subgraph::SubGraphConfig, data_sources::DataSources};

pub mod create_auth_service;
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
pub struct SortInput {
    pub field: String,
    pub direction: String,
}

impl Display for ResolverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolverType::FindOne => write!(f, "FindOne"),
            ResolverType::FindMany => write!(f, "FindMany"),
            ResolverType::CreateOne => write!(f, "CreateOne"),
            ResolverType::UpdateOne => write!(f, "UpdateOne"),
            ResolverType::UpdateMany => write!(f, "UpdateMany"),
            ResolverType::InternalType => write!(f, "InternalType"),
        }
    }
}

impl ResolverType {
    pub fn get_resolver_types() -> Vec<ResolverType> {
        vec![
            ResolverType::FindOne,
            ResolverType::FindMany,
            ResolverType::CreateOne,
            ResolverType::UpdateOne,
            ResolverType::UpdateMany,
        ]
    }
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
        debug!("Creating Service Schema");

        let key_pair;
        if subgraph_config.service.auth.is_some() {
            //info message with an unicode icon
            info!("🔐 Auth Enabled!");
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
        debug!("Building Schema");

        let object_id = Scalar::new("ObjectID");

        self = self.create_entities();

        if self.subgraph_config.service.auth.is_some() {
            self = self.create_auth_service();
        }

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
