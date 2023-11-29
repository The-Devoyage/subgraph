use serde::Serialize;
use std::path::PathBuf;

use clap::{builder::PossibleValuesParser, Parser, ValueHint};

mod generate_keypair;

/// Command line arguments for the Subgraph Service.
#[derive(Parser, Debug, Serialize, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to the subgraph config file.
    #[arg(short, long, value_hint = ValueHint::DirPath)]
    pub config: Option<PathBuf>,

    /// Service log level.
    #[serde(rename = "log-level")]
    #[arg(short, long, value_parser = PossibleValuesParser::new(["info", "debug", "info", "warn", "error"]))]
    pub log_level: Option<String>,

    /// The port this service runs on.
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Run migrations
    #[arg(short, long, value_parser = PossibleValuesParser::new(["run", "revert"]))]
    pub migrate: Option<String>,

    ///Generate Key Pair
    #[arg(short, long)]
    pub generate_keypair: bool,

    /// Start the service in watch mode. Changes made to the subgraph config file will be
    /// automatically applied. The service will restart.
    #[arg(short, long)]
    pub watch: bool,
}

impl CliArgs {
    /// Execute functions based on the flags passed to the service.
    pub fn handle_flags(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.generate_keypair()?;
        Ok(())
    }
}
