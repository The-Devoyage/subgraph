use std::str::FromStr;

use super::ServiceSchema;
use base64::{engine::general_purpose, Engine as _};
use biscuit_auth::{KeyPair, PrivateKey};
use log::{debug, error, info, trace};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;
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
    #[serde(deserialize_with = "deserialize_uuid")]
    uuid: uuid::Uuid,
    identifier: String,
    #[serde(deserialize_with = "deserialize_registration_state")]
    registration_state: PasskeyRegistration,
    #[serde(deserialize_with = "deserialize_passkey")]
    passkey: Option<Passkey>,
    #[serde(deserialize_with = "deserialize_authentication_state")]
    authentication_state: Option<PasskeyAuthentication>,
}

fn deserialize_uuid<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    debug!("Deserializing UUID");
    let s = String::deserialize(deserializer)?;
    trace!("Parsing UUID: {}", &s);
    match Uuid::from_str(&s) {
        Ok(uuid) => {
            trace!("Parsed UUID: {}", &uuid);
            Ok(uuid)
        }
        Err(_) => {
            trace!("Failed to parse UUID: {}", &s);
            Uuid::from_str(&s).map_err(serde::de::Error::custom)
        }
    }
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
    debug!("Deserializing registration state");
    let s = String::deserialize(deserializer)?;
    let reg_state = serde_json::from_str(&s).unwrap();
    trace!("Deserialized registration state: {:?}", &reg_state);
    Ok(reg_state)
}

fn deserialize_passkey<'de, D>(deserializer: D) -> Result<Option<Passkey>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    debug!("Deserializing passkey");
    let s = String::deserialize(deserializer);
    match s {
        Ok(s) => {
            let passkey = serde_json::from_str(&s).unwrap();
            trace!("Deserialized passkey: {:?}", &passkey);
            Ok(Some(passkey))
        }
        Err(_) => Ok(None),
    }
}

fn deserialize_authentication_state<'de, D>(
    deserializer: D,
) -> Result<Option<PasskeyAuthentication>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    debug!("Deserializing authentication state");
    let s = String::deserialize(deserializer);
    match s {
        Ok(s) => {
            let auth_state = serde_json::from_str(&s).unwrap();
            trace!("Deserialized authentication state: {:?}", &auth_state);
            Ok(Some(auth_state))
        }
        Err(_) => Ok(None),
    }
}

impl ServiceSchema {
    pub fn create_auth_service(mut self) -> Self {
        self = self.create_register_start();
        self = self.create_register_finish();
        self = self.create_authenticate_start();
        self = self.create_authenticate_finish();
        self
    }

    /// Enables the auth service by creating a key pair unless one is provided.
    pub fn get_key_pair(&mut self) {
        debug!("Getting key pair");
        let key_pair;
        if self.subgraph_config.service.auth.is_some() {
            //info message with an unicode icon
            info!("🔐 Auth Enabled!");
            let auth = self.subgraph_config.service.auth.clone().unwrap();
            let b64_private_key = auth.private_key;

            if b64_private_key.is_some() {
                trace!("Using provided key pair");
                let bytes_private_key = &general_purpose::URL_SAFE_NO_PAD
                    .decode(b64_private_key.unwrap())
                    .map_err(|e| {
                        error!("Error decoding private key: {}", e);
                        e
                    })
                    .unwrap();

                let private_key = PrivateKey::from_bytes(bytes_private_key);

                key_pair = Some(KeyPair::from(&private_key.unwrap()));
            } else {
                key_pair = Some(KeyPair::new());
            }
        } else {
            key_pair = None;
        }
        self.key_pair = key_pair;
    }
}
