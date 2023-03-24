use http::Method;
use log::{debug, info};

use crate::{configuration::subgraph::entities::ServiceEntity, graphql::schema::ResolverType};

use super::HttpDataSource;

impl HttpDataSource {
    pub fn get_method(entity: &ServiceEntity, resolver_type: ResolverType) -> Method {
        info!("Getting Method for Resolver Type: {:?}", resolver_type);
        let cloned_entity = entity.clone();
        let method = match resolver_type {
            ResolverType::FindOne => match cloned_entity.data_source {
                Some(ref data_source) => match data_source.resolvers {
                    Some(ref resolvers) => match resolvers.find_one {
                        Some(ref find_one) => match find_one.http_method {
                            Some(ref http_method) => http_method.method.to_string(),
                            None => "GET".to_string(),
                        },
                        None => "GET".to_string(),
                    },
                    None => "GET".to_string(),
                },
                None => "GET".to_string(),
            },
            ResolverType::FindMany => match cloned_entity.data_source {
                Some(ref data_source) => match data_source.resolvers {
                    Some(ref resolvers) => match resolvers.find_many {
                        Some(ref find_many) => match find_many.http_method {
                            Some(ref http_method) => http_method.method.to_string(),
                            None => "GET".to_string(),
                        },
                        None => "GET".to_string(),
                    },
                    None => "GET".to_string(),
                },
                None => "GET".to_string(),
            },
            ResolverType::CreateOne => match cloned_entity.data_source {
                Some(ref data_source) => match data_source.resolvers {
                    Some(ref resolvers) => match resolvers.create_one {
                        Some(ref create_one) => match create_one.http_method {
                            Some(ref http_method) => http_method.method.to_string(),
                            None => "POST".to_string(),
                        },
                        None => "POST".to_string(),
                    },
                    None => "POST".to_string(),
                },
                None => "POST".to_string(),
            },
            ResolverType::UpdateOne => match cloned_entity.data_source {
                Some(ref data_source) => match data_source.resolvers {
                    Some(ref resolvers) => match resolvers.update_one {
                        Some(ref update_one) => match update_one.http_method {
                            Some(ref http_method) => http_method.method.to_string(),
                            None => "PATCH".to_string(),
                        },
                        None => "GET".to_string(),
                    },
                    None => "GET".to_string(),
                },
                None => "GET".to_string(),
            },
        };

        debug!("Method: {:?}", method);

        match method.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            _ => Method::GET,
        }
    }
}
