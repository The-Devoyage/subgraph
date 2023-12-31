use crate::execute;

#[tokio::test]
async fn create_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_car(create_car_input: { values: { model: "Subaru", price: 10100 status: true } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn create_one_with_default_value() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_car_purchase(create_car_purchase_input: { values: { price: 101010, buyer: "iggy", status: "pending" } }) {
                id
                car_id(car_id: { query: {} }) {
                  id
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
    let data = response.data.into_json().unwrap();
    let car_id = data["create_car_purchase"]["car_id"]["id"]
        .as_i64()
        .unwrap();
    assert_eq!(car_id, 1);
}
