use crate::graphql::schema::ResolverType;

use super::ServiceSchema;
use log::info;

mod generate_resolver;

impl ServiceSchema {
    pub fn generate_entities(mut self) -> Self {
        for entity in self.subgraph_config.service.entities.clone() {
            info!("Including Entity, {}, in schema.", &entity.name);
            info!("Registering Entity Into Schema");

            info!("Adding Resolvers");
            self = self.add_resolver(&entity, ResolverType::FindOne);
            self = self.add_resolver(&entity, ResolverType::CreateOne);

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
        self
    }
}
