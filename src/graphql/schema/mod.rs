use async_graphql::dynamic::{Object, Schema, TypeRef};
use log::{debug, error, info};

use crate::{configuration::subgraph::SubGraphConfig, database::data_source::DataSource};

mod generate_entity;
mod generate_resolver;
mod generate_type;

#[derive(Clone, Copy)]
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

pub struct ServiceSchema;

impl ServiceSchema {
    pub fn generate_schema(subgraph_config: SubGraphConfig, data_source: DataSource) -> Schema {
        info!("Generating Schema From Sub Graph Config");
        info!("Generating Query Object");
        let query = Object::new("Query").extends();
        debug!("{:?}", query);
        info!("Building Schema Started");
        let schema = Schema::build("Query", None, None)
            .data(data_source)
            .enable_federation();

        let (query, schema) = ServiceSchema::generate_entity(subgraph_config, schema, query);

        info!("Register Query and Finish Building Schema.");
        let schema = schema.register(query).finish();
        debug!("{:?}", schema);
        let schema = match schema {
            Ok(sch) => sch,
            Err(err) => {
                error!("{}", err);
                panic!("Failed to build schema.")
            }
        };
        schema
    }
}
