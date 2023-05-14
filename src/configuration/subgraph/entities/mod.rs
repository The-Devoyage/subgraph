use bson::spec::ElementType;
use log::debug;
use serde::{Deserialize, Serialize};

use service_entity_field::ServiceEntityField;

use super::{cors::MethodOption, guard::Guard};
use crate::graphql::schema::ResolverType;

pub mod service_entity_field;

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
    pub guards: Option<Vec<Guard>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntity {
    pub name: String,
    pub fields: Vec<ServiceEntityField>,
    pub data_source: Option<ServiceEntityDataSource>,
    pub guards: Option<Vec<Guard>>,
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

    pub fn get_resolver(
        service_entity: &ServiceEntity,
        resolver_type: ResolverType,
    ) -> Option<ServiceEntityResolver> {
        debug!("Get Resolver: {:?}", resolver_type);
        let resolvers = ServiceEntity::get_resolvers(service_entity.clone());
        if resolvers.is_none() {
            return None;
        }
        let resolvers = resolvers.unwrap();
        match resolver_type {
            ResolverType::FindOne => {
                if resolvers.find_one.is_some() {
                    return resolvers.find_one;
                }
            }
            ResolverType::FindMany => {
                if resolvers.find_many.is_some() {
                    return resolvers.find_many;
                }
            }
            ResolverType::CreateOne => {
                if resolvers.create_one.is_some() {
                    return resolvers.create_one;
                }
            }
            ResolverType::UpdateOne => {
                if resolvers.update_one.is_some() {
                    return resolvers.update_one;
                }
            }
            ResolverType::UpdateMany => {
                if resolvers.update_many.is_some() {
                    return resolvers.update_many;
                }
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

    /// Returns vector of fields for a given entity.
    /// If field is nested, it returns all fields leading to the final field.
    pub fn get_fields_recursive(
        entity: &ServiceEntity,
        field_name: &str,
    ) -> Result<Vec<ServiceEntityField>, async_graphql::Error> {
        debug!("Get Field: {:?}", field_name);
        let entity_fields = &entity.fields;
        if field_name.contains(".") {
            debug!("Field is Nested");
            let mut fields = vec![];
            let mut field_names = ServiceEntityField::split_field_names(field_name)?;
            let first_field = ServiceEntity::get_field(entity, field_names[0])?;
            fields.push(first_field.clone());
            let nested_fields = first_field.fields;
            if nested_fields.is_none() {
                return Ok(fields);
            }
            field_names.remove(0);
            let rest_fields = ServiceEntityField::get_fields_recursive(
                nested_fields.unwrap(),
                field_names.join("."),
            )?;
            fields.extend(rest_fields);
            return Ok(fields);
        } else {
            for field in entity_fields {
                if field.name == field_name {
                    return Ok(vec![field.clone()]);
                }
            }
            Err(async_graphql::Error::new(format!(
                "Field {} not found in entity {}",
                field_name, entity.name
            )))
        }
    }

    /// Gets a field from a given entity.
    /// If field is nested, only returns nested field.
    pub fn get_field(
        entity: &ServiceEntity,
        field_name: &str,
    ) -> Result<ServiceEntityField, async_graphql::Error> {
        debug!("Get Field: {:?}", field_name);
        let entity_fields = &entity.fields;
        if field_name.contains(".") {
            debug!("Field is Nested");
            let mut field_names = ServiceEntityField::split_field_names(field_name)?;
            let first_field_name = field_names[0];
            let first_field = ServiceEntity::get_field(entity, first_field_name)?;
            let nested_fields = first_field.fields;
            if nested_fields.is_none() {
                return Err(async_graphql::Error::new(format!(
                    "Field {} is not a nested field",
                    field_name
                )));
            }
            field_names.remove(0);
            let field =
                ServiceEntityField::get_field(nested_fields.unwrap(), field_names.join("."))?;
            debug!("Found Field: {:?}", field);
            return Ok(field);
        } else {
            for field in entity_fields {
                if field.name == field_name {
                    return Ok(field.clone());
                }
            }
            Err(async_graphql::Error::new(format!(
                "Field {} not found",
                field_name
            )))
        }
    }
}
