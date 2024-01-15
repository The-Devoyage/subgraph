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

#[tokio::test]
async fn update_many_returns_correct_results() {
    // Create 4 new coffees that have a shared property and 2 new coffees that dont share that propety.
    // Upate the 4 based on a common property. The result should be 4 updated coffees - not the 2 that dont share the property.
    let uuid = uuid::Uuid::new_v4().to_string();
    let mut ids = vec![];
    for _ in 0..4 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_coffee(create_coffee_input: {{ values: {{ name: "{}", price: 7, available: true, created_by: "6510865e93142f6d61b10dd8" }} }}) {{
                        data {{
                            id
                        }}
                    }}
                }}
                "#,
            uuid
        ));
        let response = execute(request, None).await;
        let data = response.data.into_json().unwrap();
        let data = data.get("create_coffee").unwrap();
        let data = data.get("data").unwrap();
        let data = data.get("id").unwrap();
        let id = data.as_i64().unwrap();
        ids.push(id);
    }

    let request = async_graphql::Request::new(format!(
        r#"
            mutation {{
                update_coffees(update_coffees_input: {{ values: {{ price: 8 }}, query: {{ OR: [{{ name: "{}"}}, {{ name: "idontexist" }}] }} }}) {{
                    data {{
                        id
                    }}
                }}
            }}
            "#,
        uuid
    ));

    let response = execute(request, None).await;
    assert!(response.is_ok());
    let data = response.data.into_json().unwrap();
    let data = data.get("update_coffees").unwrap();
    let data = data.get("data").unwrap();
    let data = data.as_array().unwrap();
    assert_eq!(data.len(), 4);

    for id in ids {
        let mut found = false;
        for item in data {
            let item = item.as_object().unwrap();
            let item_id = item.get("id").unwrap().as_i64().unwrap();
            if id == item_id {
                found = true;
                break;
            }
        }
        assert!(found);
    }
}

#[tokio::test]
async fn primary_key_fails() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_coffee_orders(update_coffee_orders_input: { values: { status: "primary_keys_fail_test" }, query: { coffee_id: 1 } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_err());
}
