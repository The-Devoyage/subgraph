use http::{header::CONTENT_TYPE, Method};
use log::debug;
use warp::cors::Cors;

use crate::configuration::subgraph::{
    cors::{CorsConfigOptions, MethodOption},
    SubGraphConfig,
};

pub struct CorsConfig;

impl CorsConfig {
    pub fn create_cors(subgraph_config: SubGraphConfig) -> Cors {
        debug!("Enabling Cors");

        let mut cors = warp::cors();
        let cors_config = match subgraph_config.service.cors {
            Some(config) => config,
            None => {
                debug!("Cors Config not found, using default config");
                CorsConfigOptions {
                    allow_methods: Some(vec![MethodOption {
                        method: Method::POST,
                    }]),
                    allow_headers: Some(vec![CONTENT_TYPE.to_string()]),
                    allow_any_origin: Some(true),
                    allow_origins: None,
                }
            }
        };

        debug!("Generated Cors Config, {:?}", cors_config);

        let methods = match cors_config.allow_methods {
            Some(methods) => methods,
            None => {
                debug!("Cors Config allow_methods not found, using default cors `allow_methods` config: POST");
                vec![MethodOption {
                    method: Method::POST,
                }]
            }
        };

        cors = cors.allow_methods(methods.iter().map(|m| &m.method));

        if cors_config.allow_headers.is_some() {
            let headers = cors_config.allow_headers.unwrap();
            cors = cors.allow_headers(headers);
        }

        if cors_config.allow_origins.is_some() {
            let allow_origins = cors_config.allow_origins.unwrap();

            for origin in allow_origins {
                cors = cors.allow_origin(origin.as_str())
            }
        } else {
            cors = cors.allow_any_origin()
        }

        debug!("Cors Config: {:?}", cors);
        cors.build()
    }
}
