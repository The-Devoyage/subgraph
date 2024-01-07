use async_graphql::dynamic::Object;
use log::debug;

use crate::{
    configuration::subgraph::{
        entities::{service_entity_field::ServiceEntityFieldConfig, ServiceEntityConfig},
        SubGraphConfig,
    },
    data_sources::{DataSource, DataSources},
};

mod create_as_type_field;
mod create_field;
pub mod create_return_types;
mod get_field_type_ref;

pub struct ServiceEntity {
    data_sources: DataSources,
    data_source: DataSource,
    entity: ServiceEntityConfig,
    type_name: String,
    fields: Vec<ServiceEntityFieldConfig>,
    subgraph_config: SubGraphConfig,
    is_root: bool,
}

impl ServiceEntity {
    pub fn new(
        data_sources: DataSources,
        entity: ServiceEntityConfig,
        type_name: String,
        fields: Vec<ServiceEntityFieldConfig>,
        subgraph_config: SubGraphConfig,
        is_root: Option<bool>,
    ) -> Self {
        let data_source = DataSources::get_entity_data_soruce(&data_sources, &entity);
        Self {
            data_sources: data_sources.clone(),
            data_source: data_source.clone(),
            entity,
            type_name,
            fields,
            subgraph_config,
            is_root: is_root.unwrap_or(true),
        }
    }

    pub fn build(self) -> Vec<Object> {
        debug!("Creating Type For: `{}`", &self.type_name);

        let mut type_defs = Vec::new();

        let mut type_def = Object::new(&self.type_name);

        // Before processing the fields, we need to create the return types
        let return_types = self.create_return_types();
        for return_type in return_types {
            type_defs.push(return_type);
        }

        for entity_field in &self.fields {
            if entity_field.exclude_from_output.unwrap_or(false) {
                continue;
            }

            let type_defs_and_refs = self.get_field_type_ref(&entity_field);

            for object_type_def in type_defs_and_refs.type_defs {
                type_defs.push(object_type_def);
            }

            if entity_field.as_type.is_some() {
                let as_type_entity = match self.create_as_type_entity(entity_field) {
                    Ok(as_type_entity) => as_type_entity,
                    Err(_) => continue,
                };
                type_def = type_def.field(as_type_entity);
            } else {
                let field = ServiceEntity::create_field(
                    entity_field.clone(),
                    type_defs_and_refs.type_ref,
                    self.data_source.clone(),
                    self.is_root,
                    self.entity.required.unwrap_or(false),
                );
                type_def = type_def.field(field);
            }
        }

        type_defs.push(type_def);

        debug!("Created Type Defs: {:?}", type_defs);

        type_defs
    }
}
