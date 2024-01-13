use crate::execute;

#[tokio::test]
async fn update_many() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "Steve", age: 2, married: false, email: "steve@noemail.com" } }) {
                data {
                    _id
                }
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "Steve", age: 1, married: false, email: "steve@noemail.com" } }) {
                data {
                    _id
                }
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_users(update_users_input: { values: { age: 3 }, query: { name: "Steve" } }) {
                data {
                    _id
                }
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn update_many_returns_correct_results() {
    let uuid = format!("{}_test", uuid::Uuid::new_v4().to_string());
    println!("NAME: {}", uuid);
    let mut ids = vec![];
    for _ in 0..4 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_user(create_user_input: {{ values: {{ name: "{}", age: 234, married: false, email: "abcdefg@gmail.com" }} }}) {{
                        data {{
                            _id
                            name
                        }}
                    }}
                }}
                "#,
            uuid
        ));
        let response = execute(request, None).await;
        let data = response.data.into_json().unwrap();
        let data = data.get("create_user").unwrap();
        let data = data.get("data").unwrap();
        let data = data.get("_id").unwrap();
        let id = data.as_str().unwrap();
        ids.push(id.to_string());
    }

    let request = async_graphql::Request::new(format!(
        r#"
            mutation {{
                update_users(update_users_input: {{ values: {{ age: 3 }}, query: {{ OR: [{{ name: "{}" }}, {{ name: "idonexist" }}] }} }}) {{
                    data {{
                        _id
                        name
                    }}
                }}
            }}
            "#,
        uuid
    ));

    let response = execute(request, None).await;
    let data = response.data.into_json().unwrap();
    let data = data.get("update_users").unwrap();
    let data = data.get("data").unwrap();
    let data = data.as_array().unwrap();
    let data = data
        .iter()
        .map(|x| x.get("_id").unwrap().as_str().unwrap().to_string())
        .collect::<Vec<String>>();
    assert_eq!(data.len(), 4);
    for id in ids {
        assert!(data.contains(&id));
    }
}
