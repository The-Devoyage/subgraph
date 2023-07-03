use crate::execute;

#[tokio::test]
async fn update_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { name: "Johnny", age: 1, married: false}) {
                _id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_user(update_user_input: { age: 3, query: { name: "Johnny" } }) {
                _id
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
}
