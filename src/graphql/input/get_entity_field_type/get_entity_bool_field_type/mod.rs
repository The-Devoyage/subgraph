use async_graphql::dynamic::TypeRef;

use crate::{graphql::input::ServiceInput, resolver_type::ResolverType};

impl ServiceInput {
    pub fn get_entity_bool_field_type(
        resolver_type: &ResolverType,
        is_list: bool,
        is_required: bool,
    ) -> TypeRef {
        match resolver_type {
            ResolverType::FindOne
            | ResolverType::FindMany
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => {
                if is_list {
                    TypeRef::named_nn_list(TypeRef::BOOLEAN)
                } else {
                    TypeRef::named(TypeRef::BOOLEAN)
                }
            }
            ResolverType::CreateOne => match is_required {
                true => {
                    if is_list {
                        TypeRef::named_nn_list_nn(TypeRef::BOOLEAN)
                    } else {
                        TypeRef::named_nn(TypeRef::BOOLEAN)
                    }
                }
                _ => {
                    if is_list {
                        TypeRef::named_nn_list(TypeRef::BOOLEAN)
                    } else {
                        TypeRef::named(TypeRef::BOOLEAN)
                    }
                }
            },
            _ => panic!("Invalid resolver type: {:?}", resolver_type),
        }
    }
}
