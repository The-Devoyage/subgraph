use async_graphql::dynamic::{Object, Scalar, Schema, SchemaBuilder, TypeRef};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::{configuration::subgraph::SubGraphConfig, data_sources::DataSources};

mod generate_entities;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum ResolverType {
    FindOne,
    FindMany,
    CreateOne,
    // CreateMany,
    // DeleteOne,
    // DeleteMany,
    // UpdateOne,
    // UpdateMany,
}

#[derive(Debug)]
pub struct ResolverConfig {
    resolver_name: String,
    return_type: TypeRef,
}

pub struct ServiceSchema {
    pub subgraph_config: SubGraphConfig,
    pub schema_builder: SchemaBuilder,
    pub query: Object,
    pub mutation: Object,
    pub data_sources: DataSources,
}

impl ServiceSchema {
    pub fn build(subgraph_config: SubGraphConfig, data_sources: DataSources) -> Self {
        ServiceSchema {
            subgraph_config,
            schema_builder: Schema::build("Query", Some("Mutation"), None)
                .data(data_sources.clone())
                .enable_federation(),
            query: Object::new("Query").extends(),
            mutation: Object::new("Mutation"),
            data_sources,
        }
    }

    pub fn finish(mut self) -> Schema {
        info!("Finishing Schema");

        let object_id = Scalar::new("ObjectID");

        self = self.generate_entities();

        let schema_result = self
            .schema_builder
            .register(object_id)
            .register(self.query)
            .register(self.mutation)
            .finish();

        debug!("{:?}", schema_result);

        let finished = match schema_result {
            Ok(sch) => sch,
            Err(err) => {
                error!("{}", err);
                panic!("Failed to build schema.")
            }
        };

        finished
    }
}
