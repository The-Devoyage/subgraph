use async_graphql::dynamic::{Field, FieldFuture, FieldValue, TypeRef};
use bson::doc;
use log::{debug, info};

use crate::{
    configuration::subgraph::ServiceEntity,
    database::{data_source::DataSource, services::Services},
    graphql::schema::ResolverConfig,
};

use super::{ResolverType, ServiceSchema};

mod generate_resolver_input_value;

impl ServiceSchema {
    pub fn generate_resolver(entity: &ServiceEntity, resolver_type: ResolverType) -> Field {
        let resolver_config = match resolver_type {
            ResolverType::FindOne => ResolverConfig {
                resolver_name: format!("get_{}", &entity.name.to_lowercase()),
                return_type: TypeRef::named_nn(&entity.name),
            },
            ResolverType::CreateOne => ResolverConfig {
                resolver_name: format!("create_{}", &entity.name),
                return_type: TypeRef::named_nn(&entity.name),
            },
        };
        info!("Creating Resolver, {}.", resolver_config.return_type);
        debug!("{:?}", resolver_config);
        let mut field = Field::new(
            resolver_config.resolver_name,
            resolver_config.return_type,
            move |ctx| {
                FieldFuture::new(async move {
                    info!("Extracting Data Source From Context");
                    let db = ctx.data_unchecked::<DataSource>().db.clone();
                    info!("Creating Filter");
                    let _id = ctx.args.try_get("_id")?;
                    let filter = doc! {"_id": _id.string()?};
                    debug!("{:?}", filter);
                    info!("Using Find Service - Find One");
                    let result = match resolver_type {
                        ResolverType::FindOne => vec![Services::find_one(db, filter).await?],
                        ResolverType::CreateOne => Services::create(db, vec![filter]).await?,
                    };
                    debug!("{:?}", result);
                    Ok(Some(FieldValue::owned_any(result)))
                })
            },
        );
        field = ServiceSchema::generate_resolver_input_value(&entity, field, &resolver_type);
        field
    }
}
