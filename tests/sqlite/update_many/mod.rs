use crate::execute;

#[tokio::test]
async fn update_many() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_coffee(create_coffee_input: { name: "Starbucks", price: 5, available: true }) {
                id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_coffees(update_coffees_input: { price: 7, query: { name: "Starbucks" } }) {
                id
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
}
