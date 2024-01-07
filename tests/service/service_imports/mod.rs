use crate::execute;

#[tokio::test]
async fn find_one_from_import() {
    let request = async_graphql::Request::new(
        r#"
        {
            get_import(get_import_input: { query: {} }) {
                data {
                    _id
                    import_works
                }
            }
        }
    "#,
    );
    let response = execute(request, None).await;
    assert!(response.is_ok());
}
