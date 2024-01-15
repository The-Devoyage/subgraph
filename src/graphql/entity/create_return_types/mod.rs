use async_graphql::{
    dynamic::{Field, FieldFuture, FieldValue, Object, TypeRef},
    Value,
};
use bson::Document;
use json::JsonValue;
use log::{debug, error, trace};

use crate::{
    data_sources::{sql::services::ResponseRow, DataSource},
    resolver_type::ResolverType,
};

use super::ServiceEntity;

pub struct ResolverResponseMeta {
    pub request_id: String,
    pub count: i64,
    pub total_count: i64,
    pub page: i64,
    pub total_pages: i64,
    pub service_name: String,
    pub executed_at: String,
    pub service_version: Option<String>,
    pub user_uuid: Option<String>,
}

pub struct ResolverResponse<'a> {
    pub data: Vec<FieldValue<'a>>,
    pub meta: ResolverResponseMeta,
}

impl ServiceEntity {
    pub fn create_return_types(&self) -> Vec<Object> {
        debug!("Creating Return Types For: `{}`", &self.type_name);
        let mut return_types = Vec::new();
        if !self.is_root {
            return return_types;
        }
        let resolver_types = ResolverType::get_resolver_types();
        let meta_type = ServiceEntity::create_resolver_meta_return_type();
        return_types.push(meta_type);

        // For each resolver type, create a response type and push it to the return types to
        // register.
        for resolver_type in resolver_types {
            match resolver_type {
                ResolverType::InternalType => continue,
                _ => {
                    let return_type = Object::new(format!(
                        "{}_{}_response",
                        resolver_type.to_string().to_lowercase(),
                        self.entity.name
                    ));
                    let data_source = self.data_source.clone();

                    // Add the "data" field to the return type
                    let return_type = match resolver_type {
                        ResolverType::FindOne
                        | ResolverType::UpdateOne
                        | ResolverType::CreateOne => return_type.field(Field::new(
                            "data",
                            match &self.entity.required.unwrap_or(false) {
                                false => TypeRef::named(&self.entity.name.clone()),
                                true => TypeRef::named_nn(&self.entity.name.clone()),
                            },
                            move |ctx| {
                                let data_source = data_source.clone();
                                FieldFuture::new(async move {
                                    let res =
                                        ctx.parent_value.try_downcast_ref::<ResolverResponse>()?;

                                    // If the DS is a SQL DS, then we need to downcast to ResponseRow
                                    if let DataSource::SQL(_sql_ds) = data_source.clone() {
                                        // Return the first value in the data array.
                                        for v in res.data.iter() {
                                            let v = v
                                                .try_downcast_ref::<Option<ResponseRow>>()
                                                .map_err(|_| {
                                                    error!("Failed to downcast to ResponseRow");
                                                    async_graphql::Error::new(
                                                        "Failed to downcast to ResponseRow",
                                                    )
                                                })?;
                                            let fv = FieldValue::borrowed_any(v);
                                            return Ok(Some(fv));
                                        }
                                    }

                                    // If the DS is a Mongo DS, then we need to downcast to Document
                                    if let DataSource::Mongo(_mongo_ds) = data_source.clone() {
                                        // Return the first value in the data array.
                                        for v in res.data.iter() {
                                            let v = v
                                                .try_downcast_ref::<Option<Document>>()
                                                .map_err(|_| {
                                                    error!("Failed to downcast to Document");
                                                    async_graphql::Error::new(
                                                        "Failed to downcast to Document",
                                                    )
                                                })?;
                                            let fv = FieldValue::borrowed_any(v);
                                            return Ok(Some(fv));
                                        }
                                    }

                                    if let DataSource::HTTP(_http_ds) = data_source.clone() {
                                        // Return the first value in the data array.
                                        for v in res.data.iter() {
                                            let v = v.try_downcast_ref::<JsonValue>().map_err(
                                                |_| {
                                                    error!("Failed to downcast to JsonValue");
                                                    async_graphql::Error::new(
                                                        "Failed to downcast to JsonValue",
                                                    )
                                                },
                                            )?;
                                            let fv = FieldValue::borrowed_any(v);
                                            return Ok(Some(fv));
                                        }
                                    }

                                    // If we can't downcast to the correct type, then return None.
                                    Ok(Some(FieldValue::NULL))
                                })
                            },
                        )),
                        ResolverType::FindMany | ResolverType::UpdateMany => {
                            return_type.field(Field::new(
                                "data",
                                match &self.entity.required.unwrap_or(false) {
                                    false => TypeRef::named_list_nn(&self.entity.name.clone()),
                                    true => TypeRef::named_nn_list_nn(&self.entity.name.clone()),
                                },
                                move |ctx| {
                                    let data_source = data_source.clone();
                                    FieldFuture::new(async move {
                                        let res = ctx
                                            .parent_value
                                            .try_downcast_ref::<ResolverResponse>()?;

                                        // If the DS is a SQL DS, then we need to downcast to ResponseRow
                                        if let DataSource::SQL(_sql_ds) = data_source.clone() {
                                            let data = res.data.iter().map(|v| {
                                                let v = v
                                                    .try_downcast_ref::<Option<ResponseRow>>()
                                                    .unwrap(); // Should be safe to unwrap.
                                                let fv = FieldValue::borrowed_any(v);
                                                fv
                                            });

                                            return Ok(Some(FieldValue::list(data)));
                                        }

                                        // If the DS is a Mongo DS, then we need to downcast to Document
                                        if let DataSource::Mongo(_mongo_ds) = data_source.clone() {
                                            let data = res.data.iter().map(|v| {
                                                let v = v
                                                    .try_downcast_ref::<Option<Document>>()
                                                    .unwrap(); // Should be safe to unwrap.
                                                let fv = FieldValue::borrowed_any(v);
                                                fv
                                            });

                                            return Ok(Some(FieldValue::list(data)));
                                        }

                                        // If the DS is a HTTP DS, then we need to downcast to serde_json::Value
                                        if let DataSource::HTTP(_http_ds) = data_source.clone() {
                                            let data = res.data.iter().map(|v| {
                                                let v = v.try_downcast_ref::<JsonValue>().unwrap(); // Should be safe to unwrap.
                                                let fv = FieldValue::borrowed_any(v);
                                                fv
                                            });

                                            return Ok(Some(FieldValue::list(data)));
                                        }

                                        // If we can't downcast to the correct type, then return
                                        // empty list.
                                        Ok(Some(FieldValue::NULL))
                                    })
                                },
                            ))
                        }
                        ResolverType::InternalType => return_type.field(Field::new(
                            "data",
                            match &self.entity.required.unwrap_or(false) {
                                false => TypeRef::named(&self.entity.name.clone()),
                                true => TypeRef::named_nn(&self.entity.name.clone()),
                            },
                            move |ctx| {
                                let data_source = data_source.clone();
                                FieldFuture::new(async move {
                                    let res =
                                        ctx.parent_value.try_downcast_ref::<ResolverResponse>()?;

                                    //TODO: Need to determine if resolving a list or not.

                                    // If the DS is a SQL DS, then we need to downcast to ResponseRow
                                    if let DataSource::SQL(_sql_ds) = data_source.clone() {
                                        // Return the first value in the data array.
                                        for v in res.data.iter() {
                                            let v = v
                                                .try_downcast_ref::<Option<ResponseRow>>()
                                                .unwrap(); // Should be safe to unwrap.
                                            let fv = FieldValue::borrowed_any(v);
                                            return Ok(Some(fv));
                                        }
                                    }

                                    // If the DS is a Mongo DS, then we need to downcast to Document
                                    if let DataSource::Mongo(_mongo_ds) = data_source.clone() {
                                        // Return the first value in the data array.
                                        for v in res.data.iter() {
                                            let v =
                                                v.try_downcast_ref::<Option<Document>>().unwrap(); // Should be safe to unwrap.
                                            let fv = FieldValue::borrowed_any(v);
                                            return Ok(Some(fv));
                                        }
                                    }

                                    if let DataSource::HTTP(_http_ds) = data_source.clone() {
                                        // Return the first value in the data array.
                                        for v in res.data.iter() {
                                            let v = v
                                                .try_downcast_ref::<Option<serde_json::Value>>()
                                                .unwrap(); // Should be safe to unwrap.
                                            let fv = FieldValue::borrowed_any(v);
                                            return Ok(Some(fv));
                                        }
                                    }

                                    // If we can't downcast to the correct type, then return None.
                                    Ok(Some(FieldValue::NULL))
                                })
                            },
                        )),
                    };

                    // Add the "meta" field to the return type
                    let return_type =
                        return_type.field(Field::new("meta", TypeRef::named("meta"), move |ctx| {
                            FieldFuture::new(async move {
                                let res =
                                    ctx.parent_value.try_downcast_ref::<ResolverResponse>()?;
                                let meta = FieldValue::borrowed_any(&res.meta);

                                Ok(Some(meta))
                            })
                        }));

                    return_types.push(return_type);
                }
            }
        }

        trace!("Return Types: {:?}", &return_types);
        return_types
    }

    pub fn create_resolver_meta_return_type() -> Object {
        let mut meta_return_type = Object::new("meta");

        meta_return_type = meta_return_type.field(Field::new(
            "request_id",
            TypeRef::named_nn(TypeRef::STRING),
            move |ctx| {
                FieldFuture::new(async move {
                    let meta = ctx
                        .parent_value
                        .try_downcast_ref::<ResolverResponseMeta>()
                        .map_err(|e| {
                            error!("Error: {:?}", e);
                            async_graphql::Error::new("Error getting meta request_id.");
                            e
                        })?;
                    let request_id = meta.request_id.clone();

                    Ok(Some(Value::from(request_id)))
                })
            },
        ));

        meta_return_type = meta_return_type.field(Field::new(
            "count",
            TypeRef::named_nn(TypeRef::INT),
            move |ctx| {
                FieldFuture::new(async move {
                    let meta = ctx
                        .parent_value
                        .try_downcast_ref::<ResolverResponseMeta>()
                        .map_err(|e| {
                            error!("Error: {:?}", e);
                            async_graphql::Error::new("Error getting meta count.");
                            e
                        })?;
                    let count = meta.count;

                    Ok(Some(Value::from(count)))
                })
            },
        ));

        meta_return_type = meta_return_type.field(Field::new(
            "total_count",
            TypeRef::named_nn(TypeRef::INT),
            move |ctx| {
                FieldFuture::new(async move {
                    let meta = ctx
                        .parent_value
                        .try_downcast_ref::<ResolverResponseMeta>()
                        .map_err(|e| {
                            error!("Error: {:?}", e);
                            async_graphql::Error::new("Error getting meta total.");
                            e
                        })?;
                    let total_count = meta.total_count;

                    Ok(Some(Value::from(total_count)))
                })
            },
        ));

        meta_return_type = meta_return_type.field(Field::new(
            "page",
            TypeRef::named_nn(TypeRef::INT),
            move |ctx| {
                FieldFuture::new(async move {
                    let meta = ctx
                        .parent_value
                        .try_downcast_ref::<ResolverResponseMeta>()
                        .map_err(|e| {
                            error!("Error: {:?}", e);
                            async_graphql::Error::new("Error getting meta page.");
                            e
                        })?;
                    let page = meta.page;

                    Ok(Some(Value::from(page)))
                })
            },
        ));

        meta_return_type = meta_return_type.field(Field::new(
            "total_pages",
            TypeRef::named_nn(TypeRef::INT),
            move |ctx| {
                FieldFuture::new(async move {
                    let meta = ctx
                        .parent_value
                        .try_downcast_ref::<ResolverResponseMeta>()
                        .map_err(|e| {
                            error!("Error: {:?}", e);
                            async_graphql::Error::new("Error getting meta page_size.");
                            e
                        })?;
                    let total_pages = meta.total_pages;

                    Ok(Some(Value::from(total_pages)))
                })
            },
        ));

        meta_return_type = meta_return_type.field(Field::new(
            "service_name",
            TypeRef::named_nn(TypeRef::STRING),
            move |ctx| {
                FieldFuture::new(async move {
                    let meta = ctx
                        .parent_value
                        .try_downcast_ref::<ResolverResponseMeta>()
                        .map_err(|e| {
                            error!("Error: {:?}", e);
                            async_graphql::Error::new("Error getting meta service_name.");
                            e
                        })?;
                    let service_name = meta.service_name.clone();

                    Ok(Some(Value::from(service_name)))
                })
            },
        ));

        meta_return_type = meta_return_type.field(Field::new(
            "executed_at",
            TypeRef::named_nn(TypeRef::STRING),
            move |ctx| {
                FieldFuture::new(async move {
                    let meta = ctx
                        .parent_value
                        .try_downcast_ref::<ResolverResponseMeta>()
                        .map_err(|e| {
                            error!("Error: {:?}", e);
                            async_graphql::Error::new("Error getting meta executed_at.");
                            e
                        })?;
                    let executed_at = meta.executed_at.clone();

                    Ok(Some(Value::from(executed_at)))
                })
            },
        ));

        meta_return_type = meta_return_type.field(Field::new(
            "service_version",
            TypeRef::named(TypeRef::STRING),
            move |ctx| {
                FieldFuture::new(async move {
                    let meta = ctx
                        .parent_value
                        .try_downcast_ref::<ResolverResponseMeta>()
                        .map_err(|e| {
                            error!("Error: {:?}", e);
                            async_graphql::Error::new("Error getting meta service_version.");
                            e
                        })?;
                    let service_version = meta.service_version.clone();

                    match service_version {
                        None => Ok(None),
                        Some(service_version) => Ok(Some(Value::from(service_version))),
                    }
                })
            },
        ));

        meta_return_type = meta_return_type.field(Field::new(
            "user_uuid",
            TypeRef::named(TypeRef::STRING),
            move |ctx| {
                FieldFuture::new(async move {
                    let meta = ctx
                        .parent_value
                        .try_downcast_ref::<ResolverResponseMeta>()
                        .map_err(|e| {
                            error!("Error: {:?}", e);
                            async_graphql::Error::new("Error getting meta user_uuid.");
                            e
                        })?;
                    let user_uuid = meta.user_uuid.clone();

                    match user_uuid {
                        None => Ok(None),
                        Some(user_uuid) => Ok(Some(Value::from(user_uuid))),
                    }
                })
            },
        ));

        meta_return_type
    }
}
