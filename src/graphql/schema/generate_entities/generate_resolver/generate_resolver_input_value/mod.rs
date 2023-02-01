use crate::{
    configuration::subgraph::{ScalarOptions, ServiceEntity, ServiceEntityFieldOptions},
    graphql::schema::{ResolverType, ServiceSchema},
};
use async_graphql::dynamic::{Field, InputObject, InputValue, TypeRef};
use log::{debug, info};

impl ServiceSchema {
    pub fn get_entity_field_type(
        entity_field: &ServiceEntityFieldOptions,
        resolver_type: &ResolverType,
    ) -> TypeRef {
        let field_type = match &entity_field.scalar {
            ScalarOptions::String => match resolver_type {
                ResolverType::FindOne | ResolverType::FindMany => TypeRef::named(TypeRef::STRING),
                ResolverType::CreateOne => match entity_field.required {
                    true => TypeRef::named_nn(TypeRef::STRING),
                    false => TypeRef::named(TypeRef::STRING),
                },
            },
            ScalarOptions::Int => match resolver_type {
                ResolverType::FindOne | ResolverType::FindMany => TypeRef::named(TypeRef::INT),
                ResolverType::CreateOne => match entity_field.required {
                    true => TypeRef::named_nn(TypeRef::INT),
                    false => TypeRef::named(TypeRef::INT),
                },
            },
            ScalarOptions::Boolean => match resolver_type {
                ResolverType::FindOne | ResolverType::FindMany => TypeRef::named(TypeRef::BOOLEAN),
                ResolverType::CreateOne => match entity_field.required {
                    true => TypeRef::named_nn(TypeRef::BOOLEAN),
                    false => TypeRef::named(TypeRef::BOOLEAN),
                },
            },
        };

        field_type
    }

    pub fn generate_resolver_input_value(
        mut self,
        entity: &ServiceEntity,
        mut field: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        info!("Generate Resolver Input Value");

        match resolver_type {
            ResolverType::FindOne => {
                let resolver_input_name = format!("get_{}_input", &entity.name.to_lowercase());
                info!("Adding Find One Resolver, {}", resolver_input_name);

                let mut input = InputObject::new(&resolver_input_name);

                for entity_field in &entity.fields {
                    info!(
                        "Getting {} Field Type For Find One Resolver",
                        resolver_input_name
                    );
                    let field_type =
                        ServiceSchema::get_entity_field_type(&entity_field, &resolver_type);
                    debug!("Field Type: {:?}", field_type);
                    input = input.field(InputValue::new(&entity_field.name, field_type));
                }
                field = field.argument(InputValue::new(
                    &resolver_input_name,
                    TypeRef::named_nn(input.type_name()),
                ));

                info!("Created Field, {:?}", resolver_input_name);
                debug!("{:?}", field);
                self.query = self.query.field(field);

                info!("Updated Query");
                debug!("{:?}", self.query);

                self.schema_builder = self.schema_builder.register(input);
            }
            ResolverType::FindMany => {
                let resolver_input_name = format!("get_{}s_input", &entity.name.to_lowercase());
                info!("Adding Find Many Resolver, {}", resolver_input_name);

                let mut input = InputObject::new(&resolver_input_name);

                for entity_field in &entity.fields {
                    info!(
                        "Getting {}, Field Type For Find Many Resolver",
                        resolver_input_name
                    );

                    let field_type =
                        ServiceSchema::get_entity_field_type(&entity_field, &resolver_type);
                    debug!("{:?}", field_type);

                    input = input.field(InputValue::new(&entity_field.name, field_type));
                }
                field = field.argument(InputValue::new(
                    &resolver_input_name,
                    TypeRef::named_nn(input.type_name()),
                ));

                info!("Created Field, {:?}", resolver_input_name);
                debug!("{:?}", field);

                self.query = self.query.field(field);

                info!("Updated Query");
                debug!("{:?}", self.query);

                self.schema_builder = self.schema_builder.register(input);
            }
            ResolverType::CreateOne => {
                let resolver_input_name = format!("create_{}_input", &entity.name.to_lowercase());
                info!("Adding Create One Resolver, {}", resolver_input_name);
                let mut input = InputObject::new(&resolver_input_name);
                for entity_field in &entity.fields {
                    info!(
                        "Getting {} Field Type for Create One Resolver",
                        resolver_input_name
                    );
                    let field_type =
                        ServiceSchema::get_entity_field_type(&entity_field, &resolver_type);
                    input = input.field(InputValue::new(&entity_field.name, field_type));
                }
                field = field.argument(InputValue::new(
                    &resolver_input_name,
                    TypeRef::named_nn(input.type_name()),
                ));
                info!("Created Field, {:?}", resolver_input_name);
                debug!("{:?}", field);
                self.mutation = self.mutation.field(field);
                info!("Updated Mutation");
                debug!("{:?}", self.mutation);
                self.schema_builder = self.schema_builder.register(input);
            }
        }
        self
    }
}
