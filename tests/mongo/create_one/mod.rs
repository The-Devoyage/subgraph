use crate::execute;

#[tokio::test]
async fn create_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_user(create_user_input: { name: "Nick", age: 30, married: true, email: "nickisyourfan@gmail.com" }) {
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
            }) {
                _id
            }
        }
        "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
}
