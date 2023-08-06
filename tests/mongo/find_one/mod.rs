use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { name: "Bongo", age: 10, married: false, email: "nickisyourfan@gmail.com" }) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        {
            get_user(get_user_input: { name: "Bongo", age: 10, married: false, email: "nickisyourfan@gmail.com" }) {
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
            get_user(get_user_input: { name: "Foo", age: 100, married: true }) {
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
            create_user(create_user_input: { name: "Squirrel", age: 7, married: false, email: "squirrel@noemail.com" }) {
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
            create_user(create_user_input: { name: "Turtle", age: 77, married: false, email: "turtle@noemail.com" }) {
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
            create_user(create_user_input: { name: "Jackson", age: 14, married: true, email: "jackson@noemail.com" }) {
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
            create_user(create_user_input: { name: "Jordan", age: 2, married: true, email: "jordan@noemail.com" }) {
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
async fn resolve_nested_object() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_beer(create_beer_input: { 
                name: "Nested Mosiac", 
                ratings: [5, 4],
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
            get_beer(get_beer_input: { name: "Nested Mosiac" }) {
                _id
                name
                ratings
                brand {
                    name
                }
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let name = json["get_beer"]["name"].as_str().unwrap();
    let brand_name = json["get_beer"]["brand"]["name"].as_str().unwrap();
    let ratings = json["get_beer"]["ratings"].as_array().unwrap();
    assert_eq!(name, "Nested Mosiac");
    assert_eq!(brand_name, "Community");
    assert_eq!(ratings.len(), 2);
    assert_eq!(ratings[0].as_i64().unwrap(), 5);
    assert_eq!(ratings[1].as_i64().unwrap(), 4);
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
                ratings
                brand {
                    name
                }
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

#[tokio::test]
async fn find_joined_to_mongo_ds() {
    let owner = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { 
                name: "Laura", 
                age: 33,
                married: true,
                email: "laura@laura.com"
            }) {
                _id
            }
        }
        "#,
    );
    let user_response = execute(owner, None).await;
    let user_json = user_response.data.into_json().unwrap();
    let user_id = user_json["create_user"]["_id"].as_str().unwrap();

    let fav_car = async_graphql::Request::new(
        r#"
        mutation {
            create_car(create_car_input: { model: "Camero", price: 1, status: true }) {
                id
            }
        }
        "#,
    );
    let car_response = execute(fav_car, None).await;
    let car_json = car_response.data.into_json().unwrap();
    let car_id = car_json["create_car"]["id"].as_i64().unwrap();

    let fav_coffee = async_graphql::Request::new(
        r#"
        mutation {
            create_coffee(create_coffee_input: { name: "Ascension", price: 14, available: false }) {
                id
            }
        }
        "#,
    );
    let coffee_response = execute(fav_coffee, None).await;
    let coffee_json = coffee_response.data.into_json().unwrap();
    let coffee_id = coffee_json["create_coffee"]["id"].as_i64().unwrap();

    let comment_one = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { content: "join_one test", status: true }) {
                id
            }
        }
        "#,
    );
    let comment_one_response = execute(comment_one, None).await;
    let comment_one_json = comment_one_response.data.into_json().unwrap();
    let comment_one_id = comment_one_json["create_comment"]["id"].as_i64().unwrap();

    let comment_two = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { content: "join_two test", status: true }) {
                id
            }
        }
        "#,
    );
    let comment_two_response = execute(comment_two, None).await;
    let comment_two_json = comment_two_response.data.into_json().unwrap();
    let comment_two_id = comment_two_json["create_comment"]["id"].as_i64().unwrap();

    println!(
        "{}, {}, {}, {}, {}",
        user_id, car_id, coffee_id, comment_one_id, comment_two_id
    );

    let request = async_graphql::Request::new(format!(
        r#"
        mutation {{
            create_dog(create_dog_input: {{
                name: "Buddy",
                age: 5,
                owner: "{}",
                fav_car: {},
                fav_coffee: {},
                todo: 1,
                comments: [{}, {}]
            }}) {{
                _id
            }}
        }}
        "#,
        user_id, car_id, coffee_id, comment_one_id, comment_two_id
    ));

    let response = execute(request, None).await;

    let dog_json = response.data.into_json().unwrap();
    let dog_id = dog_json["create_dog"]["_id"].as_str().unwrap();
    println!("DogID: {}", dog_id);

    let request = async_graphql::Request::new(format!(
        r#"
        {{
            get_dog(get_dog_input: {{ _id: "{}" }}) {{
                _id
                name
                age
                owner(owner: {{}}) {{
                    _id
                    name
                    age
                    married
                }}
                fav_car(fav_car: {{}}) {{
                    id
                    model
                    price
                    status
                }}
                fav_coffee(fav_coffee: {{}}) {{
                    id
                    name
                    price
                    available
                }}
                todo(todo: {{}}) {{
                    id
                    userId
                    title
                    completed
                }}
                comments(comments: {{}}) {{
                    id
                    content
                    status
                }}
            }}
        }}
        "#,
        dog_id
    ));

    let response = execute(request, None).await;

    assert!(response.is_ok());
}

#[tokio::test]
async fn find_with_nested_object() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { 
                name: "Rory", 
                age: 22, 
                married: false, 
                email: "rory@rory.com",
                address: {
                    line_one: "address lineone",
                    line_two: "address linetwo",
                    city: "address city",
                    state: "address state",
                    zip: "address zip"
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
            get_user(get_user_input: { address: { line_one: "address lineone" } }) {
                _id
            }
        }
        "#,
    );
    let response = execute(request, None).await;

    assert!(response.is_ok());
}
