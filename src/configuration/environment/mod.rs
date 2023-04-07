use std::collections::HashMap;

use serde::Deserialize;

pub mod parse_subgraph_config;

#[derive(Deserialize, Debug)]
pub struct Environment {}

impl Environment {
    pub fn init() -> HashMap<String, String> {
        std::env::vars().collect()
    }
}
