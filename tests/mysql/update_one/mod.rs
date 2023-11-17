use crate::execute;

#[tokio::test]
async fn update_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_car(update_car_input: { values: { status: false }, query: { id: 1 } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}
