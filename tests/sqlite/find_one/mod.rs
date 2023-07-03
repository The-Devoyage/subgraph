use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { name: "Katz", price: 15, available: true }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn find_one_fails() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { name: "coffee_failure", price: 15, available: false }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_err());
}

#[tokio::test]
async fn find_one_by_string() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { name: "Katz" }) {
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
        query {
            get_coffee(get_coffee_input: { price: 15 }) {
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
        query {
            get_coffee(get_coffee_input: { available: true }) {
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
        query {
            get_coffee(get_coffee_input: { name: "Katz" }) {
                id
                name
                price
                available
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let coffee = json.get("get_coffee").unwrap();
    assert_eq!(coffee.get("id").unwrap(), 1);
    assert_eq!(coffee.get("name").unwrap(), &serde_json::json!("Katz"));
    assert_eq!(coffee.get("price").unwrap(), &serde_json::json!(15));
    assert_eq!(coffee.get("available").unwrap(), &serde_json::json!(true));
}
