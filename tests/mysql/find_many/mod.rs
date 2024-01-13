use crate::execute;

#[tokio::test]
async fn find_many() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_cars(get_cars_input: { query: { id: 1 } }) {
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
async fn find_many_primitive_scalars() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_cars(get_cars_input: { query: { id: 1, model: "BMW",  price: 10000 } }) {
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
            get_cars(get_cars_input: { query: { OR: [{ id: 1 }, { id: 2 }] } }) {
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
async fn find_many_with_and_filter() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_cars(get_cars_input: { query: { AND: [{ id: 1 }, { model: "BMW" }] } }) {
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
                get_car_purchases(get_car_purchases_input: { query: { car_id: { model: "BMW" } } }) {
                    data {
                        id
                        buyer
                    }
                }
            }
        "#,
    );
    let response = execute(request, None).await;
    let data = response.data.into_json().unwrap();
    assert_eq!(data["get_car_purchases"]["data"][0]["buyer"], "John Doe");
}

#[tokio::test]
async fn find_many_with_eager_loading_1() {
    let request = async_graphql::Request::new(
        r#"
            query {
                get_cars(get_cars_input: { query: { purchases: { buyer: "John Doe" } } }) {
                    data {
                        id
                    }
                }
            }
        "#,
    );
    let response = execute(request, None).await;
    let data = response.data.into_json().unwrap();
    assert_eq!(data["get_cars"]["data"][0]["id"], 1);
}

#[tokio::test]
async fn find_many_with_pagination() {
    for _i in 1..=10 {
        let request = async_graphql::Request::new(
            r#"
                mutation {
                    create_car(create_car_input: { values: { model: "Paginate33", price: 120100, status: true } }) {
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
            get_cars(get_cars_input: { query: { model: "Paginate33" },  opts: { page: 1, per_page: 3 } }) {
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
    let meta = json.get("get_cars").unwrap().get("meta").unwrap();
    assert_eq!(meta.get("count").unwrap().as_i64().unwrap(), 3);
    assert!(meta.get("total_count").unwrap().as_i64().unwrap() > 9);
    assert_eq!(meta.get("page").unwrap().as_i64().unwrap(), 1);

    let request = async_graphql::Request::new(
        r#"
        query {
            get_cars(get_cars_input: { query: { model: "Paginate33" },  opts: { page: 2, per_page: 3 } }) {
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
    let meta = json.get("get_cars").unwrap().get("meta").unwrap();
    assert_eq!(meta.get("count").unwrap().as_i64().unwrap(), 3);
    assert!(meta.get("total_count").unwrap().as_i64().unwrap() > 9);
    assert_eq!(meta.get("page").unwrap().as_i64().unwrap(), 2);
}

#[tokio::test]
async fn find_many_with_sorting_and_pagination() {
    // Create several car that can be sorted first by price and second by name. Then check to see if they are sorted correctly
    let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"];
    // Reset the database - update the car name to be the same for all car with the name
    // from the alphabet
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    update_cars(update_cars_input: {{ query: {{ model: "{}" }}, values: {{ model: "deprecated" }} }}) {{
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

    // Create a new car for each letter in the alphabet.
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_car(create_car_input: {{ values: {{ model: "{}", price: {}, status: true }} }}) {{
                        data {{
                            id
                        }}
                    }}
                }}
            "#,
            alphabet[i - 1],
            i * 100
        ));
        let response = execute(request, None).await;
        assert!(response.is_ok());
    }

    // Create a new car for each letter of the alphabet.
    // This time, change the price
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_car(create_car_input: {{ values: {{ model: "{}", price: {}, status: true }} }}) {{
                        data {{
                            id
                        }}
                    }}
                }}
            "#,
            alphabet[i - 1],
            i * 200
        ));
        let response = execute(request, None).await;
        assert!(response.is_ok());
    }

    let request = async_graphql::Request::new(
        r#"
        query {
            get_cars(get_cars_input: { query: { 
                OR: [
                    { model: "A" },
                    { model: "B" },
                    { model: "C" },
                    { model: "D" },
                    { model: "E" },
                    { model: "F" },
                    { model: "G" },
                    { model: "H" },
                    { model: "I" }
                ]
            }, opts: { sort: [{ field: "model", direction: "ASC" }, { field: "price", direction: "DESC"}] } }) {
                data {
                    id
                    model
                    price
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let data = json.get("get_cars").unwrap().get("data").unwrap();

    // Check to see if the car are sorted correctly
    // First by model and then by price
    for i in 0..=8 {
        let car = data.get(i).unwrap();
        let next_car = data.get(i + 1).unwrap();
        let car_model = car.get("model").unwrap().as_str().unwrap();
        let next_car_model = next_car.get("model").unwrap().as_str().unwrap();
        let car_price = car.get("price").unwrap().as_i64().unwrap();
        let next_car_price = next_car.get("price").unwrap().as_i64().unwrap();

        // Check to see if the car are sorted by model
        assert!(car_model <= next_car_model);

        // Check to see if the car are sorted by price
        if car_model == next_car_model {
            assert!(car_price >= next_car_price);
        }
    }
}
