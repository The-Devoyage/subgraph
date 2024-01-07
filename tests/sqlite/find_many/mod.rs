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
                    total_count
                    page
                    total_pages
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
    assert!(meta.get("total_count").is_some());
    assert!(meta.get("page").is_some());
    assert!(meta.get("total_pages").is_some());
    assert!(meta.get("service_name").is_some());
    assert!(meta.get("executed_at").is_some());
    assert!(meta.get("service_version").is_some());
    assert!(meta.get("user_uuid").is_some());
}

#[tokio::test]
async fn find_many_with_pagination() {
    // Create 10 coffees with the same name
    for _i in 1..=10 {
        let request = async_graphql::Request::new(
            r#"
                mutation {
                    create_coffee(create_coffee_input: { values: { name: "Pagination Test", price: 12, available: true, created_by: "6510865e93142f6d61b10dd8" } }) {
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

    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { query: { name: "Pagination Test" }, opts: { page: 1, per_page: 3 } }) {
                data {
                    id
                }
                meta {
                    count
                    total_count
                    page
                    total_pages
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let meta = json.get("get_coffees").unwrap().get("meta").unwrap();
    assert_eq!(meta.get("count").unwrap().as_i64().unwrap(), 3);
    assert!(meta.get("total_count").unwrap().as_i64().unwrap() > 9);
    assert_eq!(meta.get("page").unwrap().as_i64().unwrap(), 1);

    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { query: { name: "Pagination Test" }, opts: { page: 2, per_page: 3 } }) {
                data {
                    id
                }
                meta {
                    count
                    total_count
                    page
                    total_pages
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let meta = json.get("get_coffees").unwrap().get("meta").unwrap();
    assert_eq!(meta.get("count").unwrap().as_i64().unwrap(), 3);
    assert!(meta.get("total_count").unwrap().as_i64().unwrap() > 9);
    assert_eq!(meta.get("page").unwrap().as_i64().unwrap(), 2);
}
