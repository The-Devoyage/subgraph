use async_graphql::dynamic::{
    Field, FieldFuture, FieldValue, Object, Scalar, Schema, SchemaBuilder, TypeRef,
};
use biscuit_auth::KeyPair;
use log::{debug, error, trace};

use crate::{configuration::subgraph::SubGraphConfig, data_sources::DataSources};

pub mod create_auth_service;
pub mod create_entities;
pub mod create_options_input;

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
        debug!("Service Schema Initialized");
        service_schema
    }

    pub fn register_health_check(mut self) -> Self {
        debug!("Registering Health Check");
        self.query = self.query.field(Field::new(
            "health_check",
            TypeRef::named_nn(TypeRef::BOOLEAN),
            move |_| FieldFuture::new(async move { Ok(Some(FieldValue::owned_any(true))) }),
        ));
        self.mutation = self.mutation.field(Field::new(
            "create_health_check",
            TypeRef::named_nn(TypeRef::BOOLEAN),
            move |_| FieldFuture::new(async move { Ok(Some(FieldValue::owned_any(true))) }),
        ));
        self
    }

    pub fn build(mut self) -> Schema {
        debug!("Building Schema");

        // Check for key pair and create if needed
        self.get_key_pair();

        // Create shared options input
        self = self.create_options_input();

        // Create entities
        self = self.create_entities();

        // Create Health Check
        self = self.register_health_check();

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

        trace!("{:?}", schema);

        match schema {
            Ok(sch) => sch,
            Err(err) => {
                error!("{}", err);
                panic!("Failed to build schema.")
            }
        }
    }
}
