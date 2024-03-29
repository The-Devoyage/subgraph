use crate::execute;

#[tokio::test]
async fn create_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_coffee(create_coffee_input: { values: { name: "Colombe", price: 12, available: true, created_by: "6510865e93142f6d61b10dd8" } }) {
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
async fn create_one_with_default_value() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_coffee_order(create_coffee_order_input: { values: {  created_by: "6510865e93142f6d61b10dd8", uuid: "af2e25cf-14bc-4e42-9ff1-93a6d3e222af" } }) {
                data {
                    id
                    status
                }
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
    let data = response.data.into_json().unwrap();
    let status = data["create_coffee_order"]["data"]["status"]
        .as_str()
        .unwrap();
    assert_eq!(status, "pendingg");
}
