use crate::execute;

#[tokio::test]
async fn update_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_car(update_car_input: { values: { status: false }, query: { id: 1 } }) {
                data {
                    id
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn update_one_returns_correct_results() {
    // Reset the database - Ensure test can run with unique data
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_cars(update_cars_input: { values: { model: "update_one_returns_correct_deprecated" }, query: { model: "update_one_returns_correct" } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    execute(request, None).await;

    // Create two cars with the same model
    let mut ids = vec![];
    for _ in 0..2 {
        let create_request = async_graphql::Request::new(format!(
            r#"
            mutation {{
                create_car(create_car_input: {{ values: {{ model: "update_one_returns_correct", price: 12345, status: true }} }}) {{
                    data {{
                        id
                    }}
                }}
            }}
        "#,
        ));
        let response = execute(create_request, None).await;
        assert!(response.is_ok());
        let id = response.data.into_json().unwrap()["create_car"]["data"]["id"]
            .as_i64()
            .unwrap();
        ids.push(id);
    }

    // Attempt to update by matching model - expect error as two cars have the same model
    let update_request = r#"
            mutation {
              update_car(
                update_car_input: {
                  values: { model: "update_one_returns_correct_deprecatd" }
                  query: {
                    OR: [
                      { model: "update_one_returns_correct" }
                      { model: "does-not-exist" }
                    ]
                  }
                }
              ) {
                data {
                  id
                }
              }
            }
        "#;

    let response = execute(
        async_graphql::Request::new(update_request.to_string()),
        None,
    )
    .await;
    assert!(response.is_err());

    // Reset the first car to have a unique model, run the update again - expect success
    let request = async_graphql::Request::new(format!(
        r#"
            mutation {{
                update_car(update_car_input: {{ values: {{ model: "update_one_returns_correct_deprecated" }}, query: {{ id: {} }} }}) {{
                    data {{
                        id
                    }}
                }}
            }}
        "#,
        ids[0]
    ));
    execute(request, None).await;

    // Run the update again - expect success
    let response = execute(
        async_graphql::Request::new(update_request.to_string()),
        None,
    )
    .await;

    assert!(response.is_ok());
    let data = response.data.into_json().unwrap();
    let updated_id = data["update_car"]["data"]["id"].as_i64().unwrap();
    assert_eq!(updated_id, ids[1]);
}
