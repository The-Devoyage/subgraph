use crate::execute;

#[tokio::test]
async fn find_many() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { available: true }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}
