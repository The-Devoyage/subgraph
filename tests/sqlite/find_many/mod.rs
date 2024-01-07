use crate::execute;

#[tokio::test]
async fn find_many() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { query: { available: true } }) {
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

#[tokio::test]
async fn find_many_with_or_filter() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { query: { OR: [{ id: 1 }, { id: 2 }] } }) {
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

#[tokio::test]
async fn find_many_with_eager_loading() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { query: { available: true, orders: { status: "success" } } }) {
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

#[tokio::test]
async fn find_many_with_meta_data() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { query: { available: true } }) {
                data {
                    id
                }
                meta {
                    request_id
                    count
                    total
                    page
                    service_name
                    executed_at
                    service_version
                    user_uuid
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let meta = json.get("get_coffees").unwrap().get("meta").unwrap();
    assert!(meta.get("request_id").is_some());
    assert!(meta.get("count").is_some());
    assert!(meta.get("total").is_some());
    assert!(meta.get("page").is_some());
    assert!(meta.get("service_name").is_some());
    assert!(meta.get("executed_at").is_some());
    assert!(meta.get("service_version").is_some());
    assert!(meta.get("user_uuid").is_some());
}
