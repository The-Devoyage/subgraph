use async_graphql::{dynamic::ResolverContext, ErrorExtensions, Value};
use bson::Document;
use json::JsonValue;
use log::debug;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    data_sources::{sql::services::ResponseRow, DataSource},
    graphql::entity::ServiceEntity,
};

impl ServiceEntity {
    pub fn resolve_root(
        ctx: &ResolverContext,
        data_source: &DataSource,
        entity_field: &ServiceEntityFieldConfig,
    ) -> Result<Option<Value>, async_graphql::Error> {
        let field_name = ctx.field().name();

        debug!("Resolving Root Field: {:?}", &field_name);

        let value = match data_source {
            DataSource::Mongo(_ds) => {
                let doc = match ctx.parent_value.try_downcast_ref::<Document>() {
                    Ok(doc) => doc,
                    Err(_) => {
                        return Err(async_graphql::Error::new("Failed to resolve root field.")
                            .extend_with(|_err, e| {
                                e.set("field", field_name);
                            }))
                    }
                };

                let value = ServiceEntity::resolve_document_field(doc, entity_field)?;

                Ok(Some(value))
            }
            DataSource::HTTP(_ds) => {
                let json_value = match ctx.parent_value.try_downcast_ref::<JsonValue>() {
                    Ok(json_value) => json_value,
                    Err(_) => {
                        return Err(async_graphql::Error::new("Failed to resolve root field.")
                            .extend_with(|_err, e| {
                                e.set("field", field_name);
                            }))
                    }
                };

                let value = ServiceEntity::resolve_http_field(
                    json_value,
                    field_name,
                    entity_field.scalar.clone(),
                )?;

                Ok(Some(value))
            }
            DataSource::SQL(_ds) => {
                let response_row = match ctx.parent_value.try_downcast_ref::<ResponseRow>() {
                    Ok(response_row) => response_row,
                    Err(_) => {
                        return Err(async_graphql::Error::new("Failed to resolve root field.")
                            .extend_with(|_err, e| {
                                e.set("field", field_name);
                            }))
                    }
                };

                let value = ServiceEntity::resolve_sql_field(
                    response_row,
                    field_name,
                    entity_field.scalar.clone(),
                )?;

                Ok(Some(value))
            }
        };

        debug!("Resolved Root Field: {:?}", value);

        value
    }
}
