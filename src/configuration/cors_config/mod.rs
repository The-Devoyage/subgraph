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
        debug!("Creating CORS Config");

        let mut cors = warp::cors();
        let cors_config = match subgraph_config.service.cors {
            Some(config) => config,
            None => CorsConfigOptions {
                allow_methods: Some(vec![MethodOption {
                    method: Method::POST,
                }]),
                allow_headers: Some(vec![CONTENT_TYPE.to_string()]),
                allow_any_origin: Some(true),
                allow_origins: None,
            },
        };

        debug!("Generated Cors Config, {:?}", cors_config);

        if cors_config.allow_methods.is_some() {
            let methods = cors_config.allow_methods.unwrap();
            cors = cors.allow_methods(methods.iter().map(|m| &m.method));
        }

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
        cors.build()
    }
}
