use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { name: "Bongo", age: 10, married: false }) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        {
            get_user(get_user_input: { name: "Bongo", age: 10, married: false }) {
                _id
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
        {
            get_user(get_user_input: { name: "Foo", age: 100, married: true}) {
                _id
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
        mutation {
            create_user(create_user_input: { name: "Squirrel", age: 7, married: false }) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        {
            get_user(get_user_input: { name: "Squirrel" }) {
                _id
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
            create_user(create_user_input: { name: "Turtle", age: 77, married: false }) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        {
            get_user(get_user_input: { age: 77 }) {
                _id
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
            create_user(create_user_input: { name: "Jackson", age: 14, married: true }) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        {
            get_user(get_user_input: { married: true }) {
                _id
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
            create_user(create_user_input: { name: "Jordan", age: 2, married: true}) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        {
            get_user(get_user_input: { name: "Jordan" }) {
                _id
                name
                age
                married
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
    let json = response.data.into_json().unwrap();
    let name = json["get_user"]["name"].as_str().unwrap();
    let object_id = json["get_user"]["_id"].as_str().unwrap();
    let age = json["get_user"]["age"].as_i64().unwrap();
    let married = json["get_user"]["married"].as_bool().unwrap();
    assert_eq!(name, "Jordan");
    assert_eq!(object_id.len(), 24);
    assert_eq!(age, 2);
    assert_eq!(married, true);
}

#[tokio::test]
async fn find_one_by_nested_object() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_beer(create_beer_input: { 
                name: "Mosiac", 
                ratings: [5, 4, 5, 4, 3],
                brand: { 
                    name: "Community" 
                } 
            }) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        {
            get_beer(get_beer_input: { brand: { name: "Community" } }) {
                _id
            }
        }
        "#,
    );
    let response = execute(request, None).await;

    assert!(response.is_ok());
}

#[tokio::test]
async fn find_one_by_list() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_beer(create_beer_input: { 
                name: "Mosiac", 
                ratings: [5, 4, 5, 4, 3],
                brand: { 
                    name: "Community" 
                } 
            }) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        {
            get_beer(get_beer_input: { ratings: [5] }) {
                _id
            }
        }
        "#,
    );
    let response = execute(request, None).await;

    assert!(response.is_ok());
}
