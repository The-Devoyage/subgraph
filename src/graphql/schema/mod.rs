use async_graphql::dynamic::{InputObject, Object, Schema, SchemaBuilder, TypeRef};
use log::{debug, error, info};

use crate::{configuration::subgraph::SubGraphConfig, database::data_source::DataSource};

mod generate_entities;

#[derive(Clone, Copy, Debug)]
pub enum ResolverType {
    FindOne,
    // FindMany,
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
}

impl ServiceSchema {
    pub fn build(subgraph_config: SubGraphConfig, data_source: DataSource) -> Self {
        ServiceSchema {
            subgraph_config,
            schema_builder: Schema::build("Query", Some("Mutation"), None)
                .data(data_source)
                .enable_federation(),
            query: Object::new("Query").extends(),
            mutation: Object::new("Mutation"),
        }
    }

    pub fn finish(mut self) -> Schema {
        self = self.generate_entities();
        info!("Finishing Schema");

        let schema = self
            .schema_builder
            .register(self.query)
            .register(self.mutation)
            .finish();
        debug!("{:?}", schema);

        let finished = match schema {
            Ok(sch) => sch,
            Err(err) => {
                error!("{}", err);
                panic!("Failed to build schema.")
            }
        };

        finished
    }
}
