use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "Oakley", age: 5, married: false, email: "nickisyourfan@gmail.com" } }) {
                data {
                    _id
                }
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        {
            get_users(get_users_input: { query: { married: false } }) {
                data {
                    _id
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
        mutation {
            create_user(create_user_input: { values: { name: "BongoWithOrFilter", age: 203, married: false, email: "nickisyourfan@gmail.com" } }) {
                data {
                    _id
                }
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "BongoWithOrFilter", age: 204, married: false, email: "nickisyourfan@gmail.com" } }) {
                data {
                    _id
                }
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        {
            get_users(get_users_input: { query: { OR: [{ age: 203 }, { age: 204 }] } }) {
                data {
                    _id
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
        mutation {
            create_user(create_user_input: { values: { name: "FindManyWithAndFilter", age: 988, married: false, email: "nickisyourfan@gmail.com" } }) {
                data {
                    _id
                }
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "FindManyWithAndFilter2", age: 988, married: false, email: "nickisyourfan@gmail.com" } }) {
                data {
                    _id
                }
            }
        }
        "#,
    );
    execute(request, None).await;

    let request = async_graphql::Request::new(
        r#"
        {
            get_user(get_user_input: { query: { AND: [{ age: 988 }, { married: false }] } }) {
                data {
                    _id
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
        mutation {
            create_user(create_user_input: { values: { name: "FindManyWithEagerLoading", age: 57, married: true, email: "abcdefg@fakemail.com" } }) {
                data {
                    _id
                }
            }   
        }
        "#,
    );
    let response = execute(request, None).await;
    let data = response.data.into_json().unwrap();
    let user_id = data.get("create_user").unwrap()["data"]
        .as_object()
        .unwrap()
        .get("_id")
        .unwrap()
        .as_str()
        .unwrap();

    let request = async_graphql::Request::new(format!(
        r#"
            mutation {{
                create_user_access(create_user_access_input: {{ values: {{ user_id: "{}", view: true }} }}) {{
                    data {{
                        _id
                    }}
                }}   
            }}
            "#,
        user_id
    ));
    execute(request, None).await;

    let request = async_graphql::Request::new(format!(
        r#"
            {{
                get_users(get_users_input: {{ query: {{ user_access: {{ user_id: "{}", view: true }} }} }}) {{
                    data {{
                        _id
                        name
                    }}
                }}
            }}
            "#,
        user_id
    ));

    let response = execute(request, None).await;

    let data = response.data.into_json().unwrap();
    let users = data.get("get_users").unwrap()["data"].as_array().unwrap();
    let user = users.get(0).unwrap().as_object().unwrap();
    let name = user.get("name").unwrap().as_str().unwrap();

    assert_eq!(name, "FindManyWithEagerLoading");
}

// #[tokio::test]
// async fn find_many_with_pagination() {
//     // Create 10 coffees with the same name
//     for _i in 1..=10 {
//         let request = async_graphql::Request::new(
//             r#"
//                 mutation {
//                     create_coffee(create_coffee_input: { values: { name: "Pagination Test", price: 12, available: true, created_by: "6510865e93142f6d61b10dd8" } }) {
//                         data {
//                             id
//                         }
//                     }
//                 }
//             "#,
//         );

//         let response = execute(request, None).await;
//         assert!(response.is_ok());
//     }

//     let request = async_graphql::Request::new(
//         r#"
//         query {
//             get_coffees(get_coffees_input: { query: { name: "Pagination Test" }, opts: { page: 1, per_page: 3 } }) {
//                 data {
//                     id
//                 }
//                 meta {
//                     count
//                     total_count
//                     page
//                     total_pages
//                 }
//             }
//         }
//         "#,
//     );

//     let response = execute(request, None).await;
//     let json = response.data.into_json().unwrap();
//     let meta = json.get("get_coffees").unwrap().get("meta").unwrap();
//     assert_eq!(meta.get("count").unwrap().as_i64().unwrap(), 3);
//     assert!(meta.get("total_count").unwrap().as_i64().unwrap() > 9);
//     assert_eq!(meta.get("page").unwrap().as_i64().unwrap(), 1);

//     let request = async_graphql::Request::new(
//         r#"
//         query {
//             get_coffees(get_coffees_input: { query: { name: "Pagination Test" }, opts: { page: 2, per_page: 3 } }) {
//                 data {
//                     id
//                 }
//                 meta {
//                     count
//                     total_count
//                     page
//                     total_pages
//                 }
//             }
//         }
//         "#,
//     );

//     let response = execute(request, None).await;
//     let json = response.data.into_json().unwrap();
//     let meta = json.get("get_coffees").unwrap().get("meta").unwrap();
//     assert_eq!(meta.get("count").unwrap().as_i64().unwrap(), 3);
//     assert!(meta.get("total_count").unwrap().as_i64().unwrap() > 9);
//     assert_eq!(meta.get("page").unwrap().as_i64().unwrap(), 2);
// }

#[tokio::test]
async fn find_many_with_pagination() {
    // Create 10 Dogs with the same name
    for _i in 1..=10 {
        let request = async_graphql::Request::new(
            r#"
                mutation {
                    create_dog(create_dog_input: { values: { name: "Pagination Test", age: 12  } }) {
                        data {
                            _id
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
            get_dogs(get_dogs_input: { query: { name: "Pagination Test" }, opts: { page: 1, per_page: 3 } }) {
                data {
                    _id
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
    let meta = json.get("get_dogs").unwrap().get("meta").unwrap();
    assert_eq!(meta.get("count").unwrap().as_i64().unwrap(), 3);
    assert!(meta.get("total_count").unwrap().as_i64().unwrap() > 9);
    assert_eq!(meta.get("page").unwrap().as_i64().unwrap(), 1);

    let request = async_graphql::Request::new(
        r#"
        query {
            get_dogs(get_dogs_input: { query: { name: "Pagination Test" }, opts: { page: 2, per_page: 3 } }) {
                data {
                    _id
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
    let meta = json.get("get_dogs").unwrap().get("meta").unwrap();
    assert_eq!(meta.get("count").unwrap().as_i64().unwrap(), 3);
    assert!(meta.get("total_count").unwrap().as_i64().unwrap() > 9);
    assert_eq!(meta.get("page").unwrap().as_i64().unwrap(), 2);
}

// #[tokio::test]
// async fn find_many_with_sorting() {
//     // Create several coffees that can be sorted first by price and second by name. Then check to see if they are sorted correctly
//     let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"];
//     // Reset the database - update the coffee name to be the same for all coffees with the name
//     // from the alphabet
//     for i in 1..=9 {
//         let request = async_graphql::Request::new(format!(
//             r#"
//                 mutation {{
//                     update_coffees(update_coffees_input: {{ query: {{ name: "{}" }}, values: {{ name: "deprecated" }} }}) {{
//                         data {{
//                             id
//                         }}
//                     }}
//                 }}
//             "#,
//             alphabet[i - 1]
//         ));
//         let response = execute(request, None).await;
//         assert!(response.is_ok());
//     }

//     // Create a new coffee for each letter in the alphabet.
//     for i in 1..=9 {
//         let request = async_graphql::Request::new(format!(
//             r#"
//                 mutation {{
//                     create_coffee(create_coffee_input: {{ values: {{ name: "{}", price: {}, available: true, created_by: "6510865e93142f6d61b10dd8" }} }}) {{
//                         data {{
//                             id
//                         }}
//                     }}
//                 }}
//             "#,
//             alphabet[i - 1],
//             i
//         ));
//         let response = execute(request, None).await;
//         assert!(response.is_ok());
//     }

//     // Create a new coffee for each letter of the alphabet.
//     // This time, change the price
//     for i in 1..=9 {
//         let request = async_graphql::Request::new(format!(
//             r#"
//                 mutation {{
//                     create_coffee(create_coffee_input: {{ values: {{ name: "{}", price: {}, available: true, created_by: "6510865e93142f6d61b10dd8" }} }}) {{
//                         data {{
//                             id
//                         }}
//                     }}
//                 }}
//             "#,
//             alphabet[i - 1],
//             i + 2
//         ));
//         let response = execute(request, None).await;
//         assert!(response.is_ok());
//     }

//     let request = async_graphql::Request::new(
//         r#"
//         query {
//             get_coffees(get_coffees_input: { query: { available: true }, opts: { sort: [{ field: "price", direction: "ASC" }, { field: "name", direction: "DESC" }] } }) {
//                 data {
//                     id
//                     name
//                     price
//                 }
//             }
//         }
//         "#,
//     );

//     let response = execute(request, None).await;
//     let json = response.data.into_json().unwrap();
//     let coffees = json
//         .get("get_coffees")
//         .unwrap()
//         .get("data")
//         .unwrap()
//         .as_array()
//         .unwrap();

//     // Check to see if the coffees are sorted correctly
//     // First by price, then by name
//     for i in 0..=8 {
//         let coffee = coffees.get(i).unwrap();
//         let next_coffee = coffees.get(i + 1).unwrap();
//         let coffee_price = coffee.get("price").unwrap().as_i64().unwrap();
//         let next_coffee_price = next_coffee.get("price").unwrap().as_i64().unwrap();
//         let coffee_name = coffee.get("name").unwrap().as_str().unwrap();
//         let next_coffee_name = next_coffee.get("name").unwrap().as_str().unwrap();

//         // Check to see if the price is sorted correctly
//         assert!(coffee_price <= next_coffee_price);

//         // If the price is the same, check to see if the name is sorted correctly
//         if coffee_price == next_coffee_price {
//             assert!(coffee_name >= next_coffee_name);
//         }
//     }
// }

#[tokio::test]
async fn find_many_with_sorting() {
    let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"];
    // Reset the database - update the dog name to be the same for all dogs with the name
    // from the alphabet
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    update_dogs(update_dogs_input: {{ query: {{ name: "{}" }}, values: {{ name: "deprecated" }} }}) {{
                        data {{
                            _id
                        }}
                    }}
                }}
            "#,
            alphabet[i - 1]
        ));
        let response = execute(request, None).await;
        assert!(response.is_ok());
    }

    // Create a new dog for each letter in the alphabet.
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_dog(create_dog_input: {{ values: {{ name: "{}", age: {} }} }}) {{
                        data {{
                            _id
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

    // Create a new dog for each letter of the alphabet.
    // This time, change the age
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_dog(create_dog_input: {{ values: {{ name: "{}", age: {} }} }}) {{
                        data {{
                            _id
                            age
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
            get_dogs(get_dogs_input: { query: {
                OR: [
                  { name: "A" },
                  { name: "B" },
                  { name: "C" },
                  { name: "D" },
                  { name: "E" },
                  { name: "F" },
                  { name: "G" },
                  { name: "H" },
                  { name: "I" },
                ]
            }, opts: { sort: [{ field: "age", direction: "ASC" }, { field: "name", direction: "DESC" }] } }) {
                data {
                    _id
                    name
                    age
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let dogs = json
        .get("get_dogs")
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap();

    // Check to see if the dogs are sorted correctly
    // First by age, then by name
    for i in 0..=8 {
        let dog = dogs.get(i).unwrap();
        let next_dog = dogs.get(i + 1).unwrap();
        let dog_age = dog.get("age").unwrap().as_i64().unwrap();
        let next_dog_age = next_dog.get("age").unwrap().as_i64().unwrap();
        let dog_name = dog.get("name").unwrap().as_str().unwrap();
        let next_dog_name = next_dog.get("name").unwrap().as_str().unwrap();

        // Check to see if the age is sorted correctly
        assert!(dog_age <= next_dog_age);

        // If the age is the same, check to see if the name is sorted correctly
        if dog_age == next_dog_age {
            assert!(dog_name >= next_dog_name);
        }
    }
}

#[tokio::test]
async fn find_many_with_like_filter() {
    // create a dog with a unique name, and a bunch of dogs with similar names
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_dog(create_dog_input: {{ values: {{ name: "{}", age: {} }} }}) {{
                        data {{
                            _id
                        }}
                    }}
                }}
            "#,
            format!("{}_{}", uuid::Uuid::new_v4(), "with_like_filter"),
            i
        ));
        let response = execute(request, None).await;
        assert!(response.is_ok());
    }

    let request = async_graphql::Request::new(
        r#"
        query {
            get_dogs(get_dogs_input: { query: { LIKE: { name: "/with_like_filter/i" } } }) {
                data {
                    _id
                    name
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let dogs = json
        .get("get_dogs")
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap();

    // Make sure there are at least 9 dogs returned
    assert!(dogs.len() >= 9);

    // Make sure all the dogs have a similar name
    for dog in dogs {
        let dog_name = dog.get("name").unwrap().as_str().unwrap();
        assert!(dog_name.contains("with_like_filter"));
    }
}
// #[tokio::test]
// async fn find_many_with_lt_gt_filters() {
//     // Create several coffees that that a price that is less than 50 but greater than 40
//     let prices = vec![41, 42, 43, 44, 45, 46, 47, 48, 49];
//     for price in prices {
//         let request = async_graphql::Request::new(format!(
//             r#"
//                 mutation {{
//                     create_coffee(create_coffee_input: {{ values: {{ name: "lt_gt_filter", price: {}, available: true, created_by: "6510865e93142f6d61b10dd8" }} }}) {{
//                         data {{
//                             id
//                         }}
//                     }}
//                 }}
//             "#,
//             price
//         ));
//         let response = execute(request, None).await;
//         assert!(response.is_ok());
//     }

//     let request = async_graphql::Request::new(
//         r#"
//         query {
//             get_coffees(get_coffees_input: { query: { AND: [{ GT: { price: 40 } }, { LT: { price: 50 } }] } }) {
//                 data {
//                     id
//                     name
//                     price
//                 }
//             }
//         }
//         "#,
//     );

//     let response = execute(request, None).await;
//     let json = response.data.into_json().unwrap();
//     let coffees = json
//         .get("get_coffees")
//         .unwrap()
//         .get("data")
//         .unwrap()
//         .as_array()
//         .unwrap();

//     // Make sure there are at least 9 coffees returned
//     assert!(coffees.len() >= 9);

//     // Make sure all the coffees returned have prices between 40 and 50
//     for coffee in coffees {
//         let price = coffee.get("price").unwrap().as_i64().unwrap();
//         assert!(price > 40 && price < 50);
//     }
// }

#[tokio::test]
async fn find_many_with_lt_gt_filters() {
    // Create several dogs with ages from 30-40
    let ages = vec![30, 31, 32, 33, 34, 35, 36, 37, 38, 39];
    for age in ages {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_dog(create_dog_input: {{ values: {{ name: "lt_gt_filter", age: {} }} }}) {{
                        data {{
                            _id
                        }}
                    }}
                }}
            "#,
            age
        ));
        let response = execute(request, None).await;
        assert!(response.is_ok());
    }

    let request = async_graphql::Request::new(
        r#"
        query {
            get_dogs(get_dogs_input: { query: { AND: [{ GT: { age: 30 } }, { LT: { age: 40 } }] } }) {
                data {
                    _id
                    name
                    age
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let dogs = json
        .get("get_dogs")
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap();

    // Make sure there are at least 9 dogs returned
    assert!(dogs.len() >= 9);

    // Make sure all the dogs returned have ages between 30 and 40
    for dog in dogs {
        let age = dog.get("age").unwrap().as_i64().unwrap();
        assert!(age > 30 && age < 40);
    }
}
