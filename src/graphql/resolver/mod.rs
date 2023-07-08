use async_graphql::dynamic::Field;
use log::debug;

use crate::configuration::subgraph::{
    entities::{service_entity_field::ServiceEntityField, ServiceEntity},
    SubGraphConfig,
};

use super::schema::ResolverType;

mod create_resolver_function;
mod create_resolver_name;
mod get_resolver_type_ref;

#[derive(Debug)]
pub struct ServiceResolver {
    subgraph_config: SubGraphConfig,
    resolver_type: ResolverType,
    entity: ServiceEntity,
    as_field: Option<ServiceEntityField>,
}

impl ServiceResolver {
    pub fn new(
        subgraph_config: SubGraphConfig,
        resolver_type: ResolverType,
        entity: ServiceEntity,
        as_field: Option<ServiceEntityField>,
    ) -> Self {
        debug!("Creating Service Resolver Builder");
        Self {
            subgraph_config,
            resolver_type,
            entity,
            as_field,
        }
    }

    pub fn build(self) -> Field {
        debug!("Creating Service Resolver");

        let resolver = Field::new(
            self.create_resolver_name(),
            self.get_resolver_type_ref(),
            self.create_resolver_function(),
        );

        debug!("Created Service Resolver: {:?}", resolver);
        resolver
    }
}
