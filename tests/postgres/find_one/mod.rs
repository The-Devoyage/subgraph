use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { id: 1 }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn find_one_fails() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { id: 1922929 }) {
                id
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_err());
}

#[tokio::test]
async fn find_one_by_string() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { content: "findOneByString", status: true }) {
                id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { content: "findOneByString" }) {
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
            create_comment(create_comment_input: { content: "findOneByInt", status: true }) {
                id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { id: 2 }) {
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
            create_comment(create_comment_input: { content: "findOneByBool", status: true }) {
                id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { status: true }) {
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
            create_comment(create_comment_input: { content: "returnsCorrectScalars", status: true }) {
                id
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { content: "returnsCorrectScalars" }) {
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