use crate::execute;

#[tokio::test]
async fn find_many() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_cars(get_cars_input: { query: { id: 1 } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn find_many_primitive_scalars() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_cars(get_cars_input: { query: { id: 1, model: "BMW",  price: 10000 } }) {
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
            get_cars(get_cars_input: { query: { OR: [{ id: 1 }, { id: 2 }] } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn find_many_with_and_filter() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_cars(get_cars_input: { query: { AND: [{ id: 1 }, { model: "BMW" }] } }) {
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
                get_car_purchases(get_car_purchases_input: { query: { car_id: { model: "BMW" } } }) {
                    id
                    buyer
                }
            }
        "#,
    );
    let response = execute(request, None).await;
    let data = response.data.into_json().unwrap();
    assert_eq!(data["get_car_purchases"][0]["buyer"], "John Doe");
}

#[tokio::test]
async fn find_many_with_eager_loading_1() {
    let request = async_graphql::Request::new(
        r#"
            query {
                get_cars(get_cars_input: { query: { purchases: { buyer: "John Doe" } } }) {
                    id
                }
            }
        "#,
    );
    let response = execute(request, None).await;
    let data = response.data.into_json().unwrap();
    assert_eq!(data["get_cars"][0]["id"], 1);
}
