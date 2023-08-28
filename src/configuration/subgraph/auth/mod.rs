use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceAuth {
    pub identifier: String,
    pub requesting_party: String,
    pub requesting_party_name: String,
    pub requesting_party_origin: String,
    /// The name of the data source to save credentials.
    pub data_source: String,
}
