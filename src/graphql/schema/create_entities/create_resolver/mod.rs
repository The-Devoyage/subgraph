use log::debug;

use crate::{configuration::subgraph::entities::ServiceEntity, graphql::resolver::ServiceResolver};

use super::{ResolverType, ServiceSchemaBuilder};

mod create_resolver_input_value;

impl ServiceSchemaBuilder {
    pub fn add_resolver(mut self, entity: &ServiceEntity, resolver_type: ResolverType) -> Self {
        debug!("Adding Resolver");

        let resolver = ServiceResolver::new(
            self.subgraph_config.clone(),
            resolver_type,
            entity.clone(),
            None,
        )
        .build();

        self = self.create_resolver_input_value(&entity, resolver, &resolver_type);
        self
    }
}
