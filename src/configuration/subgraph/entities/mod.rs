use serde::{Deserialize, Serialize};

use crate::graphql::schema::ResolverType;

use super::cors::MethodOption;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityResolverConfig {
    pub find_one: Option<ServiceEntityResolverOptions>,
    pub find_many: Option<ServiceEntityResolverOptions>,
    pub create_one: Option<ServiceEntityResolverOptions>,
    pub update_one: Option<ServiceEntityResolverOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScalarOptions {
    String,
    Int,
    Boolean,
    ObjectID,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryPair(pub String, pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityFieldOptions {
    pub name: String,
    pub scalar: ScalarOptions,
    pub required: bool,
    pub exclude_from_input: Option<Vec<ResolverType>>,
    pub exclude_from_output: Option<Vec<ResolverType>>,
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
pub struct ServiceEntityResolverOptions {
    pub fields: Option<Vec<ServiceEntityFieldOptions>>,
    pub path: Option<String>,
    pub search_query: Option<Vec<QueryPair>>,
    pub http_method: Option<MethodOption>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntity {
    pub name: String,
    pub fields: Vec<ServiceEntityFieldOptions>,
    pub data_source: Option<ServiceEntityDataSource>,
}
