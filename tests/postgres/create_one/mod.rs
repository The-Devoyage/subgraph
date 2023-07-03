use crate::execute;

#[tokio::test]
async fn create_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { content: "create_one test", status: true }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}
