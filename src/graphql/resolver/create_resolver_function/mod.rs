use async_graphql::dynamic::{FieldFuture, ResolverContext};
use log::debug;

use crate::data_sources::DataSources;

use super::ServiceResolver;

impl ServiceResolver {
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
                let input_document =
                    ServiceResolver::get_resolver_input(&ctx, &as_field, &resolver_type)?;

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
