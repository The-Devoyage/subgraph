use clap::Parser;

use subgraph::{
    cli_args,
    configuration::{environment::Environment, subgraph::SubGraphConfig},
    run, utils,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = cli_args::CliArgs::parse();
    let environment = Environment::init();
    let mut subgraph_config = SubGraphConfig::init(&args);
    subgraph_config = Environment::replace_env_vars_in_config(subgraph_config, environment);

    utils::logger::Logger::init(&args, &subgraph_config);
    Ok(run(args, subgraph_config).await?.0.await)
}
