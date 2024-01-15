use async_graphql::dynamic::TypeRef;

use crate::{graphql::input::ServiceInput, resolver_type::ResolverType};

impl ServiceInput {
    pub fn get_entity_int_field_type(
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
                    TypeRef::named_nn_list(TypeRef::INT)
                } else {
                    TypeRef::named(TypeRef::INT)
                }
            }
            ResolverType::CreateOne => match is_required {
                true => {
                    if is_list {
                        TypeRef::named_nn_list_nn(TypeRef::INT)
                    } else {
                        TypeRef::named_nn(TypeRef::INT)
                    }
                }
                _ => {
                    if is_list {
                        TypeRef::named_nn_list(TypeRef::INT)
                    } else {
                        TypeRef::named(TypeRef::INT)
                    }
                }
            },
            _ => panic!("Invalid resolver type: {:?}", resolver_type),
        }
    }
}
