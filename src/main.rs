use async_graphql::{
    dynamic::Schema,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use clap::Parser;
use env_logger::Env;
use http::StatusCode;
use log::info;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

mod cli_args;
mod configuration;
mod data_sources;
mod graphql;

#[tokio::main]
async fn main() {
    let args = cli_args::CliArgs::parse();

    env_logger::Builder::from_env(
        Env::default().default_filter_or(args.clone().log_level.unwrap()),
    )
    .init();

    let environment = configuration::environment::Environment::init();
    let mut subgraph_config = configuration::subgraph::SubGraphConfig::init(&args);
    subgraph_config = configuration::environment::Environment::replace_env_vars_in_config(
        subgraph_config,
        environment,
    );

    let data_sources =
        data_sources::DataSources::init(subgraph_config.service.data_sources.clone()).await;

    let schema =
        graphql::schema::ServiceSchemaBuilder::new(subgraph_config.clone(), data_sources).build();

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (Schema, async_graphql::Request)| async move {
            let dynamic_request = schema.execute(request).await;
            let response = GraphQLResponse::from(dynamic_request);
            Ok::<_, Infallible>(response)
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let cors = configuration::cors_config::CorsConfig::create_cors(subgraph_config);

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

    info!(
        "Playground: http://localhost:{:?}",
        args.clone().port.unwrap()
    );

    let server = warp::serve(routes)
        .run(([0, 0, 0, 0], args.port.unwrap()))
        .await;

    server
}
