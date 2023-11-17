use async_graphql::dynamic::TypeRef;
use log::debug;

use crate::graphql::schema::ResolverType;

use super::ServiceResolver;

impl ServiceResolver {
    pub fn get_resolver_type_ref(&self) -> TypeRef {
        debug!("Getting Resolver Type Ref");

        // If as_field is Some, it is assumed to be a Internal Join.
        // Use the list prop from the field definition.
        let list = match &self.as_field {
            Some(field) => field.list,
            None => None,
        };

        let list = list.unwrap_or(false);

        // If is internal type, use the field definition to determine if required.
        // Otherwise, use the entity definition.
        let entity_required = match self.resolver_type {
            ResolverType::InternalType => match &self.as_field {
                Some(field) => field.required.unwrap_or(false),
                None => false,
            },
            _ => self.entity.required.unwrap_or(false),
        };

        let type_ref = match self.resolver_type {
            ResolverType::FindOne => match entity_required {
                true => TypeRef::named_nn(&self.entity.name),
                false => TypeRef::named(&self.entity.name),
            },
            ResolverType::CreateOne => match entity_required {
                true => TypeRef::named_nn(&self.entity.name),
                false => TypeRef::named(&self.entity.name),
            },
            ResolverType::FindMany => match entity_required {
                true => TypeRef::named_nn_list_nn(&self.entity.name),
                false => TypeRef::named_list_nn(&self.entity.name),
            },
            ResolverType::UpdateOne => match entity_required {
                true => TypeRef::named_nn(&self.entity.name),
                false => TypeRef::named(&self.entity.name),
            },
            ResolverType::UpdateMany => match entity_required {
                true => TypeRef::named_nn_list_nn(&self.entity.name),
                false => TypeRef::named_list_nn(&self.entity.name),
            },
            ResolverType::InternalType => match list {
                true => match entity_required {
                    true => TypeRef::named_nn_list_nn(&self.entity.name),
                    false => TypeRef::named_list_nn(&self.entity.name),
                },
                false => match entity_required {
                    true => TypeRef::named_nn(&self.entity.name),
                    false => TypeRef::named(&self.entity.name),
                },
            },
        };

        debug!("Resolver Type Ref: {:?}", type_ref);

        type_ref
    }
}
