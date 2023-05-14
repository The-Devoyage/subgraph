use crate::{
    configuration::subgraph::entities::{service_entity_field::ServiceEntityField, ServiceEntity},
    graphql::schema::{ResolverType, ServiceSchemaBuilder},
};
use async_graphql::dynamic::{Field, InputObject, InputValue, TypeRef};
use log::{debug, info};

pub mod get_entity_field_type;

impl ServiceSchemaBuilder {
    pub fn is_excluded_input_field(
        entity_field: &ServiceEntityField,
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

    pub fn create_find_one_input(
        mut self,
        entity: &ServiceEntity,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        let resolver_input_name = format!("get_{}_input", &entity.name.to_lowercase());
        debug!("Creating Find One Resolver Input: {}", resolver_input_name);

        let inputs = ServiceSchemaBuilder::create_input(
            resolver_input_name.clone(),
            entity.fields.clone(),
            resolver_type,
        );

        resolver = resolver.argument(InputValue::new(
            &resolver_input_name,
            TypeRef::named_nn(resolver_input_name.clone()),
        ));

        self.query = self.query.field(resolver);
        self = self.register_inputs(inputs);
        self
    }

    pub fn create_find_many_input(
        mut self,
        entity: &ServiceEntity,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        let resolver_input_name = format!("get_{}s_input", &entity.name.to_lowercase());
        info!("Creating Find Many Resolver Input: {}", resolver_input_name);

        let inputs = ServiceSchemaBuilder::create_input(
            resolver_input_name.clone(),
            entity.fields.clone(),
            resolver_type,
        );

        resolver = resolver.argument(InputValue::new(
            &resolver_input_name,
            TypeRef::named_nn(resolver_input_name.clone()),
        ));

        self.query = self.query.field(resolver);
        self = self.register_inputs(inputs);
        self
    }

    pub fn create_create_one_input(
        mut self,
        entity: &ServiceEntity,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        let resolver_input_name = format!("create_{}_input", &entity.name.to_lowercase());
        info!(
            "Creating Create One Resolver Input: {}",
            resolver_input_name
        );

        let inputs = ServiceSchemaBuilder::create_input(
            resolver_input_name.clone(),
            entity.fields.clone(),
            resolver_type,
        );

        resolver = resolver.argument(InputValue::new(
            &resolver_input_name,
            TypeRef::named_nn(resolver_input_name.clone()),
        ));

        self.mutation = self.mutation.field(resolver);
        self = self.register_inputs(inputs);
        self
    }

    pub fn create_update_one_input(
        mut self,
        entity: &ServiceEntity,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        let resolver_input_name = format!("update_{}_input", &entity.name.to_lowercase());
        debug!(
            "Creating Update One Resolver Input: {}",
            resolver_input_name
        );

        let mut inputs = ServiceSchemaBuilder::create_input(
            resolver_input_name.clone(),
            entity.fields.clone(),
            resolver_type,
        );

        let mut input = inputs
            .iter()
            .position(|input| input.type_name() == resolver_input_name)
            .map(|i| inputs.remove(i))
            .unwrap();
        let query_input_name = format!("get_{}_input", &entity.name.to_lowercase());
        input = input.field(InputValue::new(
            "query",
            TypeRef::named_nn(query_input_name.clone()),
        ));
        inputs.push(input);

        resolver = resolver.argument(InputValue::new(
            &resolver_input_name,
            TypeRef::named_nn(resolver_input_name.clone()),
        ));

        self.mutation = self.mutation.field(resolver);
        self = self.register_inputs(inputs);
        self
    }

    pub fn create_update_many_input(
        mut self,
        entity: &ServiceEntity,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        let resolver_input_name = format!("update_{}s_input", &entity.name.to_lowercase());
        debug!(
            "Creating Update Many Resolver Input: {}",
            resolver_input_name
        );

        let mut inputs = ServiceSchemaBuilder::create_input(
            resolver_input_name.clone(),
            entity.fields.clone(),
            resolver_type,
        );

        let mut input = inputs
            .iter()
            .position(|input| input.type_name() == resolver_input_name)
            .map(|i| inputs.remove(i))
            .unwrap();
        let query_input_name = format!("get_{}_input", &entity.name.to_lowercase());
        input = input.field(InputValue::new(
            "query",
            TypeRef::named_nn(query_input_name.clone()),
        ));
        inputs.push(input);

        resolver = resolver.argument(InputValue::new(
            &resolver_input_name,
            TypeRef::named_nn(resolver_input_name.clone()),
        ));

        self.mutation = self.mutation.field(resolver);
        self = self.register_inputs(inputs);
        self
    }

    pub fn create_input(
        input_name: String,
        fields: Vec<ServiceEntityField>,
        resolver_type: &ResolverType,
    ) -> Vec<InputObject> {
        debug!("Creating Input: {}", input_name);
        let mut inputs = Vec::new();
        let mut input = InputObject::new(&input_name);
        for field in &fields {
            if !ServiceSchemaBuilder::is_excluded_input_field(field, resolver_type) {
                let parent_input_name = input_name.clone().replace("_input", "");
                let type_ref_with_inputs = ServiceSchemaBuilder::get_entity_field_type(
                    field,
                    resolver_type,
                    &parent_input_name,
                );

                for input in type_ref_with_inputs.inputs {
                    inputs.push(input);
                }

                input = input.field(InputValue::new(
                    field.name.clone(),
                    type_ref_with_inputs.type_ref,
                ));
            }
        }
        inputs.push(input);
        inputs
    }

    pub fn register_inputs(mut self, inputs: Vec<InputObject>) -> Self {
        debug!("Registering Inputs");
        for input in inputs {
            debug!("Registering Input: {}", input.type_name());
            self.schema_builder = self.schema_builder.register(input);
        }
        self
    }

    pub fn create_resolver_input_value(
        mut self,
        entity: &ServiceEntity,
        resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        debug!("Creating Resolver Input Value");

        match resolver_type {
            ResolverType::FindOne => {
                self = self.create_find_one_input(&entity, resolver, &resolver_type);
            }
            ResolverType::FindMany => {
                self = self.create_find_many_input(&entity, resolver, &resolver_type);
            }
            ResolverType::CreateOne => {
                self = self.create_create_one_input(&entity, resolver, &resolver_type);
            }
            ResolverType::UpdateOne => {
                self = self.create_update_one_input(&entity, resolver, &resolver_type);
            }
            ResolverType::UpdateMany => {
                self = self.create_update_many_input(&entity, resolver, &resolver_type);
            }
        }
        self
    }
}
