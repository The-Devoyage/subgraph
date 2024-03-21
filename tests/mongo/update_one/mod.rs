use crate::execute;

#[tokio::test]
async fn update_one() {
    let name_uuid = uuid::Uuid::new_v4().to_string();
    let request = format!(
        r#"
        mutation {{
            create_user(create_user_input: {{ values: {{ name: "{}", age: 1, married: false, email: "johnny@johnny.com" }} }}) {{
                data {{
                    _id
                }}
            }}
        }}
        "#,
        name_uuid
    );
    execute(async_graphql::Request::new(request), None).await;
    let request = format!(
        r#"
        mutation {{
            update_user(update_user_input: {{ values: {{ age: 3 }}, query: {{ name: "{}" }} }}) {{
                data {{
                    _id
                }}
            }}
        }}
        "#,
        name_uuid
    );
    let response = execute(async_graphql::Request::new(request), None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn update_one_returns_correct_results() {
    // Reset the database - Ensure test can run with unique data
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_users(update_users_input: { values: { name: "update_one_returns_correct_deprecated" }, query: { name: "update_one_returns_correct" } }) {
                data {
                    _id
                }
            }
        }
        "#,
    );
    execute(request, None).await;

    // Create two users with the same name
    let mut ids = vec![];
    for _ in 0..2 {
        let create_request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_user(create_user_input: {{ values: {{ name: "update_one_returns_correct", age: 3, married: false, email: "testy@gmail.com" }} }}) {{
                        data {{
                            _id
                        }}
                    }}
                }}
            "#,
        ));
        let response = execute(create_request, None).await;
        assert!(response.is_ok());
        let id = response.data.into_json().unwrap()["create_user"]["data"]["_id"]
            .as_str()
            .unwrap()
            .to_string();
        ids.push(id);
    }

    // Attempt to update by matching name - expect error as two users have the same name
    let update_request = r#"
        mutation {
            update_user(update_user_input: { values: { age: 3 }, query: { name: "update_one_returns_correct" } }) {
                data {
                    _id
                }
            }
        }
        "#;

    let request = async_graphql::Request::new(update_request);
    let response = execute(request, None).await;
    assert!(response.is_err());

    // Reset the first id to a deprecated name, so there is only one with the name
    let revert_request = format!(
        r#"
        mutation {{
            update_user(update_user_input: {{ values: {{ name: "update_one_returns_correct_deprecated" }}, query: {{ _id: "{}" }} }}) {{
                data {{
                    _id
                }}
            }}
        }}
        "#,
        ids[0]
    );

    assert!(
        execute(async_graphql::Request::new(revert_request.clone()), None)
            .await
            .is_ok()
    );

    // Attempt to update by matching name - expect success as only one user has the name
    let request = async_graphql::Request::new(update_request);
    let response = execute(request, None).await;
    assert!(response.is_ok());
    let id = response.data.into_json().unwrap()["update_user"]["data"]["_id"]
        .as_str()
        .unwrap()
        .to_string();
    assert_eq!(id, ids[1]);
}
