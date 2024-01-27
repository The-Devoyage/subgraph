use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { id: 1 } }) {
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
        mutation {
            create_car(create_car_input: { values: { model: "Suub", price: 1000, status: true } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { model: "Suub" } }) {
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
        mutation {
            create_car(create_car_input: { values: { model: "Tesla", price: 9898, status: true } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { price: 9898 } }) {
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
        mutation {
            create_car(create_car_input: { values: { model: "Ford", price: 97, status: true } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { status: true } }) {
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
        mutation {
            create_car(create_car_input: { values: { model: "Mazda", price: 1075, status: false } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { model: "Mazda", price: 1075, status: false } }) {
                data {
                    id
                    model
                    price
                    status
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let model = json["get_car"]["data"]["model"].as_str().unwrap();
    let price = json["get_car"]["data"]["price"].as_i64().unwrap();
    let status = json["get_car"]["data"]["status"].as_bool().unwrap();
    assert_eq!(model, "Mazda");
    assert_eq!(price, 1075);
    assert_eq!(status, false);
}

#[tokio::test]
async fn returns_correct_scalars_uuid_date() {
    let uuid = uuid::Uuid::new_v4().to_string();
    let order_date = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let request = async_graphql::Request::new(format!(
        r#"
            mutation {{
                create_car_purchase(create_car_purchase_input: {{ values: {{ price: 98765, buyer: "Bongo", status: "sold", uuid: "{}", order_date: "{}" }} }}) {{
                    data {{
                        id
                    }}
                }}
            }}
            "#,
        uuid, order_date
    ));
    execute(request, None).await;

    let request = async_graphql::Request::new(format!(
        r#"
            query {{
                get_car_purchase(get_car_purchase_input: {{ query: {{ uuid: "{}" }} }}) {{
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
    let uuid = json["get_car_purchase"]["data"]["uuid"].as_str().unwrap();
    let order_date = json["get_car_purchase"]["data"]["order_date"]
        .as_str()
        .unwrap();
    assert_eq!(uuid, uuid);
    assert_eq!(order_date, order_date);
}

#[tokio::test]
async fn find_one_with_or_filter() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { OR: [{ id: 1 }, { id: 2 }] } }) {
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
async fn find_one_with_and_filter() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { AND: [{ id: 1 }, { id: 2 }] } }) {
                data {
                    id
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_err());
}

#[tokio::test]
async fn find_one_with_virtual_field() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { id: 1, virtual_id: "me too" } }) {
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
