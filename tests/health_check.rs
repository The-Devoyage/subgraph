use std::path::PathBuf;

use async_graphql::dynamic::Schema;
use http::HeaderMap;
use subgraph::configuration::{environment::Environment, subgraph::SubGraphConfig};

#[tokio::test]
async fn test_health_check() {
    let test_args = subgraph::cli_args::CliArgs {
        config: PathBuf::from("./examples/joins-types.toml"),
        port: None,
        log_level: None,
    };
    spawn_app(test_args).await;

    assert!(true);
}

#[tokio::test]
async fn test_two() {
    let text_args = subgraph::cli_args::CliArgs {
        config: PathBuf::from("./examples/joins-types.toml"),
        port: None,
        log_level: None,
    };

    let schema = spawn_app(text_args).await;

    let request = async_graphql::Request::new(
        r#"
        {
            get_user(get_user_input: { name: "Nick" }) {
                _id
            }
        }
        "#,
    );

    let headers = HeaderMap::new();

    let res = schema.execute(request.data(headers)).await;
    // .execute("mutation { create_user(create_user_input: { name: \"Scout\", age: 2, married: false }) { _id } }")
    println!("res: {:?}", res);

    assert!(res.is_ok());
}

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
