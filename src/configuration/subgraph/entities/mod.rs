use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceEntityResolver {
    FindOne(ServiceEntityResolverOptions),
    FindMany(ServiceEntityResolverOptions),
    CreateOne(ServiceEntityResolverOptions),
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityDataSource {
    pub from: Option<String>,
    pub collection: Option<String>,
    pub path: Option<String>,
    pub search_query: Option<Vec<QueryPair>>,
    pub resolvers: Option<Vec<ServiceEntityResolver>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityResolverOptions {
    pub fields: Option<Vec<ServiceEntityFieldOptions>>,
    pub path: Option<String>,
    pub search_query: Option<Vec<QueryPair>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntity {
    pub name: String,
    pub fields: Vec<ServiceEntityFieldOptions>,
    pub data_source: Option<ServiceEntityDataSource>,
}
