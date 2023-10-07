use std::collections::HashMap;

use log::debug;
use serde_json::{json, Value};

use crate::configuration::subgraph::SubGraphConfig;

use super::Environment;

impl Environment {
    pub fn replace_env_vars_in_config(
        config: SubGraphConfig,
        env: HashMap<String, String>,
    ) -> SubGraphConfig {
        debug!("Replacing env vars in config");
        let config_json = json!(config);

        let replaced_json = Environment::replace_env_vars_in_json(config_json, env);

        match serde_json::from_value(replaced_json) {
            Ok(config) => config,
            Err(e) => panic!("Error parsing config: {}", e),
        }
    }

    fn replace_env_vars_in_json(json: Value, env: HashMap<String, String>) -> Value {
        let mut string = json.to_string();
        for (key, value) in env {
            string = string.replace(&format!("\"${}\"", key), &format!("\"{}\"", &value));
        }
        match serde_json::from_str(&string) {
            Ok(json) => json,
            Err(e) => panic!("Error parsing config: {}", e),
        }
    }
}
