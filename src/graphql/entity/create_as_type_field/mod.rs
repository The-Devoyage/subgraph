use async_graphql::dynamic::{Field, InputValue, TypeRef};
use log::debug;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    graphql::{resolver::ServiceResolver, schema::ResolverType},
};

use super::ServiceEntity;

impl ServiceEntity {
    pub fn create_as_type_entity(
        &self,
        entity_field: &ServiceEntityFieldConfig,
    ) -> Result<Field, async_graphql::Error> {
        debug!("Creating As Type Resolver For: {:?}", entity_field);

        let list = entity_field.list.unwrap_or(false);
        let as_type_entity = self
            .subgraph_config
            .service
            .entities
            .iter()
            .find(|e| e.name == entity_field.clone().as_type.unwrap());

        let as_type_entity = match as_type_entity {
            Some(entity) => entity,
            None => panic!("Could not find entity for as_type resolver"),
        };

        let as_type_resolver = ServiceResolver::new(
            self.subgraph_config.clone(),
            ResolverType::InternalType,
            as_type_entity.clone(),
            Some(entity_field.clone()),
        )
        .build();

        let resolver_input_name = ServiceResolver::get_resolver_input_name(
            &as_type_entity.name,
            &ResolverType::InternalType,
            Some(list),
        );

        let field = as_type_resolver.argument(InputValue::new(
            format!("{}", entity_field.name),
            TypeRef::named_nn(resolver_input_name),
        ));

        Ok(field)
    }
}
