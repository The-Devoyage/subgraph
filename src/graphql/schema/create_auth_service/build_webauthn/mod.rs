use crate::{configuration::subgraph::auth::ServiceAuth, graphql::schema::ServiceSchema};
use log::{debug, error, trace};
use reqwest::Url;
use webauthn_rs::{Webauthn, WebauthnBuilder};

impl ServiceSchema {
    pub fn build_webauthn(auth_config: &ServiceAuth) -> Result<Webauthn, async_graphql::Error> {
        debug!("Building Webauthn");

        let rp_origin = Url::parse(&auth_config.requesting_party_origin).map_err(|e| {
            error!("Failed to parse requesting party origin: {:?}", e);
            error!(
                "Requesting Party Origin: {:?}",
                &auth_config.requesting_party_origin
            );
            async_graphql::Error::new(format!("Failed to parse requesting party origin: {:?}", e))
        });

        let rp_origin = match rp_origin {
            Ok(_) => rp_origin.unwrap(),
            Err(e) => return Err(e),
        };

        let webauthn_builder = WebauthnBuilder::new(&auth_config.requesting_party, &rp_origin)
            .map_err(|e| {
                error!("Failed to build webauthn builder: {:?}", e);
                trace!("Requesting Party: {:?}", &auth_config.requesting_party);
                async_graphql::Error::new(format!("Failed to build webauthn builder: {:?}", e))
            });

        let webauthn_builder = match webauthn_builder {
            Ok(_) => webauthn_builder.unwrap(),
            Err(e) => return Err(e),
        };

        let webauthn = webauthn_builder
            .rp_name(&auth_config.requesting_party_name)
            .build()
            .map_err(|e| {
                error!("Failed to build webauthn: {:?}", e);
                trace!(
                    "Requesting Party Name: {:?}",
                    &auth_config.requesting_party_name
                );
                async_graphql::Error::new(format!("Failed to build webauthn: {:?}", e))
            })?;

        trace!("Webauthn Created: {:?}", webauthn);
        Ok(webauthn)
    }
}
