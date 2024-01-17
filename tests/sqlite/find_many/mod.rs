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

#[tokio::test]
async fn find_many_with_sorting() {
    // Create several coffees that can be sorted first by price and second by name. Then check to see if they are sorted correctly
    let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"];
    // Reset the database - update the coffee name to be the same for all coffees with the name
    // from the alphabet
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    update_coffees(update_coffees_input: {{ query: {{ name: "{}" }}, values: {{ name: "deprecated" }} }}) {{
                        data {{
                            id
                        }}
                    }}
                }}
            "#,
            alphabet[i - 1]
        ));
        let response = execute(request, None).await;
        assert!(response.is_ok());
    }

    // Create a new coffee for each letter in the alphabet.
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_coffee(create_coffee_input: {{ values: {{ name: "{}", price: {}, available: true, created_by: "6510865e93142f6d61b10dd8" }} }}) {{
                        data {{
                            id
                        }}
                    }}
                }}
            "#,
            alphabet[i - 1],
            i
        ));
        let response = execute(request, None).await;
        assert!(response.is_ok());
    }

    // Create a new coffee for each letter of the alphabet.
    // This time, change the price
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_coffee(create_coffee_input: {{ values: {{ name: "{}", price: {}, available: true, created_by: "6510865e93142f6d61b10dd8" }} }}) {{
                        data {{
                            id
                        }}
                    }}
                }}
            "#,
            alphabet[i - 1],
            i + 2
        ));
        let response = execute(request, None).await;
        assert!(response.is_ok());
    }

    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { query: { available: true }, opts: { sort: [{ field: "price", direction: "ASC" }, { field: "name", direction: "DESC" }] } }) {
                data {
                    id
                    name
                    price
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let coffees = json
        .get("get_coffees")
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap();

    // Check to see if the coffees are sorted correctly
    // First by price, then by name
    for i in 0..=8 {
        let coffee = coffees.get(i).unwrap();
        let next_coffee = coffees.get(i + 1).unwrap();
        let coffee_price = coffee.get("price").unwrap().as_i64().unwrap();
        let next_coffee_price = next_coffee.get("price").unwrap().as_i64().unwrap();
        let coffee_name = coffee.get("name").unwrap().as_str().unwrap();
        let next_coffee_name = next_coffee.get("name").unwrap().as_str().unwrap();

        // Check to see if the price is sorted correctly
        assert!(coffee_price <= next_coffee_price);

        // If the price is the same, check to see if the name is sorted correctly
        if coffee_price == next_coffee_price {
            assert!(coffee_name >= next_coffee_name);
        }
    }
}

#[tokio::test]
async fn find_many_with_like_filter() {
    // Create several coffees that have similar names.
    let uuid_name = format!("with_like_filter_{}", uuid::Uuid::new_v4());
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_coffee(create_coffee_input: {{ values: {{ name: "{}", price: {}, available: true, created_by: "6510865e93142f6d61b10dd8" }} }}) {{
                        data {{
                            id
                        }}
                    }}
                }}
            "#,
            format!("{}_{}", uuid_name, i),
            i
        ));
        let response = execute(request, None).await;
        assert!(response.is_ok());
    }

    let request = async_graphql::Request::new(format!(
        r#"
        query {{
            get_coffees(get_coffees_input: {{ query: {{ LIKE: {{ name: "{}%" }} }} }}) {{
                data {{
                    id
                    name
                }}
            }}
        }}
        "#,
        uuid_name
    ));

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let coffees = json
        .get("get_coffees")
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap();

    // Make sure there are at least 9 coffees returned
    assert_eq!(coffees.len(), 9);
}

#[tokio::test]
async fn find_many_with_lt_gt_filters() {
    // Create several coffees that that a price that is less than 50 but greater than 40
    let prices = vec![41, 42, 43, 44, 45, 46, 47, 48, 49];
    for price in prices {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_coffee(create_coffee_input: {{ values: {{ name: "lt_gt_filter", price: {}, available: true, created_by: "6510865e93142f6d61b10dd8" }} }}) {{
                        data {{
                            id
                        }}
                    }}
                }}
            "#,
            price
        ));
        let response = execute(request, None).await;
        assert!(response.is_ok());
    }

    let request = async_graphql::Request::new(
        r#"
        query {
            get_coffees(get_coffees_input: { query: { AND: [{ GT: { price: 40 } }, { LT: { price: 50 } }] } }) {
                data {
                    id
                    name
                    price
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let coffees = json
        .get("get_coffees")
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap();

    // Make sure there are at least 9 coffees returned
    assert!(coffees.len() >= 9);

    // Make sure all the coffees returned have prices between 40 and 50
    for coffee in coffees {
        let price = coffee.get("price").unwrap().as_i64().unwrap();
        assert!(price > 40 && price < 50);
    }
}
