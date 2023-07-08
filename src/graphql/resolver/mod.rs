use async_graphql::dynamic::{Field, FieldFuture, ResolverContext, TypeRef};
use log::debug;

use crate::configuration::subgraph::entities::{
    service_entity_field::ServiceEntityField, ServiceEntity,
};

use super::schema::ResolverType;

mod create_resolver_function;
mod create_resolver_name;
mod get_resolver_type_ref;

pub struct ServiceResolverBuilder {
    name: String,
    resolver_type: ResolverType,
    entity: ServiceEntity,
    as_field: Option<ServiceEntityField>,
    type_ref: TypeRef,
    resolver_function: Box<(dyn for<'a> Fn(ResolverContext<'a>) -> FieldFuture<'a> + Send + Sync)>,
}

impl ServiceResolverBuilder {
    pub fn new(
        &self,
        resolver_type: ResolverType,
        entity: ServiceEntity,
        as_field: Option<ServiceEntityField>,
    ) -> Self {
        debug!("Creating Service Resolver Builder");
        Self {
            name: self.create_resolver_name(),
            resolver_type,
            entity,
            as_field,
            type_ref: self.get_resolver_type_ref(),
            resolver_function: self.create_resolver_function(),
        }
    }

    pub fn build(self) -> Self {
        self
    }
}

pub fn create_resolver(service_resolver_builder: ServiceResolverBuilder) -> Field {
    debug!("Creating Service Resolver");

    let resolver = Field::new(
        service_resolver_builder.name,
        service_resolver_builder.type_ref,
        service_resolver_builder.resolver_function,
    );

    debug!("Created Service Resolver: {:?}", resolver);
    resolver
}
