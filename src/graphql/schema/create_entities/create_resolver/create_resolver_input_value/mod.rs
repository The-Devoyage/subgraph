use crate::{
    configuration::subgraph::entities::ServiceEntity,
    graphql::{
        input::ServiceInput,
        schema::{ExcludeFromInput, ResolverType, ServiceSchemaBuilder},
    },
};
use async_graphql::dynamic::{Field, InputValue, TypeRef};
use log::debug;

mod get_resolver_input_name;
mod register_inputs;

impl ServiceSchemaBuilder {
    pub fn create_resolver_input_value(
        mut self,
        entity: &ServiceEntity,
        mut resolver: Field,
        resolver_type: &ResolverType,
    ) -> Self {
        debug!("Creating Resolver Input");

        let resolver_input_name =
            ServiceSchemaBuilder::get_resolver_input_name(&entity.name, resolver_type, None);

        let exclude_from_input = match resolver_type {
            ResolverType::FindOne => Some(ExcludeFromInput::FindOne),
            ResolverType::CreateOne => Some(ExcludeFromInput::CreateOne),
            ResolverType::FindMany => Some(ExcludeFromInput::FindMany),
            ResolverType::UpdateOne => Some(ExcludeFromInput::UpdateOne),
            ResolverType::UpdateMany => Some(ExcludeFromInput::UpdateMany),
            ResolverType::InternalType => None,
        };

        let mut inputs = ServiceInput::new(
            resolver_input_name.clone(),
            entity.fields.clone(),
            resolver_type.clone(),
            exclude_from_input,
        )
        .build();

        let include_query_input =
            resolver_type == &ResolverType::UpdateOne || resolver_type == &ResolverType::UpdateMany;

        if include_query_input {
            let mut query_input = match inputs
                .iter()
                .position(|input| input.type_name() == resolver_input_name)
                .map(|i| inputs.remove(i))
            {
                Some(input) => input,
                None => return self,
            };

            let query_input_name = match resolver_type {
                ResolverType::UpdateOne => ServiceSchemaBuilder::get_resolver_input_name(
                    &entity.name.to_lowercase(),
                    &ResolverType::FindOne,
                    None,
                ),
                ResolverType::UpdateMany => ServiceSchemaBuilder::get_resolver_input_name(
                    &format!("{}s_query", &entity.name.to_lowercase()),
                    &ResolverType::UpdateOne,
                    None,
                ),
                _ => unreachable!(),
            };

            let exclude_from_input = match resolver_type {
                ResolverType::UpdateOne => Some(ExcludeFromInput::UpdateOneQuery),
                ResolverType::UpdateMany => Some(ExcludeFromInput::UpdateManyQuery),
                _ => unreachable!(),
            };

            let rest_inputs = ServiceInput::new(
                query_input_name.clone(),
                entity.fields.clone(),
                ResolverType::FindOne,
                exclude_from_input,
            )
            .build();

            query_input = query_input.field(InputValue::new(
                "query",
                TypeRef::named_nn(query_input_name.clone()),
            ));

            inputs.push(query_input);
            inputs.extend(rest_inputs);
        }

        if !inputs.is_empty() {
            resolver = resolver.argument(InputValue::new(
                &resolver_input_name,
                TypeRef::named_nn(resolver_input_name.clone()),
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
