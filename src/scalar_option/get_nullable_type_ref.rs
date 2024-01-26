use async_graphql::dynamic::TypeRef;
use log::{debug, trace};

use super::ScalarOption;

impl ScalarOption {
    pub fn to_nullable_type_ref(&self, is_list: bool, name: Option<&str>) -> TypeRef {
        debug!("Creating Optional Type Refs");

        let type_ref = match self {
            ScalarOption::String => {
                if is_list {
                    TypeRef::named_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named(TypeRef::STRING)
                }
            }
            ScalarOption::Int => {
                if is_list {
                    TypeRef::named_list_nn(TypeRef::INT)
                } else {
                    TypeRef::named(TypeRef::INT)
                }
            }
            ScalarOption::Boolean => {
                if is_list {
                    TypeRef::named_list_nn(TypeRef::BOOLEAN)
                } else {
                    TypeRef::named(TypeRef::BOOLEAN)
                }
            }
            ScalarOption::ObjectID => {
                if is_list {
                    TypeRef::named_list_nn("ObjectID")
                } else {
                    TypeRef::named("ObjectID")
                }
            }
            ScalarOption::Object => {
                if is_list {
                    TypeRef::named_list_nn(name.unwrap())
                } else {
                    TypeRef::named(name.unwrap())
                }
            }
            ScalarOption::UUID => {
                if is_list {
                    TypeRef::named_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named(TypeRef::STRING)
                }
            }
            ScalarOption::DateTime => {
                if is_list {
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
