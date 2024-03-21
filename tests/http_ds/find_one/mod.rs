use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        {
            get_todo(get_todo_input: { query: { id: 1 } }) {
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

//FIX: if missing required path param, it should return error.
//TODO: Current Test API does not support find one by string.
//
// #[tokio::test]
// async fn find_one_by_string() {
//     let request = async_graphql::Request::new(
//         r#"
//         {
//             get_todo(get_todo_input: { id: 1 }) {
//                 data {
//                      id
//                 }
//             }
//         }
//         "#,
//     );

//     let response = execute(request, None).await;
//     assert!(response.is_ok());
// }

#[tokio::test]
async fn find_one_by_int() {
    let request = async_graphql::Request::new(
        r#"
        {
            get_todo(get_todo_input: { query: { id: 1 } }) {
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
