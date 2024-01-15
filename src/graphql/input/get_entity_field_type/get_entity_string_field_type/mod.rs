use async_graphql::dynamic::TypeRef;
use log::debug;

use crate::{graphql::input::ServiceInput, resolver_type::ResolverType};

impl ServiceInput {
    pub fn get_entity_string_field_type(
        resolver_type: &ResolverType,
        is_list: bool,
        is_required: bool,
    ) -> TypeRef {
        debug!(
            "get_entity_string_field_type: resolver_type: {:?}, is_list: {}, is_required: {}",
            resolver_type, is_list, is_required
        );
        match resolver_type {
            ResolverType::FindOne
            | ResolverType::FindMany
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => {
                if is_list {
                    TypeRef::named_nn_list(TypeRef::STRING)
                } else {
                    TypeRef::named(TypeRef::STRING)
                }
            }
            ResolverType::CreateOne => match is_required {
                true => {
                    if is_list {
                        TypeRef::named_nn_list(TypeRef::STRING)
                    } else {
                        TypeRef::named_nn(TypeRef::STRING)
                    }
                }
                _ => {
                    if is_list {
                        TypeRef::named_nn_list(TypeRef::STRING)
                    } else {
                        TypeRef::named(TypeRef::STRING)
                    }
                }
            },
            _ => panic!("Invalid resolver type: {:?}", resolver_type),
        }
    }
}
