use crate::configuration::subgraph::ServiceEntity;

use super::ServiceSchema;
use async_graphql::{
    dynamic::{Field, FieldFuture, Object, TypeRef},
    Value,
};
use bson::Document;
use log::{debug, info};

impl ServiceSchema {
    pub fn generate_type(entity: &ServiceEntity) -> Object {
        info!("Generating Type For {}", &entity.name);
        let mut generated_type = Object::new(&entity.name);
        debug!("{:?}", generated_type);

        for field in &entity.fields {
            info!("Entity Field Found");
            debug!("Adding Field, {:?}", field);

            generated_type = generated_type
                .field(Field::new(
                    &field.name,
                    TypeRef::named_nn(TypeRef::STRING),
                    |ctx| {
                        FieldFuture::new(async move {
                            info!("Resolving field");
                            let doc = ctx.parent_value.try_downcast_ref::<Document>()?;

                            info!("Field Restult Found");
                            debug!("{:?}", doc);

                            let value = doc.get(ctx.field().name());

                            info!("Found value.");
                            debug!("{:?}", value);

                            Ok(Some(Value::from("nick bongo".to_string())))
                        })
                    },
                ))
                .key(&field.name);
        }

        generated_type
    }
}
