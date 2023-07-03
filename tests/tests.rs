use std::path::PathBuf;

use async_graphql::dynamic::Schema;
use http::HeaderMap;
use subgraph::configuration::{environment::Environment, subgraph::SubGraphConfig};

mod http_ds;
mod mongo;
mod mysql;
mod postgres;
mod sqlite;

async fn spawn_app(args: subgraph::cli_args::CliArgs) -> Schema {
    let environment = Environment::init();
    let mut subgraph_config = SubGraphConfig::init(&args);
    subgraph_config = Environment::replace_env_vars_in_config(subgraph_config, environment);

    let server = subgraph::run(args, subgraph_config)
        .await
        .expect("Failed to run server.");
    let (server, schema) = server;
    let _ = tokio::spawn(server);
    schema
}

async fn execute(
    request: async_graphql::Request,
    args: Option<subgraph::cli_args::CliArgs>,
) -> async_graphql::Response {
    let args = args.unwrap_or(subgraph::cli_args::CliArgs {
        config: PathBuf::from("./tests/test_config.toml"),
        port: None,
        log_level: None,
    });
    let schema = spawn_app(args).await;
    let headers = HeaderMap::new();
    let response = schema.execute(request.data(headers.clone())).await;
    println!("response: {:?}", response);
    response
}
