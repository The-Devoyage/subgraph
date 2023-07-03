use crate::execute;

#[tokio::test]
async fn create_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { name: "Nick", age: 30, married: true }) {
                _id
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
}
