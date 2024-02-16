use crate::configuration::subgraph::SubGraphConfig;
use async_graphql::{
    dynamic::Schema,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::{HeaderMap, StatusCode};
use local_ip_address::local_ip;
use log::{info, trace};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Future, Rejection};

pub mod cli_args;
pub mod configuration;
pub mod data_sources;
pub mod filter_operator;
pub mod graphql;
pub mod resolver_type;
pub mod scalar_option;
pub mod sql_value;
pub mod traits;
pub mod utils;

/// Starts the Subgraph Service. Initializes the DataSources and builds the GraphQL Schema.
pub async fn run(
    args: cli_args::CliArgs,
    subgraph_config: SubGraphConfig,
) -> Result<
    (
        impl Future<Output = ()>,
        Schema,
        tokio::sync::oneshot::Sender<()>,
    ),
    std::io::Error,
> {
    info!("⛵ Starting Subgraph Service");
    trace!("Service Arguments: {:?}", args);

    // Initialize DataSources
    let data_sources = data_sources::DataSources::init(
        subgraph_config.service.data_sources.clone(),
        &args,
        &subgraph_config,
    )
    .await;

    // Build GraphQL Schema
    let schema = graphql::schema::ServiceSchema::new(subgraph_config.clone(), data_sources).build();

    // GraphQL Endpoint
    let graphql_post = async_graphql_warp::graphql(schema.clone())
        .and(warp::header::headers_cloned())
        .and_then(
            |(schema, request): (Schema, async_graphql::Request), headers: HeaderMap| async move {
                let dynamic_request = schema.execute(request.data(headers)).await;
                let response = GraphQLResponse::from(dynamic_request);
                Ok::<_, Infallible>(response)
            },
        );

    // GraphQL Playground Endpoint
    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder().body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    // CORS Config
    let cors = configuration::cors_config::CorsConfig::create_cors(subgraph_config.clone());

    // Routes - Combine GraphQL and GraphQL Playground
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

    // Get Port from CLI Arguments or Subgraph Config
    let port = match args.port.clone() {
        Some(port) => port,
        None => match subgraph_config.clone().service.port {
            Some(port) => port,
            None => 0,
        },
    };

    // Create Graceful Shutdown Channel
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    // If host is true, bind to 0.0.0.0
    let host = match args.host.clone() {
        true => {
            let ip = local_ip().expect("Failed to get local IP address");
            info!("🛝 Playground: http://{:?}:{:?}", ip, port);
            [0, 0, 0, 0]
        }
        false => match subgraph_config.clone().service.host {
            Some(_host) => {
                let ip = local_ip().expect("Failed to get local IP address");
                info!("🛝 Playground: http://{:?}:{:?}", ip, port);
                [0, 0, 0, 0]
            }
            None => {
                info!("🛝 Playground: http://localhost:{:?}", port);
                [127, 0, 0, 1]
            }
        },
    };

    // Return Server, Schema and Graceful Shutdown Channel
    let (_addr, server) = warp::serve(routes).bind_with_graceful_shutdown((host, port), async {
        rx.await.ok();
    });

    info!("❇️  Subgraph Service Started");

    Ok((server, schema, tx))
}
