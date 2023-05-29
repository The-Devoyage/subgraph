use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::{DataSource, DataSources},
    graphql::schema::ResolverType,
};

use super::ServiceSchemaBuilder;
use log::{debug, info};

mod create_entity_type_defs;
mod create_resolver;

impl ServiceSchemaBuilder {
    pub fn create_entities(mut self) -> Self {
        info!("Creating Entities Config");
        debug!("Entities: {:?}", self.subgraph_config.service.entities);
        let entities = self.subgraph_config.service.entities.clone();

        for entity in entities.iter() {
            self = self.create_entity_type_defs(entity);

            let data_source = DataSources::get_data_source_for_entity(&self.data_sources, entity);
            match data_source {
                DataSource::SQL(ds) => match ds.config.dialect {
                    DialectEnum::POSTGRES => {
                        self = self.add_resolver(entity, ResolverType::FindOne);
                        self = self.add_resolver(entity, ResolverType::FindMany);
                        self = self.add_resolver(entity, ResolverType::CreateOne);
                        self = self.add_resolver(entity, ResolverType::UpdateMany);
                    }
                    DialectEnum::MYSQL => {
                        self = self.add_resolver(entity, ResolverType::FindOne);
                        self = self.add_resolver(entity, ResolverType::FindMany);
                        self = self.add_resolver(entity, ResolverType::CreateOne);
                        self = self.add_resolver(entity, ResolverType::UpdateOne);
                        self = self.add_resolver(entity, ResolverType::UpdateMany);
                    }
                    DialectEnum::SQLITE => {
                        self = self.add_resolver(entity, ResolverType::FindOne);
                        self = self.add_resolver(entity, ResolverType::FindMany);
                        self = self.add_resolver(entity, ResolverType::CreateOne);
                        self = self.add_resolver(entity, ResolverType::UpdateMany);
                    }
                },
                DataSource::Mongo(_) | DataSource::HTTP(_) => {
                    self = self.add_resolver(entity, ResolverType::FindOne);
                    self = self.add_resolver(entity, ResolverType::FindMany);
                    self = self.add_resolver(entity, ResolverType::CreateOne);
                    self = self.add_resolver(entity, ResolverType::UpdateOne);
                    self = self.add_resolver(entity, ResolverType::UpdateMany);
                }
            }
        }

        self
    }
}
