use crate::configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig;
use async_graphql::dynamic::TypeRef;
use log::{debug, trace};

use super::ScalarOption;

impl ScalarOption {
    pub fn get_nullable_type_ref(&self, entity_field: &ServiceEntityFieldConfig) -> TypeRef {
        debug!("Creating Optional Type Refs");

        let type_ref = match entity_field.scalar.clone() {
            ScalarOption::String => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named(TypeRef::STRING)
                }
            }
            ScalarOption::Int => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(TypeRef::INT)
                } else {
                    TypeRef::named(TypeRef::INT)
                }
            }
            ScalarOption::Boolean => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(TypeRef::BOOLEAN)
                } else {
                    TypeRef::named(TypeRef::BOOLEAN)
                }
            }
            ScalarOption::ObjectID => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn("ObjectID")
                } else {
                    TypeRef::named("ObjectID")
                }
            }
            ScalarOption::Object => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(entity_field.name.clone())
                } else {
                    TypeRef::named(entity_field.name.clone())
                }
            }
            ScalarOption::UUID => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named(TypeRef::STRING)
                }
            }
            ScalarOption::DateTime => {
                if entity_field.list.unwrap_or(false) {
                    TypeRef::named_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named(TypeRef::STRING)
                }
            }
        };
        trace!("{:?}", type_ref);
        type_ref
    }
}
