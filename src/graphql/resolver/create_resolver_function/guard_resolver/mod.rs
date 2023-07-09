use async_graphql::{dynamic::ResolverContext, SelectionField};
use bson::Document;
use evalexpr::HashMapContext;
use http::HeaderMap;
use log::debug;

use crate::{
    configuration::subgraph::{
        entities::{service_entity_field::ServiceEntityFieldConfig, ServiceEntityConfig},
        guard::Guard,
    },
    graphql::schema::ResolverType,
};

use super::ServiceResolver;

impl ServiceResolver {
    pub fn guard_resolver(
        ctx: &ResolverContext,
        input_document: &Document,
        entity: &ServiceEntityConfig,
        service_guards: Option<Vec<Guard>>,
        resolver_type: &ResolverType,
    ) -> Result<(), async_graphql::Error> {
        let headers = ctx.data_unchecked::<HeaderMap>().clone();
        let guard_context =
            Guard::create_guard_context(headers, input_document.clone(), entity.clone())?;
        let resolver_guards =
            match ServiceEntityConfig::get_resolver(&entity, resolver_type.clone()) {
                Some(resolver) => resolver.guards,
                None => None,
            };
        let entity_guards = entity.guards.clone();

        if service_guards.is_some() {
            Guard::check(&service_guards.unwrap(), &guard_context)?;
        }
        if resolver_guards.is_some() {
            Guard::check(&resolver_guards.unwrap(), &guard_context)?;
        }
        if entity_guards.is_some() {
            Guard::check(&entity_guards.unwrap(), &guard_context)?;
        }

        let selection_fields = ctx
            .field()
            .selection_set()
            .into_iter()
            .map(|f| f)
            .collect::<Vec<SelectionField>>();

        for selection_field in selection_fields {
            ServiceResolver::guard_field(selection_field, &entity, guard_context.clone())?;
        }

        Ok(())
    }

    pub fn guard_field(
        selection_field: SelectionField,
        entity: &ServiceEntityConfig,
        guard_context: HashMapContext,
    ) -> Result<(), async_graphql::Error> {
        debug!("Guard Field");
        let field_name = selection_field.name();
        let fields = ServiceEntityFieldConfig::get_fields_recursive(
            entity.fields.clone(),
            field_name.to_string(),
        )?;
        ServiceResolver::guard_nested(selection_field, fields, field_name, guard_context)?;
        Ok(())
    }

    pub fn guard_nested(
        selection_field: SelectionField,
        fields: Vec<ServiceEntityFieldConfig>,
        field_name: &str,
        guard_context: HashMapContext,
    ) -> Result<(), async_graphql::Error> {
        debug!("Guard Nested");
        debug!("Fields: {:?}", fields);

        let field = fields
            .iter()
            .find(|field| field.name == field_name)
            .unwrap();
        let guards = ServiceEntityFieldConfig::get_guards(field.clone());

        if guards.is_some() {
            Guard::check(&guards.unwrap(), &guard_context)?;
        }

        if field.as_type.is_some() {
            //TODO: Guards for as type entities?
            return Ok(());
        }

        if selection_field.selection_set().count() > 0 && field.as_type.is_none() {
            for selection_field in selection_field.selection_set().into_iter() {
                ServiceResolver::guard_nested(
                    selection_field,
                    field.fields.clone().unwrap(),
                    selection_field.name(),
                    guard_context.clone(),
                )?;
            }
        }

        Ok(())
    }
}
