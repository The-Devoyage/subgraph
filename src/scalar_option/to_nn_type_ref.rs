use super::ScalarOption;
use async_graphql::dynamic::TypeRef;
use log::{debug, trace};

impl ScalarOption {
    pub fn to_nn_type_ref(&self, is_list: bool, name: &str) -> TypeRef {
        debug!("Creating Non-Optional Type Refs");

        let type_ref = match self {
            ScalarOption::String => {
                if is_list {
                    TypeRef::named_nn_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named_nn(TypeRef::STRING)
                }
            }
            ScalarOption::Int => {
                if is_list {
                    TypeRef::named_nn_list_nn(TypeRef::INT)
                } else {
                    TypeRef::named_nn(TypeRef::INT)
                }
            }
            ScalarOption::Boolean => {
                if is_list {
                    TypeRef::named_nn_list_nn(TypeRef::BOOLEAN)
                } else {
                    TypeRef::named_nn(TypeRef::BOOLEAN)
                }
            }
            ScalarOption::ObjectID => {
                if is_list {
                    TypeRef::named_nn_list_nn("ObjectID")
                } else {
                    TypeRef::named_nn("ObjectID")
                }
            }
            ScalarOption::Object => {
                if is_list {
                    TypeRef::named_nn_list_nn(name)
                } else {
                    TypeRef::named_nn(name)
                }
            }
            ScalarOption::UUID => {
                if is_list {
                    TypeRef::named_nn_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named_nn(TypeRef::STRING)
                }
            }
            ScalarOption::DateTime => {
                if is_list {
                    TypeRef::named_nn_list_nn(TypeRef::STRING)
                } else {
                    TypeRef::named_nn(TypeRef::STRING)
                }
            }
        };

        trace!("{:?}", type_ref);
        type_ref
    }
}
