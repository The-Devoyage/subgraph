use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "Oakley", age: 5, married: false, email: "nickisyourfan@gmail.com" } }) {
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
            get_users(get_users_input: { query: { married: false } }) {
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
async fn find_many_with_or_filter() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "BongoWithOrFilter", age: 203, married: false, email: "nickisyourfan@gmail.com" } }) {
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
            create_user(create_user_input: { values: { name: "BongoWithOrFilter", age: 204, married: false, email: "nickisyourfan@gmail.com" } }) {
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
            get_users(get_users_input: { query: { OR: [{ age: 203 }, { age: 204 }] } }) {
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
async fn find_many_with_and_filter() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "FindManyWithAndFilter", age: 988, married: false, email: "nickisyourfan@gmail.com" } }) {
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
            create_user(create_user_input: { values: { name: "FindManyWithAndFilter2", age: 988, married: false, email: "nickisyourfan@gmail.com" } }) {
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
            get_user(get_user_input: { query: { AND: [{ age: 988 }, { married: false }] } }) {
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
async fn find_many_with_eager_loading() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "FindManyWithEagerLoading", age: 57, married: true, email: "abcdefg@fakemail.com" } }) {
                data {
                    _id
                }
            }   
        }
        "#,
    );
    let response = execute(request, None).await;
    let data = response.data.into_json().unwrap();
    let user_id = data.get("create_user").unwrap()["data"]
        .as_object()
        .unwrap()
        .get("_id")
        .unwrap()
        .as_str()
        .unwrap();

    let request = async_graphql::Request::new(format!(
        r#"
            mutation {{
                create_user_access(create_user_access_input: {{ values: {{ user_id: "{}", view: true }} }}) {{
                    data {{
                        _id
                    }}
                }}   
            }}
            "#,
        user_id
    ));
    execute(request, None).await;

    let request = async_graphql::Request::new(format!(
        r#"
            {{
                get_users(get_users_input: {{ query: {{ user_access: {{ user_id: "{}", view: true }} }} }}) {{
                    data {{
                        _id
                        name
                    }}
                }}
            }}
            "#,
        user_id
    ));

    let response = execute(request, None).await;

    let data = response.data.into_json().unwrap();
    let users = data.get("get_users").unwrap()["data"].as_array().unwrap();
    let user = users.get(0).unwrap().as_object().unwrap();
    let name = user.get("name").unwrap().as_str().unwrap();

    assert_eq!(name, "FindManyWithEagerLoading");
}
