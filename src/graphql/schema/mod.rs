use std::fmt::Display;

use async_graphql::dynamic::{Object, Scalar, Schema, SchemaBuilder};
use biscuit_auth::KeyPair;
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};

use crate::{configuration::subgraph::SubGraphConfig, data_sources::DataSources};

pub mod create_auth_service;
pub mod create_entities;
pub mod create_options_input;

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
    pub key_pair: Option<KeyPair>,
}

impl ServiceSchema {
    pub fn new(subgraph_config: SubGraphConfig, data_sources: DataSources) -> Self {
        debug!("Creating Service Schema");

        let service_schema = ServiceSchema {
            subgraph_config,
            schema_builder: Schema::build("Query", Some("Mutation"), None),
            query: Object::new("Query").extends(),
            mutation: Object::new("Mutation"),
            data_sources,
            key_pair: None,
        };

        service_schema
    }

    pub fn build(mut self) -> Schema {
        debug!("Building Schema");

        // Check for key pair and create if needed
        self.get_key_pair();

        // Create shared options input
        self = self.create_options_input();

        // Create entities
        self = self.create_entities();

        // Create auth service
        if self.subgraph_config.service.auth.is_some() {
            self = self.create_auth_service();
        }

        // List scalars
        let object_id = Scalar::new("ObjectID");

        // Register Query and Mutation
        let schema = self
            .schema_builder
            .data(self.data_sources.clone())
            .data(self.key_pair)
            .enable_federation()
            .register(object_id)
            .register(self.query)
            .register(self.mutation)
            .finish();

        trace!("Schema Created: {:?}", schema);

        match schema {
            Ok(sch) => sch,
            Err(err) => {
                error!("{}", err);
                panic!("Failed to build schema.")
            }
        }
    }
}
