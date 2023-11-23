use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GuardDataContext {
    pub entity_name: String,
    pub query: String,
    pub variables: Vec<VariablePair>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariablePair(pub String, pub String);
