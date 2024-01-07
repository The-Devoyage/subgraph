use crate::execute;

#[tokio::test]
async fn update_many() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_coffee(create_coffee_input: { values: { name: "Starbucks", price: 5, available: true, created_by: "6510865e93142f6d61b10dd8" } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_coffees(update_coffees_input: { values: { price: 7 }, query: { name: "Starbucks" } }) {
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
