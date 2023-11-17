use crate::{
    configuration::subgraph::entities::ServiceEntityConfig, graphql::entity::ServiceEntity,
};

use super::ServiceSchemaBuilder;
use async_graphql::dynamic::{Enum, Object};
use log::debug;

impl ServiceSchemaBuilder {
    pub fn register_types(mut self, type_defs: Vec<Object>) -> Self {
        debug!("Registering Types");
        for type_def in type_defs {
            debug!("Registering Type Def: {:?}", type_def);
            self.schema_builder = self.schema_builder.register(type_def);
        }
        self
    }

    pub fn _register_enums(mut self, enums: Vec<Enum>) -> Self {
        debug!("Registering Enums");
        for enum_def in enums {
            debug!("Registering Enum Def: {:?}", enum_def);
            self.schema_builder = self.schema_builder.register(enum_def);
        }
        self
    }

    pub fn create_entity_type_defs(mut self, entity: &ServiceEntityConfig) -> Self {
        debug!("Creating Types For Entity: {}", &entity.name);
        let entity_type_defs = ServiceEntity::new(
            self.data_sources.clone(),
            entity.clone(),
            entity.name.clone(),
            entity.fields.clone(),
            self.subgraph_config.clone(),
            None,
        )
        .build();

        self = self.register_types(entity_type_defs);

        self
    }
}
