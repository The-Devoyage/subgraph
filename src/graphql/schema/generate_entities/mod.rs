use crate::{
    configuration::subgraph::data_sources::ServiceDataSourceConfig, graphql::schema::ResolverType,
};

use super::ServiceSchema;
use log::info;

mod generate_resolver;

impl ServiceSchema {
    pub fn generate_entities(mut self) -> Self {
        for entity in self.subgraph_config.service.entities.clone() {
            info!("Including Entity, {}, in schema.", &entity.name);

            self = self.add_resolver(&entity, ResolverType::FindOne);
            self = self.add_resolver(&entity, ResolverType::CreateOne);
            self = self.add_resolver(&entity, ResolverType::FindMany);
        }
        self
    }
}
