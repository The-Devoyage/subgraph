use log::debug;

use crate::resolver_type::ResolverType;

use super::ServiceResolver;

impl ServiceResolver {
    pub fn create_resolver_name(&self) -> String {
        debug!("Creating Resolver Name");

        let base = if let Some(field_name) = &self.as_field {
            &field_name.name
        } else {
            &self.entity.name
        };

        let resolver_name = match &self.resolver_type {
            ResolverType::FindOne => format!("get_{}", base.to_lowercase()),
            ResolverType::CreateOne => format!("create_{}", base.to_lowercase()),
            ResolverType::FindMany => format!("get_{}s", base.to_lowercase()),
            ResolverType::UpdateOne => format!("update_{}", base.to_lowercase()),
            ResolverType::UpdateMany => format!("update_{}s", base.to_lowercase()),
            ResolverType::InternalType => format!("{}", base.to_lowercase()),
        };

        debug!("Resolver Name: {}", resolver_name);

        resolver_name
    }
}
