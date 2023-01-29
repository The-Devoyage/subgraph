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
            },
            false => match entity_field.scalar {
                ScalarOptions::String => TypeRef::named(TypeRef::STRING),
                ScalarOptions::Int => TypeRef::named(TypeRef::INT),
                ScalarOptions::Boolean => TypeRef::named(TypeRef::BOOLEAN),
            },
        };
        entity_field_type
    }

    pub fn add_entity_type(mut self, entity: &ServiceEntity) -> Self {
        info!("Generating Type For {}", &entity.name);
        let mut entity_type = Object::new(&entity.name);
        debug!("{:?}", entity_type);

        for entity_field in &entity.fields {
            info!("Entity Field Found");
            debug!("Adding Field, {:?}", entity_field);
            let entity_field_type =
                ServiceSchema::get_entity_field_resolver_field_type(entity_field).clone();
            entity_type = entity_type
                .field(Field::new(&entity_field.name, entity_field_type, |ctx| {
                    FieldFuture::new(async move {
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

                        match value.element_type() {
                            ElementType::String => {
                                let value = doc.get_str(field_name)?;
                                Ok(Some(Value::from(value)))
                            }
                            ElementType::Int32 => {
                                let value = doc.get_i32(field_name)?;
                                Ok(Some(Value::from(value)))
                            }
                            ElementType::Boolean => {
                                let value = doc.get_bool(field_name)?;
                                Ok(Some(Value::from(value)))
                            }
                            _ => unreachable!(),
                        }
                    })
                }))
                .key(&entity_field.name);
        }

        info!("Entity Fields Added.");
        debug!("{:?}", entity_type);

        self.schema_builder = self.schema_builder.register(entity_type);
        self
    }
}
