use crate::{
    configuration::subgraph::entities::{service_entity_field::ServiceEntityField, ServiceEntity},
    graphql::schema::{ExcludeFromInput, ResolverType, ServiceSchemaBuilder},
};
use async_graphql::dynamic::{Field, InputObject, InputValue, TypeRef};
use log::{debug, info};

pub mod get_entity_field_type;

impl ServiceSchemaBuilder {
    pub fn get_resolver_input_name(
        entity_name: &str,
        resolver_type: &ResolverType,
        list: Option<bool>,
    ) -> String {
        info!("Getting Resolver Input Name");
        let input_name = match resolver_type {
            ResolverType::FindOne => format!("get_{}_input", &entity_name.to_lowercase()),
            ResolverType::CreateOne => format!("create_{}_input", &entity_name.to_lowercase()),
            ResolverType::FindMany => format!("get_{}s_input", &entity_name.to_lowercase()),
            ResolverType::UpdateOne => format!("update_{}_input", &entity_name.to_lowercase()),
            ResolverType::UpdateMany => format!("update_{}s_input", &entity_name.to_lowercase()),
            ResolverType::InternalType => {
                if list.unwrap_or(false) {
                    format!("get_{}s_input", &entity_name.to_lowercase())
                } else {
                    format!("get_{}_input", &entity_name.to_lowercase())
                }
            }
        };
        debug!("Resolver Input Name: {}", input_name);
        input_name
    }

    pub fn is_excluded_input_field(
        entity_field: &ServiceEntityField,
        excluded: Option<ExcludeFromInput>,
    ) -> bool {
        info!("Checking If Field Should Be Excluded From Input");
        let exclude_from_input = entity_field.exclude_from_input.clone();
        debug!("Exclude From Input Config: {:?}", exclude_from_input);

        if exclude_from_input.is_none() {
            return false;
        }

        match exclude_from_input {
            Some(exclude_from_input) => {
                if exclude_from_input.contains(&ExcludeFromInput::All) {
                    return true;
                }
                if exclude_from_input.contains(&excluded.unwrap()) {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub fn create_find_one_input(
        mut self,
        entity: &ServiceEntity,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        let resolver_input_name =
            ServiceSchemaBuilder::get_resolver_input_name(&entity.name, resolver_type, None);

        debug!("Creating Find One Resolver Input: {}", resolver_input_name);

        let inputs = ServiceSchemaBuilder::create_input(
            resolver_input_name.clone(),
            entity.fields.clone(),
            resolver_type,
            Some(ExcludeFromInput::FindOne),
        );

        if !inputs.is_empty() {
            resolver = resolver.argument(InputValue::new(
                &resolver_input_name,
                TypeRef::named_nn(resolver_input_name.clone()),
            ));
        }

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
        let resolver_input_name =
            ServiceSchemaBuilder::get_resolver_input_name(&entity.name, resolver_type, None);
        info!("Creating Find Many Resolver Input: {}", resolver_input_name);

        let inputs = ServiceSchemaBuilder::create_input(
            resolver_input_name.clone(),
            entity.fields.clone(),
            resolver_type,
            Some(ExcludeFromInput::FindMany),
        );

        if !inputs.is_empty() {
            resolver = resolver.argument(InputValue::new(
                &resolver_input_name,
                TypeRef::named_nn(resolver_input_name.clone()),
            ));
        }

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
        let resolver_input_name =
            ServiceSchemaBuilder::get_resolver_input_name(&entity.name, resolver_type, None);
        info!(
            "Creating Create One Resolver Input: {}",
            resolver_input_name
        );

        let inputs = ServiceSchemaBuilder::create_input(
            resolver_input_name.clone(),
            entity.fields.clone(),
            resolver_type,
            Some(ExcludeFromInput::CreateOne),
        );

        if !inputs.is_empty() {
            resolver = resolver.argument(InputValue::new(
                &resolver_input_name,
                TypeRef::named_nn(resolver_input_name.clone()),
            ));
        }

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
        let resolver_input_name =
            ServiceSchemaBuilder::get_resolver_input_name(&entity.name, resolver_type, None);
        debug!(
            "Creating Update One Resolver Input: {}",
            resolver_input_name
        );

        let mut inputs = ServiceSchemaBuilder::create_input(
            resolver_input_name.clone(),
            entity.fields.clone(),
            resolver_type,
            Some(ExcludeFromInput::UpdateOne),
        );

        let mut update_one_input = match inputs
            .iter()
            .position(|input| input.type_name() == resolver_input_name)
            .map(|i| inputs.remove(i))
        {
            Some(input) => input,
            None => return self,
        };

        let update_one_query_input_name = ServiceSchemaBuilder::get_resolver_input_name(
            &format!("{}_query", &entity.name.to_lowercase()),
            &ResolverType::UpdateOne,
            None,
        );
        let update_one_inputs = ServiceSchemaBuilder::create_input(
            update_one_query_input_name.clone(),
            entity.fields.clone(),
            &ResolverType::FindOne,
            Some(ExcludeFromInput::UpdateOneQuery),
        );
        update_one_input = update_one_input.field(InputValue::new(
            "query",
            TypeRef::named_nn(update_one_query_input_name.clone()),
        ));

        inputs.push(update_one_input);
        inputs.extend(update_one_inputs);

        if !inputs.is_empty() {
            resolver = resolver.argument(InputValue::new(
                &resolver_input_name,
                TypeRef::named_nn(resolver_input_name.clone()),
            ));
        }

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
        let resolver_input_name =
            ServiceSchemaBuilder::get_resolver_input_name(&entity.name, resolver_type, None);
        debug!(
            "Creating Update Many Resolver Input: {}",
            resolver_input_name
        );

        let mut inputs = ServiceSchemaBuilder::create_input(
            resolver_input_name.clone(),
            entity.fields.clone(),
            resolver_type,
            Some(ExcludeFromInput::UpdateMany),
        );

        let mut update_many_input = match inputs
            .iter()
            .position(|input| input.type_name() == resolver_input_name)
            .map(|i| inputs.remove(i))
        {
            Some(input) => input,
            None => return self,
        };

        let update_many_query_input_name = ServiceSchemaBuilder::get_resolver_input_name(
            &format!("{}s_query", &entity.name.to_lowercase()),
            &ResolverType::UpdateOne, //HACK: UpdateMany uses UpdateOne query input to pluralize
            None,
        );
        let update_many_inputs = ServiceSchemaBuilder::create_input(
            update_many_query_input_name.clone(),
            entity.fields.clone(),
            &ResolverType::FindOne,
            Some(ExcludeFromInput::UpdateManyQuery),
        );
        update_many_input = update_many_input.field(InputValue::new(
            "query",
            TypeRef::named_nn(update_many_query_input_name.clone()),
        ));

        inputs.push(update_many_input);
        inputs.extend(update_many_inputs);

        if !inputs.is_empty() {
            resolver = resolver.argument(InputValue::new(
                &resolver_input_name,
                TypeRef::named_nn(resolver_input_name.clone()),
            ));
        }

        self.mutation = self.mutation.field(resolver);
        self = self.register_inputs(inputs);
        self
    }

    pub fn create_input(
        input_name: String,
        fields: Vec<ServiceEntityField>,
        resolver_type: &ResolverType,
        exclude_from_input: Option<ExcludeFromInput>,
    ) -> Vec<InputObject> {
        debug!("Creating Input: {}", input_name);
        let mut inputs = Vec::new();
        let mut input = InputObject::new(&input_name);
        let mut excluded_count = 0;
        for field in &fields {
            if !ServiceSchemaBuilder::is_excluded_input_field(field, exclude_from_input.clone()) {
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
            } else {
                excluded_count = excluded_count + 1
            }
        }
        if excluded_count != fields.len() {
            inputs.push(input);
        }
        debug!("Created Inputs: {:?}", inputs);
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
            _ => panic!("Resolver Type Not Supported"),
        }
        self
    }
}
