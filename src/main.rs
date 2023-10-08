use std::sync::{Arc, Mutex};

use clap::Parser;

use log::{debug, error, info};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use subgraph::{
    cli_args,
    configuration::{environment::Environment, subgraph::SubGraphConfig},
    run, utils,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = cli_args::CliArgs::parse();
    let environment = Environment::new();
    args.handle_flags();

    if args.config.is_some() {
        let mut subgraph_config = match SubGraphConfig::new(&args) {
            Ok(config) => config,
            Err(error) => {
                panic!("Provide Valid Subgraph Config: {:?}", error);
            }
        };

        subgraph_config =
            Environment::replace_env_vars_in_config(subgraph_config, environment.clone());
        utils::logger::Logger::init(&args, &subgraph_config);

        if args.clone().watch {
            let config = Arc::new(Mutex::new(subgraph_config.clone()));
            let cloned_config = Arc::clone(&config);
            let cloned_environment = environment.clone();
            let cloned_args = args.clone();
            let (tx, rx) = std::sync::mpsc::channel::<bool>();

            tx.send(false).unwrap();

            let mut last_received = std::time::Instant::now();

            let mut watcher: RecommendedWatcher = match Watcher::new(
                move |event: Result<Event, notify::Error>| {
                    let event = event.unwrap();
                    let cloned_environment = cloned_environment.clone();

                    let is_timeout = last_received.elapsed().as_secs() > 1;

                    debug!("Timeout: {:?}", is_timeout);

                    if event.kind.is_modify() && is_timeout {
                        debug!("File Changed: {:?}", event);
                        let subgraph_config = SubGraphConfig::new(&cloned_args);
                        match subgraph_config {
                            Ok(config) => {
                                let subgraph_config = Environment::replace_env_vars_in_config(
                                    config,
                                    cloned_environment,
                                );
                                *cloned_config.lock().unwrap() = subgraph_config;
                                last_received = std::time::Instant::now();
                                tx.send(true).unwrap();
                            }
                            Err(error) => {
                                debug!("Error: {:?}", error);
                                error!("Something went wrong, waiting for changes...")
                            }
                        };
                    }
                },
                notify::Config::default(),
            ) {
                Ok(watcher) => {
                    info!("👀 Watching for changes...");
                    watcher
                }
                Err(_) => panic!("ERROR"),
            };

            match watcher.watch(
                args.clone().config.unwrap().as_path().parent().unwrap(),
                RecursiveMode::Recursive,
            ) {
                Ok(_) => (),
                Err(e) => error!("Error watching file: {e}"),
            };

            let cloned_config = Arc::clone(&config);
            let mut shutdown_handle: Option<tokio::sync::oneshot::Sender<()>> = None;

            loop {
                match rx.recv() {
                    Ok(is_restart) => {
                        if is_restart && shutdown_handle.is_some() {
                            shutdown_handle.unwrap().send(()).unwrap();
                        }

                        let server_options =
                            run(args.clone(), cloned_config.lock().unwrap().clone()).await;

                        match server_options {
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
                    Err(_) => error!("Error receiving message"),
                }
            }
        }
        let (server, _schema, _tx) = run(args.clone(), subgraph_config).await?;
        server.await;
        Ok(())
    } else {
        Ok(())
    }
}
