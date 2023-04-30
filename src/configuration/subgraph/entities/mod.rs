use bson::spec::ElementType;
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
    pub update_many: Option<ServiceEntityResolver>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScalarOptions {
    String,
    Int,
    Boolean,
    ObjectID,
    Object,
}

impl ScalarOptions {
    pub fn to_bson_type(self) -> ElementType {
        debug!("Converting Scalar To BSON Element Type: {:?}", self);
        match self {
            ScalarOptions::String => ElementType::String,
            ScalarOptions::Int => ElementType::Int32,
            ScalarOptions::Boolean => ElementType::Boolean,
            ScalarOptions::ObjectID => ElementType::ObjectId,
            ScalarOptions::Object => ElementType::EmbeddedDocument,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryPair(pub String, pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityField {
    pub name: String,
    pub scalar: ScalarOptions,
    pub required: Option<bool>,
    pub exclude_from_input: Option<Vec<ResolverType>>,
    pub exclude_from_output: Option<bool>,
    pub fields: Option<Vec<ServiceEntityField>>,
    pub list: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityDataSource {
    pub from: Option<String>,
    pub collection: Option<String>,
    pub table: Option<String>,
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

    pub fn get_entity_data_source(
        service_entity: &ServiceEntity,
    ) -> Option<ServiceEntityDataSource> {
        debug!("Get Data Source From Service Entity: {:?}", service_entity);
        let data_source = &service_entity.data_source;
        if data_source.is_some() {
            let data_source = data_source.clone().unwrap();
            debug!("Data Source: {:?}", data_source);
            return Some(data_source.clone());
        }
        None
    }

    pub fn get_mongo_collection_name(entity: &ServiceEntity) -> String {
        debug!("Found Entity Data Source: {:?}", entity.data_source);
        let data_source = ServiceEntity::get_entity_data_source(entity);
        if data_source.is_none() {
            return entity.name.clone();
        }
        let collection = data_source.unwrap().collection;
        if collection.is_none() {
            return entity.name.clone();
        }
        collection.unwrap()
    }

    //TODO: How to tell if two names are the same?
    pub fn get_field(entity: &ServiceEntity, field_name: &str) -> Option<ServiceEntityField> {
        debug!("Get Field: {:?}", field_name);
        let fields = &entity.fields;
        for field in fields {
            if field.name == field_name {
                return Some(field.clone());
            }
        }
        None
    }
}
