use std::path::PathBuf;

use async_graphql::dynamic::Schema;
use http::HeaderMap;
use subgraph::configuration::{environment::Environment, subgraph::SubGraphConfig};

mod http_ds;
mod mongo;
mod mysql;
mod postgres;
mod service;
mod sqlite;

async fn spawn_app(args: subgraph::cli_args::CliArgs) -> Schema {
    let environment = Environment::new();
    let mut subgraph_config = SubGraphConfig::new(&args).unwrap();
    subgraph_config = Environment::replace_env_vars_in_config(subgraph_config, environment);

    let server = subgraph::run(args, subgraph_config)
        .await
        .expect("Failed to run server.");
    let (server, schema, _shutdown) = server;
    let _ = tokio::spawn(server);
    schema
}

async fn execute(
    request: async_graphql::Request,
    args: Option<subgraph::cli_args::CliArgs>,
) -> async_graphql::Response {
    let args = args.unwrap_or(subgraph::cli_args::CliArgs {
        config: Some(PathBuf::from("./tests/test_config.toml")),
        port: None,
        log_level: None,
        generate_keypair: false,
        migrate: None,
        watch: false,
    });
    let schema = spawn_app(args).await;
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "ErEBCkcKDW5pY2tpc3lvdXJmYW4KJGQ3MjgxNjg2LTdhNGMtNGE4Yi04MzY3LWFiYzJlMDUyNTNkORgDIg4KDAgKEgMYgAgSAxiBCBIkCAASIMwJxaQ8TbWNeTeIxPFkgNGHM-8V_UzvtijMTVgxwlwUGkD2EOehKSTh2ycqf2J12f9BfOghhzJZigtkIu7ZSZQUQGV_jMSigkL3OHIaEbKcXhOgfbKzJ1z76h6ww4U_1-gPIiIKIL8OSIotMVhBwLPTvLdtXyN_Dv3YnFcqXK_u0ZcfvtKm".parse().unwrap());
    let response = schema.execute(request.data(headers.clone())).await;
    println!("response: {:?}", response);
    response
}
