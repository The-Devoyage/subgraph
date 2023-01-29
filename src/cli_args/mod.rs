use serde::Serialize;
use std::path::PathBuf;

use clap::{builder::PossibleValuesParser, Parser, ValueHint};

#[derive(Parser, Debug, Serialize, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to the subgraph config file.
    #[arg(short, long, value_hint = ValueHint::DirPath)]
    pub config: PathBuf,

    /// Service log level.
    #[serde(rename = "log-level")]
    #[arg(short, long, value_parser = PossibleValuesParser::new(["info", "debug", "info", "warn", "error"]), default_value = "info")]
    pub log_level: Option<String>,

    // The port this service runs on.
    #[arg(short, long, default_value = "0")]
    pub port: Option<u16>,

    // Mongo DB Connection URI String
    #[arg(short, long)]
    pub mongo_uri: Option<String>,
}
