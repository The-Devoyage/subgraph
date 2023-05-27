use async_graphql::{
    dynamic::Schema,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use clap::Parser;
use env_logger::Env;
use http::{HeaderMap, StatusCode};
use log::{info, Level};
use std::{convert::Infallible, str::FromStr};
use warp::{http::Response as HttpResponse, Filter, Rejection};

mod cli_args;
mod configuration;
mod data_sources;
mod graphql;
mod utils;

#[tokio::main]
async fn main() {
    let args = cli_args::CliArgs::parse();

    let environment = configuration::environment::Environment::init();
    let mut subgraph_config = configuration::subgraph::SubGraphConfig::init(&args);
    subgraph_config = configuration::environment::Environment::replace_env_vars_in_config(
        subgraph_config,
        environment,
    );

    let log_level = match args.log_level.clone() {
        Some(level) => {
            println!("Using Args Log Level: {}", level);
            let level_from_str = Level::from_str(&level);
            match level_from_str {
                Ok(level) => level,
                Err(_) => panic!("Failed to get log level from args."),
            }
        }
        None => match subgraph_config.clone().service.log_level {
            Some(level) => {
                println!("Using Config Log Level: {}", level);
                utils::log_level::LogLevelEnum::parse_log_level(level)
            }
            None => {
                println!("Using Default Log Level: {}", Level::Info);
                Level::Info
            }
        },
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(log_level.to_string())).init();

    let data_sources =
        data_sources::DataSources::init(subgraph_config.service.data_sources.clone()).await;

    let schema =
        graphql::schema::ServiceSchemaBuilder::new(subgraph_config.clone(), data_sources).build();

    let graphql_post = async_graphql_warp::graphql(schema)
        .and(warp::header::headers_cloned())
        .and_then(
            |(schema, request): (Schema, async_graphql::Request), headers: HeaderMap| async move {
                let dynamic_request = schema.execute(request.data(headers)).await;
                let response = GraphQLResponse::from(dynamic_request);
                Ok::<_, Infallible>(response)
            },
        );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let cors = configuration::cors_config::CorsConfig::create_cors(subgraph_config.clone());

    let routes =
        graphql_playground
            .or(graphql_post)
            .with(cors)
            .recover(|err: Rejection| async move {
                if let Some(GraphQLBadRequest(err)) = err.find() {
                    return Ok::<_, Infallible>(warp::reply::with_status(
                        err.to_string(),
                        StatusCode::BAD_REQUEST,
                    ));
                }

                Ok(warp::reply::with_status(
                    "INTERNAL_SERVER_ERROR".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            });

    let port = match args.port.clone() {
        Some(port) => port,
        None => match subgraph_config.clone().service.port {
            Some(port) => port,
            None => 0,
        },
    };

    info!("Playground: http://localhost:{:?}", port);

    let server = warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    server
}
