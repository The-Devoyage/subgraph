use crate::execute;

#[tokio::test]
async fn update_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_todo(update_todo_input: { values: { completed: false }, query: { id: 1 } }) {
                data {
                    id
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}
