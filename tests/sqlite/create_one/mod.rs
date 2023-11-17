use crate::execute;

#[tokio::test]
async fn create_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_coffee(create_coffee_input: { values: { name: "Colombe", price: 12, available: true, created_by: "6510865e93142f6d61b10dd8" } }) {
                id
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
}
