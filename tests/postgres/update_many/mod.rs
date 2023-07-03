use crate::execute;

#[tokio::test]
async fn update_many() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_comments(update_comments_input: { content: "update_many test", status: false, query: { id: 2 } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}
