use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

use crate::cli_args::CliArgs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScalarOptions {
    String,
    Int,
    Boolean,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityFieldOptions {
    pub name: String,
    pub scalar: ScalarOptions,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntity {
    pub name: String,
    pub fields: Vec<ServiceEntityFieldOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub service_name: String,
    pub entities: Vec<ServiceEntity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGraphConfig {
    pub service: ServiceConfig,
}

impl SubGraphConfig {
    pub fn init(args: &CliArgs) -> SubGraphConfig {
        let read_file = File::open(&args.config);
        let mut file_config = String::new();
        match read_file {
            Ok(mut f) => {
                f.read_to_string(&mut file_config)
                    .expect("Failed To Read Config File");
            }
            Err(_) => println!("Error Reading Config File"),
        };
        let subgraph_config = toml::from_str::<SubGraphConfig>(&file_config);

        let subgraph_config = match subgraph_config {
            Ok(config) => config,
            Err(error) => {
                error!("Invalid Subgraph Config");
                debug!("{}", error);
                panic!("Provide Valid Subgraph Config");
            }
        };

        subgraph_config
    }
}
