use regex::Regex;
use std::collections::HashMap;

use serde_json::{json, Value};

use crate::configuration::subgraph::SubGraphConfig;

use super::Environment;

impl Environment {
    pub fn replace_env_vars_in_config(
        config: SubGraphConfig,
        env: HashMap<String, String>,
    ) -> SubGraphConfig {
        println!("Replacing env vars in config");
        let config_json = json!(config);

        let replaced_json = Environment::replace_env_vars_in_json(config_json, env);

        match serde_json::from_value(replaced_json) {
            Ok(config) => config,
            Err(e) => panic!("Error parsing config: {}", e),
        }
    }

    fn replace_env_vars_in_json(json: Value, env: HashMap<String, String>) -> Value {
        let json_string = json.to_string();
        let re = Regex::new(r#"(\$[A-Za-z_][A-Za-z0-9_]*)|\$[A-Za-z_][A-Za-z0-9_]*"#).unwrap();
        let replaced_json = re
            .replace_all(&json_string, |caps: &regex::Captures| {
                let env_var = &caps[1];
                let env_var = env.get(env_var.trim_start_matches('$'));
                match env_var {
                    Some(value) => value.to_string(),
                    None => caps[0].to_string(),
                }
            })
            .to_string();

        println!("Replaced json: {}", replaced_json);

        let json = match serde_json::from_str(&replaced_json) {
            Ok(json) => json,
            Err(e) => panic!("Error parsing config: {}", e),
        };

        println!("Replaced json: {}", json);

        json
    }
}
