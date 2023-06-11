use crate::{
    configuration::subgraph::entities::{service_entity_field::ServiceEntityField, ServiceEntity},
    data_sources::{sql::services::ResponseRow, DataSource, DataSources},
    graphql::schema::ResolverType,
};

use super::ServiceSchemaBuilder;
use async_graphql::{
    dynamic::{Field, FieldFuture, InputValue, Object, TypeRef},
    ErrorExtensions,
};
use bson::{to_document, Document};
use json::JsonValue;
use log::debug;

pub mod get_field_type_ref;
pub mod resolve_fields;

#[derive(Debug)]
pub struct TypeRefsAndDefs {
    type_ref: TypeRef,
    type_defs: Vec<Object>,
    is_root_object: bool,
}

impl ServiceSchemaBuilder {
    pub fn create_field(
        entity_field: ServiceEntityField,
        type_ref: TypeRef,
        data_source: &DataSource,
        is_root_object: bool,
    ) -> Field {
        debug!("Creating Field, {:?}", entity_field.name);
        let cloned_entity_field = entity_field.clone();
        let cloned_data_source = data_source.clone();

        let field = Field::new(&entity_field.name, type_ref, move |ctx| {
            let cloned_entity_field = cloned_entity_field.clone();
            let data_source = cloned_data_source.clone();

            // Resolve Field
            FieldFuture::new(async move {
                debug!("---Resolving Entity Field");
                let scalar = cloned_entity_field.scalar;

                let field_name = ctx.field().name();
                debug!("---Field Name: {:?} as {:?}", field_name, scalar);
                debug!("---Is Root Object: {:?}", is_root_object);

                match is_root_object {
                    false => {
                        let object = ctx.parent_value.as_value().unwrap();
                        let json = object.clone().into_json().unwrap();
                        debug!("---Found JSON: {:?}", json);

                        let json_object: serde_json::Value;
                        if json.is_string() {
                            debug!("---Found String: {:?}", json.as_str().unwrap());
                            json_object = serde_json::from_str(&json.as_str().unwrap()).unwrap();
                        } else {
                            json_object = json;
                        }

                        debug!("---Converted To JSON Object: {:?}", json_object);

                        let document: Document;

                        if json_object.is_array() {
                            document = to_document(&json_object[0]).unwrap();
                        } else if json_object.is_object() {
                            document = to_document(&json_object).unwrap();
                        } else {
                            return Err(async_graphql::Error::new(
                                "Invalid JSON Object - Received unexpected JSON type",
                            )
                            .extend_with(|_err, e| {
                                e.set("field", field_name);
                                e.set("received", json_object.to_string());
                            }));
                        }

                        debug!("---Converted To Document: {:?}", document);

                        let value = ServiceSchemaBuilder::resolve_document_field(
                            &document,
                            field_name,
                            scalar.clone(),
                            cloned_entity_field.list.unwrap_or(false),
                        )
                        .unwrap();

                        debug!(
                            "---Found Document Field Value: {:?}: {:?} - {:?}",
                            field_name, value, scalar
                        );

                        Ok(Some(value.clone()))
                    }
                    true => match data_source {
                        DataSource::Mongo(_ds) => {
                            let doc = ctx.parent_value.try_downcast_ref::<Document>().unwrap();
                            debug!("---Found Document: {:?}", doc);

                            let value = ServiceSchemaBuilder::resolve_document_field(
                                doc,
                                field_name,
                                scalar.clone(),
                                entity_field.list.unwrap_or(false),
                            )
                            .unwrap();

                            debug!(
                                "---Found Document Field Value: {:?}: {:?} - {:?}",
                                field_name, value, scalar
                            );

                            Ok(Some(value))
                        }
                        DataSource::HTTP(_ds) => {
                            let json_value =
                                ctx.parent_value.try_downcast_ref::<JsonValue>().unwrap();
                            debug!("---Found Json Value: {:?}", json_value);

                            let value = ServiceSchemaBuilder::resolve_http_field(
                                json_value, field_name, scalar,
                            )
                            .await;

                            Ok(Some(value.unwrap()))
                        }
                        DataSource::SQL(_ds) => {
                            let response_row =
                                ctx.parent_value.try_downcast_ref::<ResponseRow>().unwrap();

                            let value = ServiceSchemaBuilder::resolve_sql_field(
                                response_row,
                                field_name,
                                scalar,
                            );

                            Ok(Some(value.unwrap()))
                        }
                    },
                }
            })
        });
        debug!("---Created Field: {:?}", field);
        field
    }

    pub fn add_field(
        mut entity_type: Object,
        entity_field: ServiceEntityField,
        type_ref: TypeRef,
        data_source: DataSource,
        is_root_object: bool,
    ) -> Object {
        debug!("Adding Field: {:?}", entity_field.name);
        let cloned_entity_field = entity_field.clone();
        entity_type = entity_type
            .field(ServiceSchemaBuilder::create_field(
                cloned_entity_field,
                type_ref,
                &data_source,
                is_root_object,
            ))
            .key(&entity_field.name);

        debug!("Added Field With Resolver:\n {:?}", entity_type);

        entity_type
    }

    pub fn create_type_defs(
        &self,
        data_sources: &DataSources,
        entity: &ServiceEntity,
        type_name: String,
        fields: Vec<ServiceEntityField>,
    ) -> Vec<Object> {
        let mut type_defs = Vec::new();

        debug!("Creating Type For: `{}`", type_name);
        let mut type_def = Object::new(type_name);

        let data_source = DataSources::get_data_source_for_entity(data_sources, entity);

        for entity_field in fields {
            if entity_field.exclude_from_output.unwrap_or(false) {
                continue;
            }

            debug!("Creating Field Def For:\n {:?}", entity_field);
            let type_defs_and_refs = self.get_field_type_ref(&entity_field, &data_sources, entity);

            debug!("Field Type Defs and Ref, {:?}", type_defs_and_refs);

            for object_type_def in type_defs_and_refs.type_defs {
                type_defs.push(object_type_def);
            }

            if entity_field.as_type.is_some() {
                debug!("Creating As Type Resolver For: {:?}", entity_field);
                let list = entity_field.list.unwrap_or(false);
                let as_type_entity = self
                    .subgraph_config
                    .service
                    .entities
                    .iter()
                    .find(|e| e.name == entity_field.clone().as_type.unwrap());
                if as_type_entity.is_none() {
                    panic!(
                        "Could not find entity `{}` for as_type resolver",
                        entity_field.as_type.unwrap()
                    );
                }
                let as_type_entity = as_type_entity.unwrap();
                let as_type_resolver = self.create_resolver(
                    as_type_entity,
                    ResolverType::InternalType,
                    Some(entity_field.name.clone()),
                    Some(list),
                    Some(entity.name.clone()),
                );
                let resolver_input_name = ServiceSchemaBuilder::get_resolver_input_name(
                    &as_type_entity.name,
                    &ResolverType::InternalType,
                    Some(list),
                );
                type_def = type_def.field(as_type_resolver.argument(InputValue::new(
                    format!("{}", entity_field.name),
                    TypeRef::named_nn(resolver_input_name),
                )));
            } else {
                let cloned_entity_field = entity_field.clone();
                type_def = ServiceSchemaBuilder::add_field(
                    type_def,
                    cloned_entity_field,
                    type_defs_and_refs.type_ref,
                    data_source.clone(),
                    type_defs_and_refs.is_root_object,
                );
            }
        }

        type_defs.push(type_def);

        debug!("Created Type Defs: {:?}", type_defs);

        type_defs
    }

    pub fn register_types(mut self, type_defs: Vec<Object>) -> Self {
        debug!("Registering Types");
        for type_def in type_defs {
            debug!("Registering Type Def: {:?}", type_def);
            self.schema_builder = self.schema_builder.register(type_def);
        }
        self
    }

    pub fn create_entity_type_defs(mut self, entity: &ServiceEntity) -> Self {
        debug!("Creating Types For Entity: {}", &entity.name);
        let entity_type_defs = self.create_type_defs(
            &self.data_sources.clone(),
            entity,
            entity.name.clone(),
            entity.fields.clone(),
        );

        self = self.register_types(entity_type_defs);

        self
    }
}
