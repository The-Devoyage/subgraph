use log::debug;
use serde::{Deserialize, Serialize};

use crate::graphql::schema::ResolverType;

use super::cors::MethodOption;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityResolverConfig {
    pub find_one: Option<ServiceEntityResolver>,
    pub find_many: Option<ServiceEntityResolver>,
    pub create_one: Option<ServiceEntityResolver>,
    pub update_one: Option<ServiceEntityResolver>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScalarOptions {
    String,
    Int,
    Boolean,
    ObjectID,
    Object,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryPair(pub String, pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityField {
    pub name: String,
    pub scalar: ScalarOptions,
    pub required: bool,
    pub exclude_from_input: Option<Vec<ResolverType>>,
    pub fields: Option<Vec<ServiceEntityField>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityDataSource {
    pub from: Option<String>,
    pub collection: Option<String>,
    pub path: Option<String>,
    pub search_query: Option<Vec<QueryPair>>,
    pub resolvers: Option<ServiceEntityResolverConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityResolver {
    pub fields: Option<Vec<ServiceEntityField>>,
    pub path: Option<String>,
    pub search_query: Option<Vec<QueryPair>>,
    pub http_method: Option<MethodOption>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntity {
    pub name: String,
    pub fields: Vec<ServiceEntityField>,
    pub data_source: Option<ServiceEntityDataSource>,
}

impl ServiceEntity {
    pub fn get_resolvers(service_entity: ServiceEntity) -> Option<ServiceEntityResolverConfig> {
        debug!("Get Resolvers From Service Entity: {:?}", service_entity);
        let data_source = service_entity.data_source;
        if data_source.is_some() {
            let resolvers = data_source.unwrap().resolvers;
            if resolvers.is_some() {
                let resolvers = resolvers.unwrap();
                debug!("Resolvers: {:?}", resolvers);
                return Some(resolvers);
            }
        }
        None
    }
}
