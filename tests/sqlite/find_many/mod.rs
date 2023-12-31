use crate::execute;

#[tokio::test]
async fn find_many() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { query: { available: true } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn find_many_with_or_filter() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { query: { OR: [{ id: 1 }, { id: 2 }] } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn find_many_with_eager_loading() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { query: { available: true, orders: { status: "success" } } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}
