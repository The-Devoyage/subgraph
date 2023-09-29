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
        entity_required: bool,
    ) -> Result<Option<Value>, async_graphql::Error> {
        let field_name = ctx.field().name();

        debug!("Resolving Root Field: {:?}", &field_name);

        let value = match data_source {
            DataSource::Mongo(_ds) => {
                let doc = match ctx.parent_value.try_downcast_ref::<Option<Document>>() {
                    Ok(doc) => {
                        if let Some(doc) = doc {
                            let value = ServiceEntity::resolve_document_field(doc, entity_field)?;
                            Ok(Some(value))
                        } else {
                            if entity_required {
                                return Err(async_graphql::Error::new(
                                    "Failed to resolve root field.",
                                )
                                .extend_with(|_err, e| {
                                    e.set("field", field_name);
                                    e.set("entity", entity_field.name.clone());
                                }));
                            } else {
                                debug!("Parent value is null, returning null.");
                                if entity_required {
                                    return Err(async_graphql::Error::new(
                                        "Failed to resolve root field.",
                                    )
                                    .extend_with(|_err, e| {
                                        e.set("field", field_name);
                                        e.set("entity", entity_field.name.clone());
                                    }));
                                } else {
                                    debug!("Parent value is null, returning null.");
                                    Ok(Some(Value::Null))
                                }
                            }
                        }
                    }
                    Err(_) => {
                        if entity_required {
                            return Err(async_graphql::Error::new("Failed to resolve root field.")
                                .extend_with(|_err, e| {
                                    e.set("field", field_name);
                                    e.set("entity", entity_field.name.clone());
                                }));
                        } else {
                            debug!("Failed to downcast parent value, returning null.");
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
                            return Err(async_graphql::Error::new("Failed to resolve root field.")
                                .extend_with(|_err, e| {
                                    e.set("field", field_name);
                                    e.set("entity", entity_field.name.clone());
                                }));
                        } else {
                            return Ok(Some(Value::Null));
                        }
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
                let response_row = match ctx.parent_value.try_downcast_ref::<Option<ResponseRow>>()
                {
                    Ok(response_row) => {
                        if let Some(rr) = response_row {
                            let value = ServiceEntity::resolve_sql_field(
                                rr,
                                field_name,
                                entity_field.scalar.clone(),
                            )?;
                            Ok(Some(value))
                        } else {
                            if entity_required {
                                return Err(async_graphql::Error::new(
                                    "Failed to resolve root field.",
                                )
                                .extend_with(|_err, e| {
                                    e.set("field", field_name);
                                    e.set("entity", entity_field.name.clone());
                                }));
                            } else {
                                debug!("Parent value is null, returning null.");
                                Ok(Some(Value::Null))
                            }
                        }
                    }
                    Err(e) => {
                        debug!("Failed to downcast parent value: {:?}", e);
                        if entity_required {
                            return Err(async_graphql::Error::new("Failed to resolve root field.")
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

        debug!("Resolved Root Field: {:?}", value);

        value
    }
}
