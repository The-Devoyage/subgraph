use log::debug;

use crate::{
    configuration::subgraph::entities::{
        service_entity_field::ServiceEntityFieldConfig, ServiceEntityConfig,
    },
    data_sources::DataSources,
    graphql::entity::ServiceEntity,
    scalar_option::ScalarOption,
};

use super::TypeRefsAndDefs;

impl ServiceEntity {
    pub fn create_optional_type_refs(
        &self,
        entity: &ServiceEntityConfig,
        entity_field: &ServiceEntityFieldConfig,
        data_sources: &DataSources,
    ) -> TypeRefsAndDefs {
        debug!("Creating Optional Type Refs");
        let mut type_defs = Vec::new();

        let type_ref = entity_field
            .scalar
            .to_nullable_type_ref(entity_field.list.unwrap_or(false), &entity_field.name);

        match entity_field.scalar.clone() {
            ScalarOption::Object => {
                let object_type_defs = ServiceEntity::new(
                    data_sources.clone(),
                    entity.clone(),
                    entity_field.name.clone(),
                    entity_field.fields.clone().unwrap_or(vec![]),
                    self.subgraph_config.clone(),
                    Some(false),
                )
                .build();

                for object in object_type_defs {
                    type_defs.push(object)
                }
            }
            _ => {}
        };

        TypeRefsAndDefs {
            type_ref,
            type_defs,
        }
    }
}
