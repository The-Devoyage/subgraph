use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::PathBuf};

use crate::{
    cli_args::CliArgs, configuration::subgraph::entities::ServiceEntityConfig,
    utils::logger::LogLevelEnum,
};

use self::guard::Guard;

pub mod auth;
pub mod cors;
pub mod data_sources;
pub mod entities;
pub mod guard;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub port: Option<u16>,
    pub log_level: Option<LogLevelEnum>,
    pub auth: Option<auth::ServiceAuth>,
    pub guards: Option<Vec<Guard>>,
    pub entities: Vec<entities::ServiceEntityConfig>,
    pub data_sources: Vec<data_sources::ServiceDataSourceConfig>,
    pub cors: Option<cors::CorsConfigOptions>,
    pub imports: Option<Vec<PathBuf>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubGraphConfig {
    pub service: ServiceConfig,
}

impl SubGraphConfig {
    pub fn new(args: &CliArgs) -> SubGraphConfig {
        let read_file = File::open(&args.config.as_ref().unwrap());

        let mut file_config = String::new();

        match read_file {
            Ok(mut f) => {
                f.read_to_string(&mut file_config)
                    .expect("Failed To Read Config File");
            }
            Err(err) => println!("Error Reading Config File: {}", err),
        };

        let subgraph_config = toml::from_str::<SubGraphConfig>(&file_config);

        let mut subgraph_config = match subgraph_config {
            Ok(config) => config,
            Err(error) => {
                println!("{}", error);
                panic!("Provide Valid Subgraph Config");
            }
        };

        if subgraph_config.service.imports.is_some() {
            let imports = subgraph_config.service.imports.clone().unwrap();
            for path in imports {
                let read_import_config = File::open(&path);

                let mut import_config = String::new();

                match read_import_config {
                    Ok(mut f) => {
                        f.read_to_string(&mut import_config)
                            .expect("Failed To Read Imported Config File");
                    }
                    Err(err) => println!("Error Reading Config File: {}", err),
                };

                let import_config = toml::from_str::<ServiceEntityConfig>(&import_config);

                if import_config.is_ok() {
                    let mut service = subgraph_config.service.clone();
                    service.entities.push(import_config.unwrap());
                    subgraph_config.service = service;
                    return subgraph_config;
                }
            }
        }

        subgraph_config
    }

    pub fn get_entity(self, entity_name: &str) -> Option<entities::ServiceEntityConfig> {
        for entity in self.service.entities {
            if entity.name == entity_name {
                return Some(entity);
            }
        }

        None
    }
}
