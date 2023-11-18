use crate::execute;

#[tokio::test]
async fn guard_success() {
    let request = async_graphql::Request::new(
        r#"
        {
            get_user(get_user_input: { name: "Bongo", age: 10, married: false, email: "nickisyourfan@gmail.com" }) {
                _id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(!response.is_ok());
}
