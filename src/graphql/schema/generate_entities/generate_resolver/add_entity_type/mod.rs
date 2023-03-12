use crate::{
    configuration::subgraph::entities::{ScalarOptions, ServiceEntity, ServiceEntityFieldOptions},
    data_sources::{DataSource, DataSources},
};

use super::ServiceSchema;
use async_graphql::{
    dynamic::{Field, FieldFuture, Object, TypeRef},
    Value,
};
use bson::Document;
use json::JsonValue;
use log::{debug, info};

impl ServiceSchema {
    pub fn get_entity_field_resolver_field_type(
        entity_field: &ServiceEntityFieldOptions,
    ) -> TypeRef {
        let entity_field_type = match entity_field.required {
            true => match entity_field.scalar {
                ScalarOptions::String => TypeRef::named_nn(TypeRef::STRING),
                ScalarOptions::Int => TypeRef::named_nn(TypeRef::INT),
                ScalarOptions::Boolean => TypeRef::named_nn(TypeRef::BOOLEAN),
                ScalarOptions::ObjectID => TypeRef::named_nn("ObjectID"),
            },
            false => match entity_field.scalar {
                ScalarOptions::String => TypeRef::named(TypeRef::STRING),
                ScalarOptions::Int => TypeRef::named(TypeRef::INT),
                ScalarOptions::Boolean => TypeRef::named(TypeRef::BOOLEAN),
                ScalarOptions::ObjectID => TypeRef::named("ObjectID"),
            },
        };
        entity_field_type
    }

    pub async fn resolve_http_field(
        json_value: &JsonValue,
        field_name: &str,
        scalar: ScalarOptions,
    ) -> Result<Value, async_graphql::Error> {
        info!("Resolving HTTP Field");

        let value = &json_value[field_name];

        debug!("Accessed Field '{}': {:?}", field_name, value);

        match scalar {
            ScalarOptions::String => {
                info!("Found String Value");
                Ok(Value::from(value.to_string()))
            }
            ScalarOptions::Int => {
                info!("Found Int Value");
                let value = value.as_i32().unwrap();
                Ok(Value::from(value))
            }
            ScalarOptions::Boolean => {
                info!("Found Boolean Value");
                let value = value.as_bool().unwrap();
                Ok(Value::from(value))
            }
            ScalarOptions::ObjectID => {
                info!("Found ObjectID Value");
                let value = value.to_string();
                Ok(Value::from(value))
            }
        }
    }

    pub async fn resolve_document_field(
        doc: &Document,
        field_name: &str,
        scalar: ScalarOptions,
    ) -> Result<Value, async_graphql::Error> {
        info!("Resolving Mongo Field");

        match scalar {
            ScalarOptions::String => {
                let value = doc.get_str(field_name)?;
                debug!("Found String Value: {:?}", value);
                Ok(Value::from(value))
            }
            ScalarOptions::Int => {
                let value = doc.get_i32(field_name)?;
                debug!("Found Int Value: {:?}", value);
                Ok(Value::from(value))
            }
            ScalarOptions::Boolean => {
                let value = doc.get_bool(field_name)?;
                debug!("Found Boolean Value: {:?}", value);
                Ok(Value::from(value))
            }
            ScalarOptions::ObjectID => {
                let value = doc.get_object_id(field_name)?;
                debug!("Found ObjectID Value: {:?}", value);
                Ok(Value::from(value.to_string()))
            }
        }
    }

    pub fn add_entity_type(mut self, entity: &ServiceEntity) -> Self {
        info!("Generating Type For {}", &entity.name);
        let mut entity_type = Object::new(&entity.name);
        debug!("Entity Type: {:?}", entity_type);

        let entity = entity.clone();
        let data_sources = &self.data_sources.clone();

        for entity_field in &entity.fields {
            info!("Entity Field Found");
            debug!("Adding Field: {:?}", entity_field);
            let entity_field_type =
                ServiceSchema::get_entity_field_resolver_field_type(&entity_field).clone();

            let cloned_entity_field = entity_field.clone();
            let entity = entity.clone();
            let data_sources = data_sources.clone();

            entity_type = entity_type
                .field(Field::new(
                    &entity_field.name,
                    entity_field_type,
                    move |ctx| {
                        let cloned_entity_field = cloned_entity_field.clone();
                        let entity = entity.clone();
                        let data_sources = data_sources.clone();

                        FieldFuture::new(async move {
                            info!("Resolving Entity Field");
                            let scalar = cloned_entity_field.scalar;
                            let entity = entity.clone();
                            let data_sources = data_sources.clone();

                            let field_name = ctx.field().name();
                            debug!("Field Name: {:?}", field_name);

                            let data_source =
                                DataSources::get_entity_data_source(&entity, &data_sources);

                            match data_source {
                                DataSource::Mongo(_ds) => {
                                    let doc =
                                        ctx.parent_value.try_downcast_ref::<Document>().unwrap();
                                    debug!("Found Document: {:?}", doc);

                                    let value = ServiceSchema::resolve_document_field(
                                        doc, field_name, scalar,
                                    )
                                    .await;
                                    Ok(Some(value.unwrap()))
                                }
                                DataSource::HTTP(_ds) => {
                                    let json_value =
                                        ctx.parent_value.try_downcast_ref::<JsonValue>().unwrap();

                                    let value = ServiceSchema::resolve_http_field(
                                        json_value, field_name, scalar,
                                    )
                                    .await;
                                    Ok(Some(value.unwrap()))
                                }
                            }
                        })
                    },
                ))
                .key(&entity_field.name);
        }

        info!("Entity Fields Added.");
        debug!("{:?}", entity_type);

        self.schema_builder = self.schema_builder.register(entity_type);
        self
    }
}
