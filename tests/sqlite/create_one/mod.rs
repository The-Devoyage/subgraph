use crate::execute;

#[tokio::test]
async fn create_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_coffee(create_coffee_input: { name: "Colombe", price: 12, available: true, created_by: "23456" }) {
                id
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
}
