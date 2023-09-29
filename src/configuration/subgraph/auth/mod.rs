use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceAuth {
    /// URL of the service to authenticate with. ex: domain.com
    /// Must be a domain of the requesting party origin.
    pub requesting_party: String,
    /// The name of the requesting party. This is used to identify the requesting party. Non vital. ex: MyWebsite
    pub requesting_party_name: String,
    /// The origin of the requesting party. This is used to validate the requesting party. ex: https://domain.com
    /// Must be a valid URL.
    pub requesting_party_origin: String,
    /// The name of the data source to save credentials.
    pub data_source: String,
    ///Private Key - Manually set your private key. Leave blank to generate a new key pair each
    ///time the service starts. Must be base64 encoded byte array.
    pub private_key: Option<String>,
}
