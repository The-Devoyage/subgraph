use crate::execute;

#[tokio::test]
async fn find_many() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comments(get_comments_input: { query: { id: 1 } }) {
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
            get_comments(get_comments_input: { query: { AND: [{ id: 1 }, { text: "This is content test." }] } }) {
                data {
                    id
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;
    assert!(response.is_err());
}

#[tokio::test]
async fn find_many_with_or_filter() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comments(get_comments_input: { query: { OR: [{ id: 1 }, { id: 2 }] } }) {
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
            get_comments(get_comments_input: { query: { reactions: { content: "This is content test." } } }) {
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
async fn find_many_with_pagination() {
    for _i in 1..=10 {
        let request = async_graphql::Request::new(
            r#"
                mutation {
                    create_comment(create_comment_input: { values: { content: "Pagination Test", status: true } }) {
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
            get_comments(get_comments_input: { query: { content: "Pagination Test" }, opts: { page: 1, per_page: 3 } }) {
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
    let meta = json.get("get_comments").unwrap().get("meta").unwrap();
    assert_eq!(meta.get("count").unwrap().as_i64().unwrap(), 3);
    assert!(meta.get("total_count").unwrap().as_i64().unwrap() > 9);
    assert_eq!(meta.get("page").unwrap().as_i64().unwrap(), 1);

    let request = async_graphql::Request::new(
        r#"
        query {
            get_comments(get_comments_input: { query: { content: "Pagination Test" }, opts: { page: 2, per_page: 3 } }) {
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
    let meta = json.get("get_comments").unwrap().get("meta").unwrap();
    assert_eq!(meta.get("count").unwrap().as_i64().unwrap(), 3);
    assert!(meta.get("total_count").unwrap().as_i64().unwrap() > 9);
    assert_eq!(meta.get("page").unwrap().as_i64().unwrap(), 2);
}
