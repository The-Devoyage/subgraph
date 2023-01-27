use async_graphql::dynamic::{Field, InputValue, TypeRef};

use crate::{
    configuration::subgraph::{ScalarOptions, ServiceEntity},
    graphql::schema::{ResolverType, ServiceSchema},
};

impl ServiceSchema {
    pub fn generate_resolver_input_value(
        entity: &ServiceEntity,
        mut field: Field,
        resolver_type: &ResolverType,
    ) -> Field {
        for entity_field in &entity.fields {
            let field_type = match &entity_field.scalar {
                ScalarOptions::String => match resolver_type {
                    ResolverType::FindOne | ResolverType::CreateOne => {
                        match entity_field.required {
                            true => TypeRef::named_nn(TypeRef::STRING),
                            false => TypeRef::named(TypeRef::STRING),
                        }
                    }
                },
                ScalarOptions::Int => match resolver_type {
                    ResolverType::FindOne | ResolverType::CreateOne => {
                        match entity_field.required {
                            true => TypeRef::named_nn(TypeRef::INT),
                            false => TypeRef::named(TypeRef::INT),
                        }
                    }
                },
                ScalarOptions::Boolean => match resolver_type {
                    ResolverType::FindOne | ResolverType::CreateOne => {
                        match entity_field.required {
                            true => TypeRef::named_nn(TypeRef::BOOLEAN),
                            false => TypeRef::named(TypeRef::BOOLEAN),
                        }
                    }
                },
            };
            field = field.argument(InputValue::new(&entity_field.name, field_type));
        }
        field
    }
}
