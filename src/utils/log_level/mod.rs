use log::Level;
use serde::{Deserialize, Serialize};
use std::fmt;

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
