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
    graphql::schema::{create_auth_service::TokenData, ResolverType},
};

use super::ServiceResolver;

impl ServiceResolver {
    pub fn guard_resolver(
        ctx: &ResolverContext,
        input_document: &Document,
        entity: &ServiceEntityConfig,
        service_guards: Option<Vec<Guard>>,
        resolver_type: &ResolverType,
        headers: HeaderMap,
        token_data: Option<TokenData>,
    ) -> Result<(), async_graphql::Error> {
        debug!("Guard Resolver Function");

        let guard_context = Guard::create_guard_context(
            headers,
            token_data,
            input_document.clone(),
            resolver_type.to_string(),
        )?;
        let resolver_guards =
            match ServiceEntityConfig::get_resolver(&entity, resolver_type.clone()) {
                Some(resolver) => {
                    debug!(
                        "Guarding resolver {:?} for entity {:?}",
                        resolver_type, entity.name
                    );
                    resolver.guards
                }
                None => None,
            };
        match entity.guards.clone() {
            Some(guards) => Guard::check(&guards, &mut guard_context.clone())?,
            None => (),
        };

        if service_guards.is_some() {
            Guard::check(&service_guards.unwrap(), &mut guard_context.clone())?;
        }
        if resolver_guards.is_some() {
            Guard::check(&resolver_guards.unwrap(), &mut guard_context.clone())?;
        }

        let selection_fields = ctx
            .field()
            .selection_set()
            .into_iter()
            .map(|f| f)
            .collect::<Vec<SelectionField>>();

        for selection_field in selection_fields {
            if selection_field.name() != "__typename" {
                ServiceResolver::guard_field(selection_field, &entity, &mut guard_context.clone())?;
            }
        }

        Ok(())
    }

    pub fn guard_field(
        selection_field: SelectionField,
        entity: &ServiceEntityConfig,
        guard_context: &mut HashMapContext,
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
        guard_context: &mut HashMapContext,
    ) -> Result<(), async_graphql::Error> {
        debug!("Guard Nested");
        debug!("Fields: {:?}", fields);

        let field = fields
            .iter()
            .find(|field| field.name == field_name)
            .unwrap();
        let guards = ServiceEntityFieldConfig::get_guards(field.clone());

        if guards.is_some() {
            Guard::check(&guards.unwrap(), guard_context)?;
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
                    guard_context,
                )?;
            }
        }

        Ok(())
    }
}
