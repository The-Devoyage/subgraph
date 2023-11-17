use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { id: 1 } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn find_one_by_string() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { values: { content: "findOneByString", status: true } }) {
                id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { content: "findOneByString" } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn find_one_by_int() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { values: { content: "findOneByInt", status: true } }) {
                id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { id: 2 } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn find_one_by_bool() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { values: { content: "findOneByBool", status: true } }) {
                id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { status: true } }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn returns_correct_scalars() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { values: { content: "returnsCorrectScalars", status: true } }) {
                id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { content: "returnsCorrectScalars" } }) {
                id
                content
                status
            }
        }
        "#,
    );

    let response = execute(request, None).await;

    let json = response.data.into_json().unwrap();

    let comment = json.get("get_comment").unwrap();
    assert_eq!(
        comment.get("content").unwrap().as_str().unwrap(),
        "returnsCorrectScalars"
    );
    assert_eq!(comment.get("status").unwrap().as_bool().unwrap(), true);
}
