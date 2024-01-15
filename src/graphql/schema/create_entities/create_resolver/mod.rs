use log::debug;

use crate::{
    configuration::subgraph::entities::ServiceEntityConfig, graphql::resolver::ServiceResolver,
};

use super::{ResolverType, ServiceSchema};

mod create_resolver_input_value;

impl ServiceSchema {
    pub fn create_resolver(
        mut self,
        entity: &ServiceEntityConfig,
        resolver_type: ResolverType,
    ) -> Self {
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
