use crate::configuration::subgraph::{ScalarOptions, ServiceEntity, ServiceEntityFieldOptions};

use super::ServiceSchema;
use async_graphql::{
    dynamic::{Field, FieldFuture, Object, TypeRef},
    Value,
};
use bson::{spec::ElementType, Document};
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

    pub fn add_entity_type(mut self, entity: &ServiceEntity) -> Self {
        info!("Generating Type For {}", &entity.name);
        let mut entity_type = Object::new(&entity.name);
        debug!("{:?}", entity_type);

        let entity = entity.clone();

        for entity_field in entity.fields {
            info!("Entity Field Found");
            debug!("Adding Field, {:?}", entity_field);
            let entity_field_type =
                ServiceSchema::get_entity_field_resolver_field_type(&entity_field).clone();

            let cloned_entity_field = entity_field.clone();
            entity_type = entity_type
                .field(Field::new(
                    &entity_field.name,
                    entity_field_type,
                    move |ctx| {
                        let cloned_entity_field = cloned_entity_field.clone();

                        FieldFuture::new(async move {
                            let scalar = cloned_entity_field.scalar;
                            info!("Resolving Entity Field");
                            let doc = ctx.parent_value.try_downcast_ref::<Document>()?;

                            info!("Parent Result Found");
                            debug!("{:?}", doc);

                            info!("Getting Field Name");
                            let field_name = ctx.field().name();
                            debug!("{:?}", field_name);

                            info!("Accessing Field, {}", field_name);
                            let value = doc.get(field_name).unwrap();

                            info!("Found Field Value");
                            debug!("{:?}", value);

                            match scalar {
                                ScalarOptions::String => {
                                    let value = doc.get_str(field_name)?;
                                    info!("Found String Value");
                                    Ok(Some(Value::from(value)))
                                }
                                ScalarOptions::Int => {
                                    let value = doc.get_i32(field_name)?;
                                    info!("Found Int Value");
                                    Ok(Some(Value::from(value)))
                                }
                                ScalarOptions::Boolean => {
                                    let value = doc.get_bool(field_name)?;
                                    info!("Found Boolean Value");
                                    Ok(Some(Value::from(value)))
                                }
                                ScalarOptions::ObjectID => {
                                    let value = doc.get_object_id(field_name)?;
                                    info!("Found ObjectID Value");
                                    Ok(Some(Value::from(value.to_string())))
                                } // _ => unreachable!(),
                            }

                            // match value.element_type() {
                            //     ElementType::String => {
                            //         let value = doc.get_str(field_name)?;
                            //         Ok(Some(Value::from(value)))
                            //     }
                            //     ElementType::Int32 => {
                            //         let value = doc.get_i32(field_name)?;
                            //         Ok(Some(Value::from(value)))
                            //     }
                            //     ElementType::Boolean => {
                            //         let value = doc.get_bool(field_name)?;
                            //         Ok(Some(Value::from(value)))
                            //     }
                            //     ElementType::ObjectId => {
                            //         let value = doc.get_object_id(field_name)?;
                            //         Ok(Some(Value::from(value.to_string())))
                            //     }
                            //     _ => unreachable!(),
                            // }
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
