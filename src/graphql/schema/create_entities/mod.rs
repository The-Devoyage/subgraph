use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::{DataSource, DataSources},
    graphql::schema::ResolverType,
};

use super::ServiceSchema;
use log::debug;

mod create_entity_type_defs;
mod create_resolver;

impl ServiceSchema {
    pub fn create_entities(mut self) -> Self {
        debug!("Creating Entities Config");
        debug!("Entities: {:?}", self.subgraph_config.service.entities);
        let entities = self.subgraph_config.service.entities.clone();

        for entity in entities.iter() {
            self = self.create_entity_type_defs(entity);

            let data_source = DataSources::get_entity_data_soruce(&self.data_sources, entity);

            match data_source {
                DataSource::SQL(ds) => match ds.config.dialect {
                    DialectEnum::POSTGRES => {
                        self = self.create_resolver(entity, ResolverType::FindOne);
                        self = self.create_resolver(entity, ResolverType::FindMany);
                        self = self.create_resolver(entity, ResolverType::CreateOne);
                        self = self.create_resolver(entity, ResolverType::UpdateMany);
                    }
                    DialectEnum::MYSQL => {
                        self = self.create_resolver(entity, ResolverType::FindOne);
                        self = self.create_resolver(entity, ResolverType::FindMany);
                        self = self.create_resolver(entity, ResolverType::CreateOne);
                        self = self.create_resolver(entity, ResolverType::UpdateMany);
                        self = self.create_resolver(entity, ResolverType::UpdateOne);
                    }
                    DialectEnum::SQLITE => {
                        self = self.create_resolver(entity, ResolverType::FindOne);
                        self = self.create_resolver(entity, ResolverType::FindMany);
                        self = self.create_resolver(entity, ResolverType::CreateOne);
                        self = self.create_resolver(entity, ResolverType::UpdateMany);
                    }
                },
                DataSource::Mongo(_) | DataSource::HTTP(_) => {
                    self = self.create_resolver(entity, ResolverType::FindOne);
                    self = self.create_resolver(entity, ResolverType::FindMany);
                    self = self.create_resolver(entity, ResolverType::CreateOne);
                    self = self.create_resolver(entity, ResolverType::UpdateOne);
                    self = self.create_resolver(entity, ResolverType::UpdateMany);
                }
            }
        }

        self
    }
}
