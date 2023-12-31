use crate::{
    configuration::subgraph::entities::ServiceEntityConfig,
    data_sources::DataSources,
    graphql::{
        input::ServiceInput,
        schema::{ExcludeFromInput, ResolverType, ServiceSchemaBuilder},
    },
};
use async_graphql::dynamic::{Field, InputObject, InputValue, TypeRef};
use log::debug;

mod get_resolver_input_name;
mod register_inputs;

impl ServiceSchemaBuilder {
    pub fn create_resolver_input_value(
        mut self,
        entity: &ServiceEntityConfig,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        debug!("Creating Resolver Input");

        let mut inputs = Vec::new();

        let input_name =
            ServiceSchemaBuilder::get_resolver_input_name(&entity.name, resolver_type, None);
        let data_sources = self.data_sources.clone();
        let entity_data_source = DataSources::get_entity_data_soruce(&data_sources, entity);

        let mut root_input = InputObject::new(&input_name);

        let include_query_input = resolver_type == &ResolverType::UpdateOne
            || resolver_type == &ResolverType::UpdateMany
            || resolver_type == &ResolverType::FindOne
            || resolver_type == &ResolverType::FindMany;

        if include_query_input {
            let query_input_name = match resolver_type {
                ResolverType::UpdateOne => ServiceSchemaBuilder::get_resolver_input_name(
                    &format!("{}_query", &entity.name.to_lowercase()),
                    &ResolverType::FindOne,
                    None,
                ),
                ResolverType::UpdateMany => ServiceSchemaBuilder::get_resolver_input_name(
                    &format!("{}s_query", &entity.name.to_lowercase()),
                    &ResolverType::UpdateOne,
                    None,
                ),
                ResolverType::FindOne => ServiceSchemaBuilder::get_resolver_input_name(
                    &format!("{}_query", &entity.name.to_lowercase()),
                    &ResolverType::FindOne,
                    None,
                ),
                ResolverType::FindMany => ServiceSchemaBuilder::get_resolver_input_name(
                    &format!("{}s_query", &entity.name.to_lowercase()),
                    &ResolverType::FindMany,
                    None,
                ),
                _ => unreachable!(),
            };

            let exclude_from_input = match resolver_type {
                ResolverType::UpdateOne => Some(ExcludeFromInput::UpdateOneQuery),
                ResolverType::UpdateMany => Some(ExcludeFromInput::UpdateManyQuery),
                ResolverType::FindOne => Some(ExcludeFromInput::FindOne),
                ResolverType::FindMany => Some(ExcludeFromInput::FindMany),
                _ => unreachable!(),
            };

            let rest_inputs = ServiceInput::new(
                query_input_name.clone(),
                entity.fields.clone(),
                resolver_type.clone(), // NOTE: Previsously had find one here
                exclude_from_input,
                entity_data_source.clone(),
            )
            .build(Some(true));

            root_input = root_input.field(InputValue::new(
                "query",
                TypeRef::named_nn(query_input_name.clone()),
            ));

            inputs.extend(rest_inputs);
        }

        let include_values_input = resolver_type == &ResolverType::CreateOne
            || resolver_type == &ResolverType::UpdateOne
            || resolver_type == &ResolverType::UpdateMany;

        if include_values_input {
            let values_input_name = match resolver_type {
                ResolverType::CreateOne => ServiceSchemaBuilder::get_resolver_input_name(
                    &format!("{}_values", &entity.name.to_lowercase()),
                    &ResolverType::CreateOne,
                    None,
                ),
                ResolverType::UpdateOne => ServiceSchemaBuilder::get_resolver_input_name(
                    &format!("{}_values", &entity.name.to_lowercase()),
                    &ResolverType::UpdateOne,
                    None,
                ),
                ResolverType::UpdateMany => ServiceSchemaBuilder::get_resolver_input_name(
                    &format!("{}s_values", &entity.name.to_lowercase()),
                    &ResolverType::UpdateOne,
                    None,
                ),
                _ => unreachable!(),
            };

            let exclude_from_input = match resolver_type {
                ResolverType::CreateOne => Some(ExcludeFromInput::CreateOne),
                ResolverType::UpdateOne => Some(ExcludeFromInput::UpdateOne),
                ResolverType::UpdateMany => Some(ExcludeFromInput::UpdateMany),
                _ => unreachable!(),
            };

            let rest_inputs = ServiceInput::new(
                values_input_name.clone(),
                entity.fields.clone(),
                resolver_type.clone(), // NOTE: Previously had FINDONE here.
                exclude_from_input,
                entity_data_source.clone(),
            )
            .build(None);

            root_input = root_input.field(InputValue::new(
                "values",
                TypeRef::named_nn(values_input_name.clone()),
            ));

            inputs.extend(rest_inputs);
        }

        inputs.push(root_input);

        if !inputs.is_empty() {
            resolver = resolver.argument(InputValue::new(
                &input_name,
                TypeRef::named_nn(input_name.clone()),
            ));
        }

        match resolver_type {
            ResolverType::FindOne | ResolverType::FindMany => {
                self.query = self.query.field(resolver);
            }
            ResolverType::UpdateOne | ResolverType::UpdateMany | ResolverType::CreateOne => {
                self.mutation = self.mutation.field(resolver);
            }
            _ => panic!("Invalid resolver type: {:?}", resolver_type),
        }
        self = self.register_inputs(inputs);
        self
    }
}
