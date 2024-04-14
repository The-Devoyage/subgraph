use crate::configuration::subgraph::{serve::ServeOptions, SubGraphConfig};
use async_graphql::{
    dynamic::Schema,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::{HeaderMap, StatusCode};
use local_ip_address::local_ip;
use log::{error, info, trace};
use std::{collections::HashMap, convert::Infallible};
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

    // GraphQL Endpoint at /graphql
    let graphql_post = warp::path("graphql")
        .and(async_graphql_warp::graphql(schema.clone()))
        .and(warp::header::headers_cloned())
        .and_then(
            |(schema, request): (Schema, async_graphql::Request), headers: HeaderMap| async move {
                let dynamic_request = schema.execute(request.data(headers)).await;
                let response = GraphQLResponse::from(dynamic_request);
                Ok::<_, Infallible>(response)
            },
        );

    // GraphQL Playground Endpoint
    let graphql_playground = warp::path("playground").and(warp::get()).map(|| {
        HttpResponse::builder().body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    // CORS Config
    let cors = configuration::cors_config::CorsConfig::create_cors(subgraph_config.clone());

    // Init File Serving If Config Present
    if subgraph_config.service.serve.is_some() {
        let serve_options = subgraph_config.clone().service.serve.unwrap();
        if let Some(asset) = serve_options.assets {
            info!("📁 Asset Route: {:?}", asset.route);
            info!("📁 Asset Path: {:?}", asset.path);
        }
        if let Some(ssr) = serve_options.ssr {
            info!("📁 SSR Route: {:?}", ssr.route);
            info!("📁 SSR Path: {:?}", ssr.path);
        }
    }
    let asset_options = subgraph_config
        .clone()
        .service
        .serve
        .unwrap_or_default()
        .assets;
    let ssr_options = subgraph_config
        .clone()
        .service
        .serve
        .unwrap_or_default()
        .ssr;

    // Handle File Serving
    let assets_route = warp::path(asset_options.clone().unwrap_or_default().route)
        .and(warp::fs::dir(
            asset_options.clone().unwrap_or_default().path,
        ))
        .and_then(move |file| ServeOptions::handle_serve_asset(file, asset_options.clone()));

    let ssr_route = warp::path::full() // Look into nested routing
        .and(warp::post().or(warp::get()))
        // Get Request Body
        .and(warp::body::form())
        // Get Search Query
        .and(warp::query::<HashMap<String, String>>())
        .map(
            move |path: warp::filters::path::FullPath,
                  _,
                  body: HashMap<String, String>,
                  search: HashMap<String, String>| {
                trace!("SSR Path: {:?}", path);
                trace!("SSR Body: {:?}", body);
                trace!("SSR Search: {:?}", search);

                // Serve a file located at the same path as the request path
                let file = format!(
                    "{}/{}",
                    ssr_options.clone().unwrap_or_default().path,
                    path.as_str()
                );

                trace!("SSR File: {:?}", file);

                let has_extension = file.contains(".");
                let ext;

                // If the file does not have an extension, assume it is a directory and serve the index.html file
                let file = if !has_extension {
                    let file = format!("{}/index.html", file);
                    let file = std::fs::read_to_string(file);
                    ext = Some("html".to_string());
                    match file {
                        Ok(file) => file,
                        Err(err) => {
                            error!("SSR File Error: {:?}", err);
                            return HttpResponse::builder()
                                .body("SSR File Error. Check console for detail.".to_string());
                        }
                    }
                } else {
                    ext = file.split(".").last().map(|s| s.to_string());
                    let file = std::fs::read_to_string(file);
                    match file {
                        Ok(file) => file,
                        Err(err) => {
                            error!("SSR File Error: {:?}", err);
                            return HttpResponse::builder()
                                .body("SSR File Error. Check console for detail.".to_string());
                        }
                    }
                };

                // If enable_ssr, hydrate the SSR template
                let enable_hydrate = ssr_options
                    .clone()
                    .unwrap_or_default()
                    .enable_hydrate
                    .unwrap_or_default();

                if enable_hydrate && ext == Some("html".to_string()) {
                    // Hydrate the SSR template
                    let html = ServeOptions::handle_hydrate_ssr(file, ssr_options.clone());
                    match html {
                        Ok(html) => HttpResponse::builder().body(html),
                        Err(err) => {
                            error!("SSR Hydrate Error: {:?}", err);
                            HttpResponse::builder().body("SSR Hydrate Error".to_string())
                        }
                    }
                } else if enable_hydrate && ext != Some("html".to_string()) {
                    error!("SSR Hydrate Error: SSR Hydrate is only available for HTML files");
                    HttpResponse::builder().body(
                        "SSR Hydrate Error: SSR Hydrate is only available for HTML files"
                            .to_string(),
                    )
                } else {
                    // Return the file as is
                    HttpResponse::builder().body(file)
                }
            },
        );

    // Routes - Combine GraphQL and GraphQL Playground
    let routes = graphql_playground
        .or(graphql_post)
        .or(assets_route)
        .or(ssr_route)
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
        None => match subgraph_config.service.port.clone() {
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
        false => match subgraph_config.service.host {
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
