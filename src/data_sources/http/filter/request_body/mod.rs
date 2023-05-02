use async_graphql::{dynamic::ValueAccessor, Json};
use bson::Document;
use log::debug;

use crate::{data_sources::http::HttpDataSource, graphql::schema::ResolverType};

impl HttpDataSource {
    pub fn create_body_filters(
        input: &ValueAccessor<'_>,
        resolver_type: ResolverType,
    ) -> Option<Json<Document>> {
        debug!("Create Body Filters");

        match resolver_type {
            ResolverType::CreateOne => {
                let document = input.deserialize::<Document>().unwrap();
                debug!("Deserialized Document: {:#?}", document);

                let json = Json::from(document);
                Some(json)
            }
            ResolverType::FindOne | ResolverType::FindMany => None,
            ResolverType::UpdateOne | ResolverType::UpdateMany => {
                debug!("Update One Resolver Filters");
                let mut document = input.deserialize::<Document>().unwrap();
                document.remove("query");
                let json = Json::from(document);
                Some(json)
            }
        }
    }
}
