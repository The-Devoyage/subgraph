use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "Oakley", age: 5, married: false, email: "nickisyourfan@gmail.com" } }) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        {
            get_users(get_users_input: { query: { married: false } }) {
                _id
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
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "BongoWithOrFilter", age: 204, married: false, email: "nickisyourfan@gmail.com" } }) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        {
            get_users(get_users_input: { query: { OR: [{ age: 203 }, { age: 204 }] } }) {
                _id
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
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "FindManyWithAndFilter2", age: 988, married: false, email: "nickisyourfan@gmail.com" } }) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        {
            get_user(get_user_input: { query: { AND: [{ age: 988 }, { married: false }] } }) {
                _id
            }
        }
        "#,
    );

    let response = execute(request, None).await;

    assert!(response.is_ok());
}
