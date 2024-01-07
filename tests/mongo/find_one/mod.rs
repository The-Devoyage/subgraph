use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "Bongo", age: 10, married: false, email: "nickisyourfan@gmail.com" } }) {
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
        {
            get_user(get_user_input: { query: { name: "Bongo", age: 10, married: false, email: "nickisyourfan@gmail.com" } }) {
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
async fn find_one_by_string() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "Squirrel", age: 7, married: false, email: "squirrel@noemail.com" } }) {
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
        {
            get_user(get_user_input: { query: { name: "Squirrel" } }) {
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
async fn find_one_by_int() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: name: "Turtle", age: 77, married: false, email: "turtle@noemail.com" } }) {
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
        {
            get_user(get_user_input: { query: { age: 77 } }) {
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
async fn find_one_by_bool() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "Jackson", age: 14, married: true, email: "jackson@noemail.com" } }) {
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
        {
            get_user(get_user_input: { query: { married: true } }) {
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
async fn returns_correct_scalars() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "Jordan", age: 2, married: true, email: "jordan@noemail.com" } }) {
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
        {
            get_user(get_user_input: { query: { name: "Jordan" } }) {
                data {
                    _id
                    name
                    age
                    married
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
    let json = response.data.into_json().unwrap();
    let name = json["get_user"]["data"]["name"].as_str().unwrap();
    let object_id = json["get_user"]["data"]["_id"].as_str().unwrap();
    let age = json["get_user"]["data"]["age"].as_i64().unwrap();
    let married = json["get_user"]["data"]["married"].as_bool().unwrap();
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
                values: {
                    name: "Nested Mosiac", 
                    ratings: [5, 4],
                    brand: { 
                        name: "Community" 
                    } 
                }
            }) {
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
        {
            get_beer(get_beer_input: { query: { name: "Nested Mosiac" } }) {
                data {
                    _id
                    name
                    ratings
                    brand {
                        name
                    }
                }
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let name = json["get_beer"]["data"]["name"].as_str().unwrap();
    let brand_name = json["get_beer"]["data"]["brand"]["name"].as_str().unwrap();
    let ratings = json["get_beer"]["data"]["ratings"].as_array().unwrap();
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
            values: {
                name: "Mosiac", 
                ratings: [5, 4, 5, 4, 3],
                brand: { 
                    name: "Community" 
                } 
            }
            }) {
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
        {
            get_beer(get_beer_input: { query: { brand: { name: "Community" } } }) {
                data {
                    _id
                    ratings
                    brand {
                        name
                    }
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
            values: {
                name: "Mosiac", 
                ratings: [5, 4, 5, 4, 3],
                brand: { 
                    name: "Community" 
                } 
            }
            }) {
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
        {
            get_beer(get_beer_input: { query: { ratings: [5] } }) {
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
async fn find_joined_to_mongo_ds() {
    let owner = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { 
                values: {
                    name: "Laura", 
                    age: 33,
                    married: true,
                    email: "laura@laura.com"
                }
            }) {
                data {
                    _id
                }
            }
        }
        "#,
    );
    let user_response = execute(owner, None).await;
    assert!(user_response.is_ok());
    let user_json = user_response.data.into_json().unwrap();
    let user_id = user_json["create_user"]["data"]["_id"].as_str().unwrap();
    println!("user_id: {}", user_id);

    let fav_car = async_graphql::Request::new(
        r#"
        mutation {
            create_car(create_car_input: { values: { model: "Camero", price: 1, status: true } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    let car_response = execute(fav_car, None).await;
    assert!(car_response.is_ok());
    let car_json = car_response.data.into_json().unwrap();
    let car_id = car_json["create_car"]["data"]["id"].as_i64().unwrap();

    let create_coffee_mutation = format!(
        r#"
        mutation {{
            create_coffee(create_coffee_input: {{ values: {{ name: "Ascension", price: 14, available: false, created_by: "{}" }} }}) {{
                data {{
                    id
                }}
            }}
        }}
        "#,
        user_id
    );
    let fav_coffee = async_graphql::Request::new(create_coffee_mutation);
    let coffee_response = execute(fav_coffee, None).await;
    assert!(coffee_response.is_ok());
    let coffee_json = coffee_response.data.into_json().unwrap();
    let coffee_id = coffee_json["create_coffee"]["data"]["id"].as_i64().unwrap();

    let comment_one = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { values: { content: "join_one test", status: true } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    let comment_one_response = execute(comment_one, None).await;
    assert!(comment_one_response.is_ok());
    let comment_one_json = comment_one_response.data.into_json().unwrap();
    let comment_one_id = comment_one_json["create_comment"]["data"]["id"]
        .as_i64()
        .unwrap();

    let comment_two = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { values: { content: "join_two test", status: true } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    let comment_two_response = execute(comment_two, None).await;
    assert!(comment_two_response.is_ok());
    let comment_two_json = comment_two_response.data.into_json().unwrap();
    let comment_two_id = comment_two_json["create_comment"]["data"]["id"]
        .as_i64()
        .unwrap();

    println!(
        "{}, {}, {}, {}, {}",
        user_id, car_id, coffee_id, comment_one_id, comment_two_id
    );

    let request = async_graphql::Request::new(format!(
        r#"
        mutation {{
            create_dog(create_dog_input: {{
                values: {{
                    name: "Buddy",
                    age: 5,
                    owner: "{}",
                    fav_car: {},
                    fav_coffee: {},
                    todo: 1,
                    comments: [{}, {}]
                }}
            }}) {{
                data {{
                    _id
                }}
            }}
        }}
        "#,
        user_id, car_id, coffee_id, comment_one_id, comment_two_id
    ));

    let response = execute(request, None).await;
    assert!(response.is_ok());

    let dog_json = response.data.into_json().unwrap();
    let dog_id = dog_json["create_dog"]["data"]["_id"].as_str().unwrap();
    println!("DogID: {}", dog_id);

    let request = async_graphql::Request::new(format!(
        r#"
        {{
            get_dog(get_dog_input: {{ query: {{ _id: "{}" }} }}) {{
                data {{
                    _id
                    name
                    age
                    owner(owner: {{ query: {{}} }} ) {{
                        data {{
                            _id
                            name
                            age
                            married
                        }}
                    }}
                    fav_car(fav_car: {{ query: {{}} }} ) {{
                        data {{
                            id
                            model
                            price
                            status
                        }}
                    }}
                    fav_coffee( fav_coffee: {{ query: {{}} }} ) {{
                        data {{
                            id
                            name
                            price
                            available
                        }}
                    }}
                    todo(todo: {{ query: {{}} }}) {{
                        data {{
                            id
                            userId
                            title
                            completed
                        }}
                    }}
                    comments(comments: {{ query: {{}} }}) {{
                        data {{
                            id
                            content
                            status
                        }}
                    }}
                }}
            }}
        }}
        "#,
        dog_id.to_string()
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
                values: {
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
                }
            }) {
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
        {
            get_user(get_user_input: { query: { address: { line_one: "address lineone" } } }) {
                data {
                    _id
                    address {
                        line_one
                    }
                }
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());

    let json = response.data.into_json().unwrap();
    let address = json["get_user"]["data"]["address"].clone();
    assert_eq!(address["line_one"], "address lineone");
}

#[tokio::test]
async fn resolve_typename() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "BongoWithTypeName", age: 10, married: false, email: "nickisyourfan@gmail.com" } }) {
                data {
                    _id
                    __typename
                }
                __typename
            }
        }
        "#,
    );
    let response = execute(request, None).await;

    let json = response.data.into_json().unwrap();
    let typename = json["create_user"]["data"]["__typename"].as_str().unwrap();
    assert_eq!(typename, "user");
}

#[tokio::test]
async fn find_one_with_or_filter() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "BongoWithOrFilter", age: 199, married: false, email: "nickisyourfan@gmail.com" } }) {
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
            create_user(create_user_input: { values: { name: "BongoWithOrFilter", age: 198, married: false, email: "nickisyourfan@gmail.com" } }) {
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
        {
            get_user(get_user_input: { query: { OR: [{ age: 199 }, { age: 198 }] } }) {
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
async fn find_one_with_and_filter() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "BongoWithAndFilter", age: 986, married: false, email: "nickisyourfan@gmail.com" } }) {
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
        {
            get_user(get_user_input: { query: { AND: [{ age: 986 }, { married: false }] } }) {
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
async fn find_one_with_virtual_field() {
    let request = async_graphql::Request::new(
        r#"
        {
            get_user(get_user_input: { query: { virtual_id: "is virtual" } }) {
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
