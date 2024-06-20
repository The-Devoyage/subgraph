use async_graphql::{dynamic::ResolverContext, ErrorExtensions, Value};
use bson::Document;
use json::JsonValue;
use log::{debug, error, trace};

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    data_sources::{sql::services::ResponseRow, DataSource},
    graphql::entity::ServiceEntity,
    traits::async_graphql::FromJson,
};

impl ServiceEntity {
    pub fn resolve_root(
        ctx: &ResolverContext,
        data_source: &DataSource,
        entity_field: &ServiceEntityFieldConfig,
        entity_required: bool,
    ) -> Result<Option<Value>, async_graphql::Error> {
        debug!("Resolve Root Field");
        let field_name = ctx.field().name();
        trace!("Field Name: {:?}", field_name);

        let value = match data_source {
            DataSource::Mongo(_ds) => {
                let doc = match ctx.parent_value.try_downcast_ref::<Option<Document>>() {
                    Ok(doc) => {
                        trace!("Parent Value: {:?}", doc);
                        if let Some(doc) = doc {
                            trace!("Document Matched: {:?}", doc);
                            let value = entity_field
                                .scalar
                                .clone()
                                .document_field_to_async_graphql_value(doc, entity_field)?;
                            trace!("Resolved Value: {:?}", value);
                            Ok(value)
                        } else {
                            if entity_required {
                                error!(
                                    "Failed to resolve root field. Entity is marked as required. This may be due to a join on a required entity."
                                );
                                return Err(async_graphql::Error::new(
                                    "Failed to resolve root field. Entity is marked as required. This may be due to a join on a required entity.",
                                )
                                .extend_with(|_err, e| {
                                    e.set("field", field_name);
                                    e.set("entity", entity_field.name.clone());
                                }));
                            } else {
                                trace!("Parent value is null, returning null.");
                                if entity_required {
                                    error!("Failed to resolve root field.");
                                    return Err(async_graphql::Error::new(
                                        "Failed to resolve root field.",
                                    )
                                    .extend_with(|_err, e| {
                                        e.set("field", field_name);
                                        e.set("entity", entity_field.name.clone());
                                    }));
                                } else {
                                    trace!("Parent value is null, returning null.");
                                    Ok(Some(Value::Null))
                                }
                            }
                        }
                    }
                    Err(_) => {
                        if entity_required {
                            error!("Failed to resolve root field. Entity is marked as required. This may be due to a join on a required entity.");
                            return Err(async_graphql::Error::new("Failed to resolve root field. Entity is marked as required. This may be due to a join on a required entity.")
                                .extend_with(|_err, e| {
                                    e.set("field", field_name);
                                    e.set("entity", entity_field.name.clone());
                                }));
                        } else {
                            trace!("Failed to downcast parent value, returning null.");
                            return Ok(Some(Value::Null));
                        }
                    }
                };

                doc
            }
            DataSource::HTTP(_ds) => {
                let json_value = match ctx.parent_value.try_downcast_ref::<JsonValue>() {
                    Ok(json_value) => json_value,
                    Err(_) => {
                        if entity_required {
                            error!("Failed to resolve root field. Entity is marked as required. This may be due to a join on a required entity.");
                            return Err(async_graphql::Error::new("Failed to resolve root field. Entity is marked as required. This may be due to a join on a required entity.")
                                .extend_with(|_err, e| {
                                    e.set("field", field_name);
                                    e.set("entity", entity_field.name.clone());
                                }));
                        } else {
                            return Ok(Some(Value::Null));
                        }
                    }
                };

                let value = json_value.to_async_graphql_value();

                Ok(Some(value))
            }
            DataSource::SQL(_ds) => {
                let response_row = match ctx.parent_value.try_downcast_ref::<Option<ResponseRow>>()
                {
                    Ok(response_row) => {
                        if let Some(rr) = response_row {
                            let value = entity_field
                                .scalar
                                .clone()
                                .rr_to_async_graphql_value(rr, field_name)?;
                            Ok(Some(value))
                        } else {
                            if entity_required {
                                error!(
                                    "Failed to resolve root field. Entity is marked as required. This may be due to a join on a required entity"
                                );
                                return Err(async_graphql::Error::new(
                                    "Failed to resolve root field. Entity is marked as required. This may be due to a join on a required entity.",
                                )
                                .extend_with(|_err, e| {
                                    e.set("field", field_name);
                                    e.set("entity", entity_field.name.clone());
                                }));
                            } else {
                                trace!("Parent value is null, returning null.");
                                Ok(Some(Value::Null))
                            }
                        }
                    }
                    Err(e) => {
                        trace!("Failed to downcast parent value: {:?}", e);
                        if entity_required {
                            error!(
                                "Failed to resolve root field, `{}`. Entity is marked as required. This may be due to a join on a required entity.",
                                entity_field.name
                            );
                            return Err(async_graphql::Error::new(format!(
                                "Failed to resolve root field, `{}`. The entity is marked as required. This may be due to a join on a required entity.",
                                entity_field.name
                            ))
                            .extend_with(|_err, e| {
                                e.set("field", field_name);
                                e.set("entity", entity_field.name.clone());
                            }));
                        } else {
                            return Ok(Some(Value::Null));
                        }
                    }
                };
                response_row
            }
        };

        trace!("Resolved Root Field: {:?}", value);

        value
    }
}
