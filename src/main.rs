use std::{
    collections::HashMap,
    process::exit,
    sync::{Arc, Mutex},
};

use clap::Parser;

use log::{error, info};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use subgraph::{
    cli_args,
    configuration::{environment::Environment, subgraph::SubGraphConfig},
    run, utils,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Parse the service arguments
    let args = cli_args::CliArgs::parse();
    let environment = Environment::new();

    // Handle functionality that does not require a subgraph config
    // Migrations, etc.
    match args.handle_flags() {
        Ok(_) => {}
        Err(e) => {
            error!("Error handling flags: {}", e);
            exit(1);
        }
    };

    // If no config is provided, exit
    if args.config.is_none() {
        println!("No config provided, exiting...");
        return Ok(());
    }

    // Get the subgraph config
    let mut subgraph_config = SubGraphConfig::new(&args).unwrap();
    subgraph_config = Environment::replace_env_vars_in_config(subgraph_config, environment.clone());

    // Initialize the logger
    utils::logger::Logger::init(&args, &subgraph_config);

    // Start the server, if watch is enabled, start the watcher
    // else, just start the server.
    if args.clone().watch {
        start_server_with_watcher(args.clone(), subgraph_config, environment).await?;
        Ok(())
    } else {
        start_server(args.clone(), subgraph_config).await?;
        Ok(())
    }
}

/// Starts the server and returns a future that can be awaited on.
async fn start_server(
    args: cli_args::CliArgs,
    subgraph_config: SubGraphConfig,
) -> Result<(), std::io::Error> {
    let (server, _schema, _shutdown) = run(args.clone(), subgraph_config).await?;
    server.await;
    Ok(())
}

/// Starts the server and returns a future that can be awaited on.
/// Also starts a watcher that will reload the server when a file changes.
async fn start_server_with_watcher(
    args: cli_args::CliArgs,
    subgraph_config: SubGraphConfig,
    environment: HashMap<String, String>,
) -> Result<(), std::io::Error> {
    // Create a config that can be shared between the server and the watcher
    let config = Arc::new(Mutex::new(subgraph_config.clone()));
    let cloned_config = Arc::clone(&config);
    let cloned_environment = environment.clone();
    let cloned_args = args.clone();

    // Create a channel that will be used to communicate between the watcher and the server.
    // Send a message to the server to start it.
    let (tx, rx) = std::sync::mpsc::channel::<bool>();
    tx.send(false).unwrap();

    // Create a timer that will prevent the watcher from firing too often.
    let mut last_received = std::time::Instant::now();

    // Create the watcher
    let mut watcher: RecommendedWatcher = match Watcher::new(
        move |event: Result<Event, notify::Error>| {
            let event = event.unwrap();
            let cloned_environment = cloned_environment.clone();

            let is_timeout = last_received.elapsed().as_secs() > 1;

            if event.kind.is_modify() && is_timeout {
                let subgraph_config = SubGraphConfig::new(&cloned_args);
                match subgraph_config {
                    Ok(config) => {
                        let subgraph_config =
                            Environment::replace_env_vars_in_config(config, cloned_environment);
                        *cloned_config.lock().unwrap() = subgraph_config;
                        last_received = std::time::Instant::now();
                        tx.send(true).unwrap();
                    }
                    Err(error) => {
                        error!(
                            "Something went wrong, waiting for changes. Error Message: {:?}",
                            error.message
                        );
                    }
                };
            }
        },
        notify::Config::default(),
    ) {
        Ok(watcher) => {
            info!("👀 Watching for changes... 👀");
            watcher
        }
        Err(error) => {
            error!(
                "Failed to create watcher. Error Message: {}",
                error.to_string()
            );
            exit(1)
        }
    };

    // Start the watcher based on the path of the config file
    match watcher.watch(
        args.clone().config.unwrap().as_path().parent().unwrap(), // Path to watch
        RecursiveMode::Recursive,
    ) {
        Ok(_) => (),
        Err(e) => error!("Watcher failed to start. Error Message: {e}"),
    };

    // Clone the config and start the server
    // When a message is received, restart the server
    // Store the shutdown handle so that it can be used to shutdown the server
    let cloned_config = Arc::clone(&config);
    let mut shutdown_handle: Option<tokio::sync::oneshot::Sender<()>> = None;
    loop {
        match rx.recv() {
            Ok(is_restart) => {
                // If the server is already running, send a shutdown signal
                // to prevent duplicate servers from running.
                if is_restart && shutdown_handle.is_some() {
                    shutdown_handle.unwrap().send(()).unwrap();
                }

                // Start the server.
                let server_instance =
                    run(args.clone(), cloned_config.lock().unwrap().clone()).await;

                // If the server started successfully, store the shutdown handle
                // and spawn the server on a new thread.
                match server_instance {
                    Ok((server, _schema, shutdown)) => {
                        shutdown_handle = Some(shutdown);
                        tokio::spawn(async move {
                            server.await;
                        });
                    }
                    Err(_) => {
                        shutdown_handle = None;
                        error!("Something went wrong, waiting for changes...")
                    }
                }
            }
            Err(e) => {
                error!("Watcher failed, exiting. Error Message: {}", e.to_string());
                exit(1)
            }
        }
    }
}
