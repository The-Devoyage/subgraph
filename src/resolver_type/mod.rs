use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum ResolverType {
    FindOne,
    FindMany,
    CreateOne,
    UpdateOne,
    UpdateMany,
    InternalType,
}

impl Display for ResolverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolverType::FindOne => write!(f, "FindOne"),
            ResolverType::FindMany => write!(f, "FindMany"),
            ResolverType::CreateOne => write!(f, "CreateOne"),
            ResolverType::UpdateOne => write!(f, "UpdateOne"),
            ResolverType::UpdateMany => write!(f, "UpdateMany"),
            ResolverType::InternalType => write!(f, "InternalType"),
        }
    }
}

impl ResolverType {
    pub fn get_resolver_types() -> Vec<ResolverType> {
        vec![
            ResolverType::FindOne,
            ResolverType::FindMany,
            ResolverType::CreateOne,
            ResolverType::UpdateOne,
            ResolverType::UpdateMany,
        ]
    }
}
