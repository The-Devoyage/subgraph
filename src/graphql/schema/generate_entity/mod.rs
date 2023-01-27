use crate::{configuration::subgraph::SubGraphConfig, graphql::schema::ResolverType};

use super::ServiceSchema;
use async_graphql::dynamic::{Object, SchemaBuilder};
use log::info;

impl ServiceSchema {
    pub fn generate_entity(
        subgraph_config: SubGraphConfig,
        mut schema: SchemaBuilder,
        mut query: Object,
    ) -> (Object, SchemaBuilder) {
        for entity in subgraph_config.service.entities {
            info!("Including Entity, {}, in schema.", &entity.name);
            info!("Registering Entity Into Schema");
            schema = schema.register(ServiceSchema::generate_type(&entity));
            query = query.field(ServiceSchema::generate_resolver(
                &entity,
                ResolverType::FindOne,
            ));
            query = query.field(ServiceSchema::generate_resolver(
                &entity,
                ResolverType::CreateOne,
            ));
            // info!("Adding Entity Resolver");
            // schema = schema.entity_resolver(|_ctx| {
            //     FieldFuture::new(async move {
            //         Ok(Some(FieldValue::owned_any(graphql::user::User {
            //             // _id: ObjectId::new(),
            //             name: "nick".to_string(),
            //             // age: 33,
            //             // married: true,
            //         })))
            //     })
            // });
        }

        (query, schema)
    }
}
