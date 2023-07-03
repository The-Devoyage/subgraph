use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_cars(get_cars_input: { id: 1 }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}
