use async_graphql::{
    dynamic::{Field, FieldFuture, TypeRef},
    SelectionField,
};
use bson::Document;
use evalexpr::HashMapContext;
use http::HeaderMap;
use log::{debug, info};

use crate::{
    configuration::subgraph::{
        entities::{service_entity_field::ServiceEntityField, ServiceEntity},
        guard::Guard,
    },
    data_sources::DataSources,
};

use super::{ResolverType, ServiceSchemaBuilder};

mod create_resolver_input_value;

#[derive(Debug)]
pub struct ResolverConfig {
    resolver_name: String,
    return_type: TypeRef,
}

impl ServiceSchemaBuilder {
    pub fn create_resolver_config(
        entity: &ServiceEntity,
        resolver_type: ResolverType,
    ) -> ResolverConfig {
        info!("Creating Resolver Config");

        let resolver_type = match resolver_type {
            ResolverType::FindOne => ResolverConfig {
                resolver_name: format!("get_{}", &entity.name.to_lowercase()),
                return_type: TypeRef::named_nn(&entity.name),
            },
            ResolverType::CreateOne => ResolverConfig {
                resolver_name: format!("create_{}", &entity.name.to_lowercase()),
                return_type: TypeRef::named_nn(&entity.name),
            },
            ResolverType::FindMany => ResolverConfig {
                resolver_name: format!("get_{}s", &entity.name.to_lowercase()),
                return_type: TypeRef::named_nn_list_nn(&entity.name),
            },
            ResolverType::UpdateOne => ResolverConfig {
                resolver_name: format!("update_{}", &entity.name.to_lowercase()),
                return_type: TypeRef::named_nn(&entity.name),
            },
            ResolverType::UpdateMany => ResolverConfig {
                resolver_name: format!("update_{}s", &entity.name.to_lowercase()),
                return_type: TypeRef::named_nn_list_nn(&entity.name),
            },
        };

        debug!("Resolver Type: {:?}", resolver_type);

        resolver_type
    }

    pub fn guard_nested(
        selection_field: SelectionField,
        fields: Vec<ServiceEntityField>,
        field_name: &str,
        guard_context: HashMapContext,
    ) -> Result<(), async_graphql::Error> {
        debug!("Guard Nested");
        debug!("Fields: {:?}", fields);
        let field = fields
            .iter()
            .find(|field| field.name == field_name)
            .unwrap();
        let guards = ServiceEntityField::get_guards(field.clone());
        debug!("Field Guards: {:?}", guards);

        if guards.is_some() {
            Guard::check(&guards.unwrap(), &guard_context)?;
        }

        if field.as_type.is_some() {
            //NOTE: Guards for as type entities?
            return Ok(());
        }

        if selection_field.selection_set().count() > 0 {
            for selection_field in selection_field.selection_set().into_iter() {
                ServiceSchemaBuilder::guard_nested(
                    selection_field,
                    field.fields.clone().unwrap(),
                    selection_field.name(),
                    guard_context.clone(),
                )?;
            }
        }

        Ok(())
    }

    pub fn guard_field(
        selection_field: SelectionField,
        entity: &ServiceEntity,
        guard_context: HashMapContext,
    ) -> Result<(), async_graphql::Error> {
        debug!("Guard Field");
        let field_name = selection_field.name();
        let fields = ServiceEntityField::get_fields_recursive(
            entity.fields.clone(),
            field_name.to_string(),
        )?;
        ServiceSchemaBuilder::guard_nested(selection_field, fields, field_name, guard_context)?;
        Ok(())
    }

    pub fn create_resolver(mut self, entity: &ServiceEntity, resolver_type: ResolverType) -> Self {
        info!("Creating Resolver");

        let resolver_config = ServiceSchemaBuilder::create_resolver_config(entity, resolver_type);
        let cloned_entity = entity.clone();
        let service_guards = self.subgraph_config.service.guards.clone();
        let entity_guards = entity.guards.clone();
        let resolver = ServiceEntity::get_resolver(&entity, resolver_type);
        let resolver_guards = if resolver.is_some() {
            resolver.unwrap().guards
        } else {
            None
        };

        let resolver = Field::new(
            resolver_config.resolver_name,
            resolver_config.return_type,
            move |ctx| {
                let cloned_entity = cloned_entity.clone();
                let service_guards = service_guards.clone();
                let entity_guards = entity_guards.clone();
                let resolver_guards = resolver_guards.clone();

                FieldFuture::new(async move {
                    let data_sources = ctx.data_unchecked::<DataSources>().clone();
                    let input = ctx.args.try_get(&format!("{}_input", ctx.field().name()))?;
                    let headers = ctx.data_unchecked::<HeaderMap>().clone();
                    let input_document = &input.deserialize::<Document>().unwrap();
                    let guard_context = Guard::create_guard_context(
                        headers,
                        input_document.clone(),
                        cloned_entity.clone(),
                    )?;

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
                        ServiceSchemaBuilder::guard_field(
                            selection_field,
                            &cloned_entity,
                            guard_context.clone(),
                        )?;
                    }

                    let results =
                        DataSources::execute(&data_sources, &input, cloned_entity, resolver_type)
                            .await?;

                    Ok(Some(results))
                })
            },
        );

        debug!("Resolver: {:?}", resolver);

        self = self.create_resolver_input_value(&entity, resolver, &resolver_type);
        self
    }
}
