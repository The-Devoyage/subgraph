use log::debug;

use crate::resolver_type::ResolverType;

use super::ServiceResolver;

impl ServiceResolver {
    pub fn get_resolver_input_name(
        entity_name: &str,
        resolver_type: &ResolverType,
        list: Option<bool>,
    ) -> String {
        debug!("Getting Resolver Input Name For: {}", entity_name);

        let input_name = match resolver_type {
            ResolverType::FindOne => format!("get_{}_input", &entity_name.to_lowercase()),
            ResolverType::CreateOne => format!("create_{}_input", &entity_name.to_lowercase()),
            ResolverType::FindMany => format!("get_{}s_input", &entity_name.to_lowercase()),
            ResolverType::UpdateOne => format!("update_{}_input", &entity_name.to_lowercase()),
            ResolverType::UpdateMany => format!("update_{}s_input", &entity_name.to_lowercase()),
            ResolverType::InternalType => {
                if list.unwrap_or(false) {
                    format!("get_{}s_input", &entity_name.to_lowercase())
                } else {
                    format!("get_{}_input", &entity_name.to_lowercase())
                }
            }
        };

        debug!("Resolver Input Name: {}", input_name);

        input_name
    }
}
