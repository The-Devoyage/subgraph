use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { name: "Oakley", age: 5, married: false }) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        {
            get_users(get_users_input: { married: false }) {
                _id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}
