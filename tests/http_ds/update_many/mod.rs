use crate::execute;

#[tokio::test]
async fn update_many() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_todos(update_todos_input: { completed: false, query: { userId: 1 } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}
