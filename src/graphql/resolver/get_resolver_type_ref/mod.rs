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

        // Here we will create the name of the return type.
        // The return type is made elsewhere but referenced here.
        let type_ref = match self.resolver_type {
            ResolverType::InternalType => match list {
                true => {
                    let return_type_name = format!(
                        "{}_{}_response",
                        ResolverType::FindMany.to_string().to_lowercase(),
                        self.entity.name
                    );
                    TypeRef::named_nn(&return_type_name)
                }
                false => {
                    let return_type_name = format!(
                        "{}_{}_response",
                        ResolverType::FindOne.to_string().to_lowercase(),
                        self.entity.name
                    );
                    TypeRef::named_nn(&return_type_name)
                }
            },
            _ => {
                let return_type_name = format!(
                    "{}_{}_response",
                    self.resolver_type.to_string().to_lowercase(),
                    self.entity.name
                );
                TypeRef::named_nn(&return_type_name)
            }
        };

        debug!("Resolver Type Ref: {:?}", type_ref);

        type_ref
    }
}
