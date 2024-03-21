use async_graphql::dynamic::TypeRef;
use log::{debug, error, trace};

use crate::resolver_type::ResolverType;

use super::ScalarOption;

impl ScalarOption {
    /// Converts a ScalarOption to a TypeRef
    pub fn to_input_type_ref(
        &self,
        list: bool,
        required: bool,
        resolver_type: &ResolverType,
        input_name: Option<&str>,
    ) -> Result<TypeRef, async_graphql::Error> {
        debug!("Creating Input Type Ref");
        let type_ref = match self {
            ScalarOption::String | ScalarOption::UUID | ScalarOption::DateTime => TypeRef::STRING,
            Self::Int => TypeRef::INT,
            Self::Boolean => TypeRef::BOOLEAN,
            Self::ObjectID => "ObjectID",
            Self::Object => {
                if let Some(input_name) = input_name {
                    input_name
                } else {
                    error!(
                        "ScalarOption::to_input_type_ref: Object ScalarOption requires input_name"
                    );
                    return Err(async_graphql::Error::new(
                        "ScalarOption::to_input_type_ref: Object ScalarOption requires input_name",
                    ));
                }
            }
        };

        let type_ref = match resolver_type {
            ResolverType::FindOne
            | ResolverType::FindMany
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => {
                if list {
                    TypeRef::named_nn_list(type_ref)
                } else {
                    TypeRef::named(type_ref)
                }
            }
            ResolverType::CreateOne => {
                let type_ref = if list {
                    match required {
                        true => TypeRef::named_nn_list_nn(type_ref),
                        false => TypeRef::named_nn_list(type_ref),
                    }
                } else {
                    match required {
                        true => TypeRef::named_nn(type_ref),
                        false => TypeRef::named(type_ref),
                    }
                };
                type_ref
            }
            _ => {
                error!("ScalarOption::to_input_type_ref: Unsupported ResolverType");
                return Err(async_graphql::Error::new(
                    "ScalarOption::to_input_type_ref: Unsupported ResolverType",
                ));
            }
        };

        trace!("Input Type Ref: {:?}", type_ref);
        Ok(type_ref)
    }
}
