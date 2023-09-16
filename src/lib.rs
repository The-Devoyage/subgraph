use crate::configuration::subgraph::SubGraphConfig;
use async_graphql::{
    dynamic::Schema,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::{HeaderMap, StatusCode};
use log::info;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Future, Rejection};

pub mod cli_args;
pub mod configuration;
mod data_sources;
mod graphql;
pub mod utils;

pub async fn run(
    args: cli_args::CliArgs,
    subgraph_config: SubGraphConfig,
) -> Result<(impl Future<Output = ()>, Schema), std::io::Error> {
    info!("⛵ Starting Subgraph Service ⛵");
    info!("Using Args: {:?}", args);

    let data_sources =
        data_sources::DataSources::init(subgraph_config.service.data_sources.clone(), &args).await;

    let schema =
        graphql::schema::ServiceSchemaBuilder::new(subgraph_config.clone(), data_sources).build();

    let graphql_post = async_graphql_warp::graphql(schema.clone())
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

    let server = warp::serve(routes).run(([0, 0, 0, 0], port));

    Ok((server, schema))
}
