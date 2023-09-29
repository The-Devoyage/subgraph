use log::debug;

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

#[tokio::test]
async fn join_sqlite_to_mongo() {
    //Find any user to get a valid uuid
    let request = async_graphql::Request::new(
        r#"
        query {
            get_user(get_user_input: {}) {
                _id 
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let user = json.get("get_user").unwrap();
    let user_id = user.get("_id").unwrap();
    println!("UserID: {}", user_id);
    let graphql_query = format!(
        r#"
            mutation {{
                create_coffee(create_coffee_input: {{
                    name: "KatzWithUser",
                    price: 15,
                    available: true,
                    created_by: {}
                }}) {{
                    id
                }}
            }}
        "#,
        user_id
    );
    println!("Query: {}", graphql_query);

    //Insert new coffee, with valid created_by
    let request = async_graphql::Request::new(graphql_query);
    let response = execute(request, None).await;
    assert!(response.is_ok());

    //Find the coffee, join on the created_by field to populate the data.
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffee(get_coffee_input: { name: "KatzWithUser" }) {
                id
                name
                price
                available
                created_by(created_by: {}) {
                    _id
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let coffee = json.get("get_coffee").unwrap();
    //compare the created_by._id field with the user_id from the first query
    assert_eq!(
        coffee.get("created_by").unwrap().get("_id").unwrap(),
        user_id
    );
}
