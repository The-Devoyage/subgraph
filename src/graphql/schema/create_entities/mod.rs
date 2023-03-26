use crate::graphql::schema::ResolverType;

use super::ServiceSchemaBuilder;
use log::{debug, info};

mod create_entity_type;
mod create_resolver;

impl ServiceSchemaBuilder {
    pub fn create_entities(mut self) -> Self {
        info!("Creating Entities Config");
        debug!("Entities: {:?}", self.subgraph_config.service.entities);
        let entities = self.subgraph_config.service.entities.clone();

        for entity in entities.iter() {
            self = self.create_entity_type_def(entity);
            self = self.create_resolver(entity, ResolverType::FindOne);
            self = self.create_resolver(entity, ResolverType::FindMany);
            self = self.create_resolver(entity, ResolverType::CreateOne);
            self = self.create_resolver(entity, ResolverType::UpdateOne);
        }

        self
    }
}
