use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { id: 1 } }) {
                id
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
                id
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { model: "Suub" } }) {
                id
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
                id
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { price: 9898 } }) {
                id
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
                id
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { status: true } }) {
                id
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
                id
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        query {
            get_car(get_car_input: { query: { model: "Mazda", price: 1075, status: false } }) {
                id
                model
                price
                status
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let model = json["get_car"]["model"].as_str().unwrap();
    let price = json["get_car"]["price"].as_i64().unwrap();
    let status = json["get_car"]["status"].as_bool().unwrap();
    assert_eq!(model, "Mazda");
    assert_eq!(price, 1075);
    assert_eq!(status, false);
}
