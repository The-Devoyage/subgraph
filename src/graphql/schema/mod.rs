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

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum DirectionEnum {
    Asc,
    Desc,
}

//Implement disaplay
impl Display for DirectionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionEnum::Asc => write!(f, "Asc"),
            DirectionEnum::Desc => write!(f, "Desc"),
        }
    }
}

impl std::str::FromStr for DirectionEnum {
    type Err = async_graphql::Error;

    fn from_str(input: &str) -> Result<DirectionEnum, Self::Err> {
        match input {
            "ASC" | "Asc" => Ok(DirectionEnum::Asc),
            "Desc" | "DESC" => Ok(DirectionEnum::Desc),
            _ => Err(async_graphql::Error::new(format!(
                "Invalid DirectionEnum: {}",
                input
            ))),
        }
    }
}

// Accept ASC Asc DESC Desc - Deserializer for DirectionEnum
impl<'de> serde::Deserialize<'de> for DirectionEnum {
    fn deserialize<D>(deserializer: D) -> Result<DirectionEnum, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "ASC" | "Asc" => Ok(DirectionEnum::Asc),
            "Desc" | "DESC" => Ok(DirectionEnum::Desc),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid DirectionEnum: {}",
                s
            ))),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SortInput {
    pub field: String,
    pub direction: DirectionEnum,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OptionsInput {
    pub per_page: Option<i32>,
    pub page: Option<i32>,
    pub sort: Option<Vec<SortInput>>,
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

pub struct ServiceSchema {
    pub subgraph_config: SubGraphConfig,
    pub schema_builder: SchemaBuilder,
    pub query: Object,
    pub mutation: Object,
    pub data_sources: DataSources,
}

impl ServiceSchema {
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

        ServiceSchema {
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
