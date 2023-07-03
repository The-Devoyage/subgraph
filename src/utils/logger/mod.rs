use env_logger::Env;
use log::Level;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::{cli_args::CliArgs, configuration::subgraph::SubGraphConfig};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevelEnum {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl fmt::Display for LogLevelEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            LogLevelEnum::Error => "error",
            LogLevelEnum::Warn => "warn",
            LogLevelEnum::Info => "info",
            LogLevelEnum::Debug => "debug",
            LogLevelEnum::Trace => "trace",
        };
        write!(f, "{}", variant_str)
    }
}

impl LogLevelEnum {
    pub fn parse_log_level(log_level_enum: LogLevelEnum) -> Level {
        match log_level_enum {
            LogLevelEnum::Error => Level::Error,
            LogLevelEnum::Warn => Level::Warn,
            LogLevelEnum::Info => Level::Info,
            LogLevelEnum::Debug => Level::Debug,
            LogLevelEnum::Trace => Level::Trace,
        }
    }
}

pub struct Logger;
impl Logger {
    pub fn init(args: &CliArgs, subgraph_config: &SubGraphConfig) {
        let log_level = match args.log_level.clone() {
            Some(level) => {
                let level_from_str = Level::from_str(&level);
                match level_from_str {
                    Ok(level) => level,
                    Err(_) => panic!("Failed to get log level from args."),
                }
            }
            None => match subgraph_config.clone().service.log_level {
                Some(level) => {
                    println!("Using Config Log Level: {}", level);
                    LogLevelEnum::parse_log_level(level)
                }
                None => {
                    println!("Using Default Log Level: {}", Level::Info);
                    Level::Info
                }
            },
        };

        env_logger::Builder::from_env(Env::default().default_filter_or(log_level.to_string()))
            .init();
    }
}
