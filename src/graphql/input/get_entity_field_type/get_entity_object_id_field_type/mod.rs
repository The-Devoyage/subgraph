use async_graphql::dynamic::TypeRef;

use crate::graphql::{input::ServiceInput, schema::ResolverType};

impl ServiceInput {
    pub fn get_entity_object_id_field_type(
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
                    TypeRef::named_nn_list("ObjectID")
                } else {
                    TypeRef::named("ObjectID")
                }
            }
            ResolverType::CreateOne => match is_required {
                true => {
                    if is_list {
                        TypeRef::named_nn_list_nn("ObjectID")
                    } else {
                        TypeRef::named_nn("ObjectID")
                    }
                }
                _ => {
                    if is_list {
                        TypeRef::named_nn_list("ObjectID")
                    } else {
                        TypeRef::named("ObjectID")
                    }
                }
            },
            _ => panic!("Invalid resolver type"),
        }
    }
}
