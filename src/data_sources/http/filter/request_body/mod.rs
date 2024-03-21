use async_graphql::Json;
use bson::Document;
use log::debug;

use crate::{data_sources::http::HttpDataSource, resolver_type::ResolverType};

impl HttpDataSource {
    pub fn create_body_filters(
        mut input: Document,
        resolver_type: ResolverType,
    ) -> Option<Json<Document>> {
        debug!("Create Body Filters");

        match resolver_type {
            ResolverType::CreateOne => {
                let values = input.get("values").unwrap().as_document().unwrap().clone();
                let json = Json::from(values);
                Some(json)
            }
            ResolverType::FindOne | ResolverType::FindMany => None,
            ResolverType::UpdateOne | ResolverType::UpdateMany => {
                debug!("Update One Resolver Filters");
                input.remove("query");
                let json = Json::from(input);
                Some(json)
            }
            _ => panic!("Invalid resolver type"),
        }
    }
}
