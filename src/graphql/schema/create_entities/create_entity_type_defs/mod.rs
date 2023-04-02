use crate::{
    configuration::subgraph::entities::{ScalarOptions, ServiceEntity, ServiceEntityField},
    data_sources::{DataSource, DataSources},
};

use super::ServiceSchemaBuilder;
use async_graphql::dynamic::{Field, FieldFuture, Object, TypeRef};
use bson::{to_document, Document};
use json::JsonValue;
use log::debug;

pub mod resolve_fields;

#[derive(Debug)]
pub struct TypeRefsAndDefs {
    type_ref: TypeRef,
    type_defs: Vec<Object>,
}

impl ServiceSchemaBuilder {
    pub fn get_field_type_ref(
        entity_field: &ServiceEntityField,
        data_sources: &DataSources,
        entity: &ServiceEntity,
    ) -> TypeRefsAndDefs {
        debug!("Getting Field Type Ref And Defs");
        let mut type_defs = Vec::new();

        let type_ref = match entity_field.required {
            Some(true) => match entity_field.scalar.clone() {
                ScalarOptions::String => TypeRef::named_nn(TypeRef::STRING),
                ScalarOptions::Int => TypeRef::named_nn(TypeRef::INT),
                ScalarOptions::Boolean => TypeRef::named_nn(TypeRef::BOOLEAN),
                ScalarOptions::ObjectID => TypeRef::named_nn("ObjectID"),
                ScalarOptions::Object => {
                    let object_type_defs = ServiceSchemaBuilder::create_type_defs(
                        data_sources,
                        entity,
                        entity_field.name.clone(),
                        entity_field.fields.clone().unwrap_or(Vec::new()),
                        false,
                    );

                    for object in object_type_defs {
                        type_defs.push(object);
                    }

                    TypeRef::named_nn(entity_field.name.clone())
                }
            },
            _ => match entity_field.scalar.clone() {
                ScalarOptions::String => TypeRef::named(TypeRef::STRING),
                ScalarOptions::Int => TypeRef::named(TypeRef::INT),
                ScalarOptions::Boolean => TypeRef::named(TypeRef::BOOLEAN),
                ScalarOptions::ObjectID => TypeRef::named("ObjectID"),
                ScalarOptions::Object => {
                    let object_type_defs = ServiceSchemaBuilder::create_type_defs(
                        data_sources,
                        entity,
                        entity_field.name.clone(),
                        entity_field.fields.clone().unwrap_or(vec![]),
                        false,
                    );

                    for object in object_type_defs {
                        type_defs.push(object)
                    }

                    TypeRef::named(entity_field.name.clone())
                }
            },
        };

        debug!("Created Type Ref: {:?}", type_ref);
        debug!("Created Type Defs: {:?}", type_defs);

        TypeRefsAndDefs {
            type_ref,
            type_defs,
        }
    }

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
                debug!("Resolving Entity Field");
                let scalar = cloned_entity_field.scalar;

                let field_name = ctx.field().name();
                debug!("Field Name: {:?} as {:?}", field_name, scalar);
                debug!("Is Root Object: {:?}", is_root_object);

                match is_root_object {
                    false => {
                        //MOVE THIS and RETURN CONST VALUE
                        let object = ctx.parent_value.as_value().unwrap();
                        let json = object.clone().into_json().unwrap();

                        //convert json to document
                        let document = to_document(&json).unwrap();
                        debug!("Converted To Document: {:?}", document);

                        let value = ServiceSchemaBuilder::resolve_document_field(
                            &document,
                            field_name,
                            scalar.clone(),
                        )
                        .unwrap();

                        debug!(
                            "Found Document Field Value: {:?}: {:?} - {:?}",
                            field_name, value, scalar
                        );

                        Ok(Some(value.clone()))
                    }
                    true => match data_source {
                        DataSource::Mongo(_ds) => {
                            let doc = ctx.parent_value.try_downcast_ref::<Document>().unwrap();
                            debug!("Found Document: {:?}", doc);

                            let value = ServiceSchemaBuilder::resolve_document_field(
                                doc,
                                field_name,
                                scalar.clone(),
                            )
                            .unwrap();

                            debug!(
                                "Found Document Field Value: {:?}: {:?} - {:?}",
                                field_name, value, scalar
                            );

                            Ok(Some(value))
                        }
                        DataSource::HTTP(_ds) => {
                            let json_value =
                                ctx.parent_value.try_downcast_ref::<JsonValue>().unwrap();
                            debug!("Found Json Value: {:?}", json_value);

                            let value = ServiceSchemaBuilder::resolve_http_field(
                                json_value, field_name, scalar,
                            )
                            .await;

                            Ok(Some(value.unwrap()))
                        }
                    },
                }
            })
        });
        debug!("Created Field: {:?}", field);
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
        data_sources: &DataSources,
        entity: &ServiceEntity,
        type_name: String,
        fields: Vec<ServiceEntityField>,
        is_root_object: bool,
    ) -> Vec<Object> {
        let mut type_defs = Vec::new();
        debug!("Creating Type For: `{}`", type_name);
        let mut type_def = Object::new(type_name);

        let data_source = DataSources::get_entity_data_source(entity, data_sources);

        for entity_field in fields {
            debug!("Creating Field Def For:\n {:?}", entity_field);
            let type_defs_and_refs =
                ServiceSchemaBuilder::get_field_type_ref(&entity_field, &data_sources, entity);

            debug!("Field Type Defs and Ref, {:?}", type_defs_and_refs);

            for object_type_def in type_defs_and_refs.type_defs {
                type_defs.push(object_type_def);
            }

            let cloned_entity_field = entity_field.clone();
            type_def = ServiceSchemaBuilder::add_field(
                type_def,
                cloned_entity_field,
                type_defs_and_refs.type_ref,
                data_source.clone(),
                is_root_object,
            )
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
        let entity_type_defs = ServiceSchemaBuilder::create_type_defs(
            &self.data_sources.clone(),
            entity,
            entity.name.clone(),
            entity.fields.clone(),
            true,
        );

        self = self.register_types(entity_type_defs);

        self
    }
}
