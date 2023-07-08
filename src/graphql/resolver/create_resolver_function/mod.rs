use async_graphql::dynamic::{FieldFuture, ResolverContext};
use bson::Document;
use log::debug;

use crate::{
    data_sources::DataSources,
    graphql::schema::{ResolverType, ServiceSchemaBuilder},
};

use super::ServiceResolverBuilder;

impl ServiceResolverBuilder {
    pub fn create_resolver_function(
        &self,
    ) -> Box<(dyn for<'a> Fn(ResolverContext<'a>) -> FieldFuture<'a> + Send + Sync)> {
        debug!("Creating Resolver Function");
        let entity = self.entity.clone();
        let as_field = self.as_field.clone();
        let resolver_type = self.resolver_type.clone();

        Box::new(move |ctx: ResolverContext| {
            debug!("Resolving Field: {}", ctx.field().name());
            let entity = entity.clone();
            let as_field = as_field.clone();
            let resolver_type = resolver_type.clone();

            FieldFuture::new(async move {
                let data_sources = ctx.data_unchecked::<DataSources>().clone();
                let input_document = match resolver_type {
                    ResolverType::InternalType => ServiceSchemaBuilder::create_internal_input(
                        &ctx,
                        as_field.unwrap().clone(),
                    )?,
                    _ => {
                        let input = ctx.args.try_get(&format!("{}_input", ctx.field().name()))?;
                        let input_document = &input.deserialize::<Document>().unwrap();
                        input_document.clone()
                    }
                };

                let results = DataSources::execute(
                    &data_sources,
                    input_document,
                    entity,
                    resolver_type.clone(),
                )
                .await?;

                Ok(Some(results))
            })
        })
    }
}
