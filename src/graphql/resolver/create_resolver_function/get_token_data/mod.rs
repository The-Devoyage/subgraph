use async_graphql::dynamic::ResolverContext;
use biscuit_auth::{Biscuit, KeyPair};
use http::HeaderMap;
use log::debug;

use crate::graphql::{
    resolver::ServiceResolver, schema::create_entities::create_auth_service::TokenData,
};

impl ServiceResolver {
    pub fn get_token_data(
        ctx: &ResolverContext,
        headers: HeaderMap,
    ) -> Result<Option<TokenData>, async_graphql::Error> {
        let key_pair = ctx.data_unchecked::<Option<KeyPair>>();
        let token_data = if key_pair.is_some() {
            let public_key = key_pair.as_ref().unwrap().public();
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
            let biscuit = Biscuit::from_base64(biscuit_base64, public_key).map_err(|e| {
                async_graphql::Error::new(format!("Failed to parse biscuit: {:?}", e))
            })?;

            debug!("Biscuit: {:?}", biscuit);

            let mut authorizier = biscuit.authorizer().map_err(|e| {
                async_graphql::Error::new(format!("Failed to get authorizer: {:?}", e))
            })?;

            let token_data: Vec<(String, String)> = authorizier
                .query("data($identifier, $user_uuid) <- user($identifier, $user_uuid)")
                .map_err(|e| {
                    async_graphql::Error::new(format!("Failed to query biscuit: {:?}", e))
                })?;

            debug!("TokenData: {:?}", token_data);

            let token_data = TokenData {
                identifier: token_data[0].0.clone(),
                user_uuid: token_data[0].1.clone(),
            };

            Some(token_data)
        } else {
            None
        };
        Ok(token_data)
    }
}
