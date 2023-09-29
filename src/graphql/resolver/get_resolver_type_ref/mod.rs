use async_graphql::dynamic::TypeRef;
use log::debug;

use crate::graphql::schema::ResolverType;

use super::ServiceResolver;

impl ServiceResolver {
    pub fn get_resolver_type_ref(&self) -> TypeRef {
        debug!("Getting Resolver Type Ref");

        let list = match &self.as_field {
            Some(field) => field.list,
            None => None,
        };

        let list = list.unwrap_or(false);

        let type_ref = match self.resolver_type {
            ResolverType::FindOne => TypeRef::named_nn(&self.entity.name),
            ResolverType::CreateOne => TypeRef::named_nn(&self.entity.name),
            ResolverType::FindMany => TypeRef::named_nn_list_nn(&self.entity.name),
            ResolverType::UpdateOne => TypeRef::named_nn(&self.entity.name),
            ResolverType::UpdateMany => TypeRef::named_nn_list_nn(&self.entity.name),
            ResolverType::InternalType => match list {
                true => TypeRef::named_nn_list_nn(&self.entity.name),
                false => TypeRef::named_nn(&self.entity.name),
            },
        };

        debug!("Resolver Type Ref: {:?}", type_ref);

        type_ref
    }
}
