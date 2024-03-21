use crate::execute;

#[tokio::test]
async fn update_many() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_cars(update_cars_input: { values: { model: "Toyota" }, query: { price: 10100 } }) {
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
    let uuid = format!("{}_test", uuid::Uuid::new_v4().to_string());
    let mut ids = vec![];
    for _ in 0..4 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_car(create_car_input: {{ values: {{ model: "{}", price: 30100, status: true }} }}) {{
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
        let data = data.get("create_car").unwrap();
        let data = data.get("data").unwrap();
        let data = data.get("id").unwrap();
        let id = data.as_i64().unwrap();
        ids.push(id);
    }

    let request = async_graphql::Request::new(format!(
        r#"
            mutation {{
                update_cars(update_cars_input: {{ values: {{ model: "Toyota_Uniue_Update_test" }}, query: {{ OR: [ {{ model: "{}" }}, {{ model: "idontexist" }}] }} }}) {{
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
    let data = data.get("update_cars").unwrap();
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
