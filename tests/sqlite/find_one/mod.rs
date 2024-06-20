use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { query: { name: "Katz", price: 15, available: true } }) {
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
async fn find_one_by_string() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { query: { name: "Katz" } }) {
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
async fn find_one_by_int() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { query: { price: 15 } }) {
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
async fn find_one_by_bool() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { query: { available: true } }) {
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
async fn returns_correct_scalars() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { query: { name: "Katz" } }) {
                data {
                    id
                    name
                    price
                    available
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let coffee = json.get("get_coffee").unwrap();
    assert_eq!(coffee["data"].get("id").unwrap(), 1);
    assert_eq!(
        coffee["data"].get("name").unwrap(),
        &serde_json::json!("Katz")
    );
    assert_eq!(coffee["data"].get("price").unwrap(), &serde_json::json!(15));
    assert_eq!(
        coffee["data"].get("available").unwrap(),
        &serde_json::json!(true)
    );
}

#[tokio::test]
async fn returns_correct_scalars_date_uuid() {
    // Create new coffee order
    let uuid = uuid::Uuid::new_v4().to_string();
    let request = async_graphql::Request::new(format!(
        r#"
            mutation {{
                create_coffee_order(create_coffee_order_input: {{ values: {{  created_by: "6510865e93142f6d61b10dd8", uuid: "{}" }} }}) {{
                    data {{
                        id
                    }}
                }}
            }}
            "#,
        uuid
    ));
    let response = execute(request, None).await;
    assert!(response.is_ok());
    let id = response.data.into_json().unwrap()["create_coffee_order"]["data"]["id"]
        .as_i64()
        .unwrap();

    // Get coffee order
    let request = async_graphql::Request::new(format!(
        r#"
            query {{
                get_coffee_order(get_coffee_order_input: {{ query: {{ uuid: "{}" }} }}) {{
                    data {{
                        id
                        uuid
                        order_date
                    }}
                }}
            }}
            "#,
        uuid
    ));

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let coffee_order = json.get("get_coffee_order").unwrap();
    assert_eq!(coffee_order["data"].get("id").unwrap(), id);
    assert_eq!(
        coffee_order["data"].get("uuid").unwrap(),
        &serde_json::json!(uuid)
    );
    assert!(coffee_order["data"].get("order_date").unwrap().is_string());
}

#[tokio::test]
async fn join_sqlite_to_mongo() {
    //Find any user to get a valid uuid
    let request = async_graphql::Request::new(
        r#"
        query {
            get_user(get_user_input: { query: {} }) {
                data {
                    _id
                }
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let user = json.get("get_user").unwrap();
    let user_id = user["data"].get("_id").unwrap();
    let coffee_name = format!("KatzWithUser-{}", uuid::Uuid::new_v4().to_string());
    println!("UserID: {}", user_id);
    let graphql_query = format!(
        r#"
            mutation {{
                create_coffee(create_coffee_input: {{
                    values: {{
                        name: "{}",
                        price: 15,
                        available: true,
                        created_by: {}
                    }}
                }}) {{
                    data {{
                        id
                    }}
                }}
            }}
        "#,
        coffee_name, user_id
    );
    //Insert new coffee, with valid created_by
    let request = async_graphql::Request::new(graphql_query);
    let response = execute(request, None).await;
    assert!(response.is_ok());

    //Find the coffee, join on the created_by field to populate the data.
    // let request = async_graphql::Request::new(
    //     r#"
    //     query {
    //         get_coffee(get_coffee_input: { query: { name: "KatzWithUser" } }) {
    //             data {
    //                 id
    //                 name
    //                 price
    //                 available
    //                 created_by(created_by: { query: {} }) {
    //                     data {
    //                         _id
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     "#,
    // );
    let request = async_graphql::Request::new(format!(
        r#"
        query {{
            get_coffee(get_coffee_input: {{ query: {{ name: "{}" }} }}) {{
                data {{
                    id
                    name
                    price
                    available
                    created_by(created_by: {{ query: {{}} }}) {{
                        data {{
                            _id
                        }}
                    }}
                }}
            }}
        }}
        "#,
        coffee_name
    ));

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let coffee = json.get("get_coffee").unwrap();
    //compare the created_by._id field with the user_id from the first query
    assert_eq!(
        coffee["data"].get("created_by").unwrap()["data"]
            .get("_id")
            .unwrap(),
        user_id
    );
}

#[tokio::test]
async fn find_one_with_and_filter() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { query: { AND: [{ name: "Katz" }, { price: 15 }] } }) {
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
async fn find_one_with_or_filter() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { query: { OR: [{ name: "Katz" }, { price: 15 }] } }) {
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
async fn find_one_with_virtual_field() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { query: { name: "Katz", virtual_id: "im virtual" } }) {
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
