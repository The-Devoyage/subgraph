use super::ServiceSchemaBuilder;
use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::{Passkey, PasskeyAuthentication, PasskeyRegistration};

pub mod build_webauthn;
mod create_register_finish;
mod create_register_start;
pub mod delete_user;
mod finish_authentication;
pub mod get_user;
mod start_authentication;
pub mod update_user;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceUser {
    identifier: String,
    #[serde(deserialize_with = "deserialize_registration_state")]
    registration_state: PasskeyRegistration,
    passkey: Option<Passkey>,
    authentication_state: Option<PasskeyAuthentication>,
}

fn deserialize_registration_state<'de, D>(deserializer: D) -> Result<PasskeyRegistration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let reg_state = serde_json::from_str(&s).unwrap();
    Ok(reg_state)
}

impl ServiceSchemaBuilder {
    pub fn create_auth_service(mut self) -> Self {
        self = self.create_register_start();
        self = self.create_register_finish();
        self = self.create_authenticate_start();
        self = self.create_authenticate_finish();
        self
    }
}
