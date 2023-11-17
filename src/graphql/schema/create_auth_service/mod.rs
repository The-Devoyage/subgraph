use super::ServiceSchemaBuilder;
use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::{Passkey, PasskeyAuthentication, PasskeyRegistration};

pub mod build_webauthn;
pub mod delete_user;
mod finish_authentication;
mod finish_register;
pub mod get_user;
mod start_authentication;
mod start_register;
pub mod update_user;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceUser {
    uuid: uuid::Uuid,
    identifier: String,
    #[serde(deserialize_with = "deserialize_registration_state")]
    registration_state: PasskeyRegistration,
    #[serde(deserialize_with = "deserialize_passkey")]
    passkey: Option<Passkey>,
    #[serde(deserialize_with = "deserialize_authentication_state")]
    authentication_state: Option<PasskeyAuthentication>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub user_uuid: String,
    pub identifier: String,
}

fn deserialize_registration_state<'de, D>(deserializer: D) -> Result<PasskeyRegistration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let reg_state = serde_json::from_str(&s).unwrap();
    Ok(reg_state)
}

fn deserialize_passkey<'de, D>(deserializer: D) -> Result<Option<Passkey>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let passkey = serde_json::from_str(&s).unwrap();
    Ok(passkey)
}

fn deserialize_authentication_state<'de, D>(
    deserializer: D,
) -> Result<Option<PasskeyAuthentication>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let auth_state = serde_json::from_str(&s).unwrap();
    Ok(auth_state)
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
