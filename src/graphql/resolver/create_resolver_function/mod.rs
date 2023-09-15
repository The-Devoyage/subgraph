use async_graphql::dynamic::{FieldFuture, ResolverContext};
use biscuit_auth::{Biscuit, KeyPair};
use http::HeaderMap;
use log::debug;

use crate::{
    data_sources::DataSources, graphql::schema::create_entities::create_auth_service::TokenData,
};

use super::ServiceResolver;

mod get_operation_type;
mod get_resolver_input;
mod guard_resolver;

impl ServiceResolver {
    pub fn create_resolver_function(
        &self,
    ) -> Box<(dyn for<'a> Fn(ResolverContext<'a>) -> FieldFuture<'a> + Send + Sync)> {
        debug!("Creating Resolver Function");
        let entity = self.entity.clone();
        let as_field = self.as_field.clone();
        let resolver_type = self.resolver_type.clone();
        let service_guards = self.subgraph_config.service.guards.clone();
        let is_auth = self.subgraph_config.service.auth.is_some();

        Box::new(move |ctx: ResolverContext| {
            debug!("Resolving Field: {}", ctx.field().name());
            let entity = entity.clone();
            let as_field = as_field.clone();
            let resolver_type = resolver_type.clone();
            let service_guards = service_guards.clone();
            let is_auth = is_auth.clone();

            FieldFuture::new(async move {
                let data_sources = ctx.data_unchecked::<DataSources>().clone();
                let key_pair = match ctx.data_unchecked::<Option<KeyPair>>() {
                    Some(key_pair) => key_pair,
                    None => {
                        return Err(async_graphql::Error::new(format!(
                            "Failed to get key pair."
                        )));
                    }
                };
                let headers = ctx.data_unchecked::<HeaderMap>().clone();

                let token_data = if is_auth.clone() {
                    let public_key = key_pair.public();
                    let biscuit_base64 = headers.get("Authorization");
                    let biscuit_base64 = match biscuit_base64 {
                        Some(biscuit_base64) => biscuit_base64,
                        None => {
                            return Err(async_graphql::Error::new(format!(
                                "Failed to get biscuit from headers."
                            )));
                        }
                    };
                    //TODO: Provide a typed error that frontend can react to.
                    let biscuit =
                        Biscuit::from_base64(biscuit_base64, public_key).map_err(|e| {
                            async_graphql::Error::new(format!("Failed to parse biscuit: {:?}", e))
                        })?;

                    debug!("Biscuit: {:?}", biscuit);

                    let mut authorizier = biscuit.authorizer().map_err(|e| {
                        async_graphql::Error::new(format!("Failed to get authorizer: {:?}", e))
                    })?;

                    let token_data: Vec<(String, i64)> = authorizier
                        .query("data($identifier, $user_id) <- user($identifier, $user_id)")
                        .map_err(|e| {
                            async_graphql::Error::new(format!("Failed to query biscuit: {:?}", e))
                        })?;

                    debug!("TokenData: {:?}", token_data);

                    // biscuit.authorize(&authorizier).map_err(|e| {
                    //     async_graphql::Error::new(format!("Failed to authorize: {:?}", e))
                    // })?;

                    let token_data = TokenData {
                        identifier: token_data[0].0.clone(),
                        user_id: token_data[0].1.clone(),
                    };

                    Some(token_data)
                } else {
                    None
                };

                let input_document =
                    ServiceResolver::get_resolver_input(&ctx, &as_field, &resolver_type)?;

                ServiceResolver::guard_resolver(
                    &ctx,
                    &input_document,
                    &entity,
                    service_guards.clone(),
                    &resolver_type,
                    token_data,
                )?;

                let operation_type = ServiceResolver::get_operation_type(&resolver_type, &as_field);

                let results =
                    DataSources::execute(&data_sources, input_document, entity, operation_type)
                        .await?;

                Ok(Some(results))
            })
        })
    }
}
