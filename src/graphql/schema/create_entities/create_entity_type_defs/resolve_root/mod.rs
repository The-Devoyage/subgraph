use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityField,
    data_sources::{sql::services::ResponseRow, DataSource},
    graphql::schema::ServiceSchemaBuilder,
};

use async_graphql::{dynamic::ResolverContext, ErrorExtensions, Value};
use bson::Document;
use json::JsonValue;
use log::debug;

impl ServiceSchemaBuilder {
    pub fn resolve_root(
        ctx: &ResolverContext,
        data_source: &DataSource,
        entity_field: &ServiceEntityField,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolving Root Field");

        let field_name = ctx.field().name();
        let value = match data_source {
            DataSource::Mongo(_ds) => {
                let doc = ctx.parent_value.try_downcast_ref::<Document>().unwrap();

                let value = ServiceSchemaBuilder::resolve_document_field(
                    doc,
                    field_name,
                    entity_field.scalar.clone(),
                    entity_field.list.unwrap_or(false),
                );

                let value = match value {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(
                            async_graphql::Error::new("Failed to resolve document field.")
                                .extend_with(|_err, e| {
                                    e.set("field", field_name);
                                }),
                        )
                    }
                };

                Ok(Some(value))
            }
            DataSource::HTTP(_ds) => {
                let json_value = ctx.parent_value.try_downcast_ref::<JsonValue>().unwrap();

                let value = ServiceSchemaBuilder::resolve_http_field(
                    json_value,
                    field_name,
                    entity_field.scalar.clone(),
                );

                Ok(Some(value.unwrap()))
            }
            DataSource::SQL(_ds) => {
                let response_row = ctx.parent_value.try_downcast_ref::<ResponseRow>().unwrap();

                let value = ServiceSchemaBuilder::resolve_sql_field(
                    response_row,
                    field_name,
                    entity_field.scalar.clone(),
                );

                Ok(Some(value.unwrap()))
            }
        };

        debug!("Resolved Root Field: {:?}", value);
        value
    }
}
