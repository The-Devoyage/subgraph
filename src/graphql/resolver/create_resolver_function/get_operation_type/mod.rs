use log::debug;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    graphql::resolver::ServiceResolver, resolver_type::ResolverType,
};

impl ServiceResolver {
    pub fn get_operation_type(
        resolver_type: &ResolverType,
        as_field: &Option<ServiceEntityFieldConfig>,
    ) -> ResolverType {
        debug!("Getting Operation Type For Resolver Type");
        match as_field {
            Some(as_field) => {
                if as_field.list.unwrap_or(false) {
                    ResolverType::FindMany
                } else {
                    ResolverType::FindOne
                }
            }
            None => resolver_type.clone(),
        }
    }
}
