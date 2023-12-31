use crate::execute;

#[tokio::test]
async fn create_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { values: { name: "Nick", age: 30, married: true, email: "nickisyourfan@gmail.com" } }) {
                _id
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn create_one_nested_object() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { 
                values: {
                    name: "Rory", 
                    age: 22, 
                    married: false, 
                    email: "rory@rory.com",
                    address: {
                        line_one: "address lineone",
                        line_two: "address linetwo",
                        city: "address city",
                        state: "address state",
                        zip: "address zip"
                    }
                }
            }) {
                _id
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
            create_user(create_user_input: { values: { name: "Nick", age: 30, married: true, email: "nick@nick.com" } }) {
                _id
                middle_name
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
    let data = response.data.into_json().unwrap();
    let middle_name = data["create_user"]["middle_name"].as_str().unwrap();
    assert_eq!(middle_name, "jack");
}
