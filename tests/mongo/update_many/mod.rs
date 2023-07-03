use crate::execute;

#[tokio::test]
async fn update_many() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { name: "Steve", age: 2, married: false}) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { name: "Steve", age: 1, married: false}) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_users(update_users_input: { age: 3, query: { name: "Steve" } }) {
                _id
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
}
