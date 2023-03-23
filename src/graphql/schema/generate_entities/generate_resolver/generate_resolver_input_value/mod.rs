use crate::{
    configuration::subgraph::entities::{ScalarOptions, ServiceEntity, ServiceEntityFieldOptions},
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
                ResolverType::FindOne | ResolverType::FindMany | ResolverType::UpdateOne => {
                    TypeRef::named(TypeRef::STRING)
                }
                ResolverType::CreateOne => match entity_field.required {
                    true => TypeRef::named_nn(TypeRef::STRING),
                    false => TypeRef::named(TypeRef::STRING),
                },
            },
            ScalarOptions::Int => match resolver_type {
                ResolverType::FindOne | ResolverType::FindMany | ResolverType::UpdateOne => {
                    TypeRef::named(TypeRef::INT)
                }
                ResolverType::CreateOne => match entity_field.required {
                    true => TypeRef::named_nn(TypeRef::INT),
                    false => TypeRef::named(TypeRef::INT),
                },
            },
            ScalarOptions::Boolean => match resolver_type {
                ResolverType::FindOne | ResolverType::FindMany | ResolverType::UpdateOne => {
                    TypeRef::named(TypeRef::BOOLEAN)
                }
                ResolverType::CreateOne => match entity_field.required {
                    true => TypeRef::named_nn(TypeRef::BOOLEAN),
                    false => TypeRef::named(TypeRef::BOOLEAN),
                },
            },
            ScalarOptions::ObjectID => match resolver_type {
                ResolverType::FindOne | ResolverType::FindMany | ResolverType::UpdateOne => {
                    TypeRef::named("ObjectID")
                }
                ResolverType::CreateOne => match entity_field.required {
                    true => TypeRef::named_nn("ObjectID"),
                    false => TypeRef::named("ObjectID"),
                },
            },
        };

        field_type
    }

    pub fn exclude_field_from_input(
        entity_field: &ServiceEntityFieldOptions,
        resolver_type: &ResolverType,
    ) -> bool {
        info!("Checking If Field Should Be Excluded From Input");
        let exclude_from_input = entity_field.exclude_from_input.clone();
        debug!("Exclude From Input Config: {:?}", exclude_from_input);
        let mut exclude = false;
        if exclude_from_input.is_some() {
            if exclude_from_input.unwrap().contains(&resolver_type) {
                exclude = true;
            }
        }
        debug!("Exclude {}: {}", entity_field.name, exclude);
        exclude
    }

    pub fn generate_find_one_input(
        mut self,
        entity: &ServiceEntity,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        let resolver_input_name = format!("get_{}_input", &entity.name.to_lowercase());
        info!("Creating Find One Resolver: {}", resolver_input_name);

        let mut input = InputObject::new(&resolver_input_name);

        for entity_field in &entity.fields {
            if !ServiceSchema::exclude_field_from_input(&entity_field, resolver_type) {
                let field_type = ServiceSchema::get_entity_field_type(&entity_field, resolver_type);
                input = input.field(InputValue::new(&entity_field.name, field_type));
            }
        }

        resolver = resolver.argument(InputValue::new(
            &resolver_input_name,
            TypeRef::named_nn(input.type_name()),
        ));

        self.query = self.query.field(resolver);
        self.schema_builder = self.schema_builder.register(input);
        self
    }

    pub fn generate_find_many_input(
        mut self,
        entity: &ServiceEntity,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        let resolver_input_name = format!("get_{}s_input", &entity.name.to_lowercase());
        info!("Creating Find Many Resolver: {}", resolver_input_name);

        let mut input = InputObject::new(&resolver_input_name);

        for entity_field in &entity.fields {
            if !ServiceSchema::exclude_field_from_input(&entity_field, resolver_type) {
                let field_type = ServiceSchema::get_entity_field_type(&entity_field, resolver_type);
                input = input.field(InputValue::new(&entity_field.name, field_type));
            }
        }

        resolver = resolver.argument(InputValue::new(
            &resolver_input_name,
            TypeRef::named_nn(input.type_name()),
        ));

        self.query = self.query.field(resolver);
        self.schema_builder = self.schema_builder.register(input);
        self
    }

    pub fn generate_create_one_input(
        mut self,
        entity: &ServiceEntity,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        let resolver_input_name = format!("create_{}_input", &entity.name.to_lowercase());
        info!("Creating Create One Resolver: {}", resolver_input_name);

        let mut input = InputObject::new(&resolver_input_name);

        for entity_field in &entity.fields {
            if !ServiceSchema::exclude_field_from_input(&entity_field, resolver_type) {
                let field_type = ServiceSchema::get_entity_field_type(&entity_field, resolver_type);
                input = input.field(InputValue::new(&entity_field.name, field_type));
            }
        }

        resolver = resolver.argument(InputValue::new(
            &resolver_input_name,
            TypeRef::named_nn(input.type_name()),
        ));

        self.mutation = self.mutation.field(resolver);
        self.schema_builder = self.schema_builder.register(input);
        self
    }

    pub fn generate_update_one_input(
        mut self,
        entity: &ServiceEntity,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        info!("Creating Update One Resolver Input");
        let resolver_input_name = format!("update_{}_input", &entity.name.to_lowercase());
        debug!("Input Name: {}", resolver_input_name);

        let mut input = InputObject::new(&resolver_input_name);

        for entity_field in &entity.fields {
            if !ServiceSchema::exclude_field_from_input(&entity_field, resolver_type) {
                let field_type = ServiceSchema::get_entity_field_type(&entity_field, resolver_type);
                input = input.field(InputValue::new(&entity_field.name, field_type));
            }
        }

        input = input.field(InputValue::new(
            "query",
            TypeRef::named_nn(format!("get_{}_input", &entity.name.to_lowercase())),
        ));

        resolver = resolver.argument(InputValue::new(
            &resolver_input_name,
            TypeRef::named_nn(input.type_name()),
        ));

        self.mutation = self.mutation.field(resolver);
        self.schema_builder = self.schema_builder.register(input);
        self
    }

    pub fn generate_resolver_input_value(
        mut self,
        entity: &ServiceEntity,
        resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        info!("Generate Resolver Input Value");

        match resolver_type {
            ResolverType::FindOne => {
                self = self.generate_find_one_input(&entity, resolver, &resolver_type);
            }
            ResolverType::FindMany => {
                self = self.generate_find_many_input(&entity, resolver, &resolver_type);
            }
            ResolverType::CreateOne => {
                self = self.generate_create_one_input(&entity, resolver, &resolver_type);
            }
            ResolverType::UpdateOne => {
                self = self.generate_update_one_input(&entity, resolver, &resolver_type);
            }
        }
        self
    }
}
