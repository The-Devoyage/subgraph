use crate::{configuration::subgraph::auth::ServiceAuth, graphql::schema::ServiceSchemaBuilder};
use log::debug;
use reqwest::Url;
use webauthn_rs::{Webauthn, WebauthnBuilder};

impl ServiceSchemaBuilder {
    pub fn build_webauthn(auth_config: &ServiceAuth) -> Result<Webauthn, async_graphql::Error> {
        debug!("Building Webauthn");

        let rp_origin = Url::parse(&auth_config.requesting_party_origin)?;

        let webauthn_builder = WebauthnBuilder::new(&auth_config.requesting_party, &rp_origin)?;

        let webauthn = webauthn_builder
            .rp_name(&auth_config.requesting_party_name)
            .build()?;

        debug!("Webauthn Created: {:?}", webauthn);
        Ok(webauthn)
    }
}
