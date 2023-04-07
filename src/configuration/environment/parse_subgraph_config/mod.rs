use std::collections::HashMap;

use serde_json::{json, Value};

use crate::configuration::subgraph::SubGraphConfig;

use super::Environment;

impl Environment {
    pub fn replace_env_vars_in_config(
        config: SubGraphConfig,
        env: HashMap<String, String>,
    ) -> SubGraphConfig {
        let config_json = json!(config);

        let replaced_json = Environment::replace_env_vars_in_json(config_json, env);

        serde_json::from_value(replaced_json).unwrap()
    }

    fn replace_env_vars_in_json(json: Value, env: HashMap<String, String>) -> Value {
        match json {
            Value::String(s) => {
                let mut replaced = s;
                for (key, value) in env {
                    replaced = replaced.replace(&format!("${}", key), &value);
                }
                Value::String(replaced)
            }
            Value::Array(arr) => {
                let replaced = arr
                    .into_iter()
                    .map(|elem| Environment::replace_env_vars_in_json(elem, env.clone()))
                    .collect();
                Value::Array(replaced)
            }
            Value::Object(obj) => {
                let replaced = obj
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            key,
                            Environment::replace_env_vars_in_json(value, env.clone()),
                        )
                    })
                    .collect();
                Value::Object(replaced)
            }
            v => v,
        }
    }
}
