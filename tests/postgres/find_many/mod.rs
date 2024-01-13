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
#[tokio::test]
async fn find_many_with_sorting_and_pagination() {
    // Create several comments that can be first sorted by content and second by id. Then check to
    // see if they are sorted correctly.
    let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"];
    // Reset the database - update the comment content to be the same for all comments with the
    // content from the alphabet.
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    update_comments(update_comments_input: {{ query: {{ content: "{}" }}, values: {{ content: "deprecated" }} }}) {{
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

    // Create several comments that can be first sorted by content and second by id.
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_comment(create_comment_input: {{ values: {{ content: "{}", status: true }} }}) {{
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

    // Create them again so they have different ids with the same content.
    for i in 1..=9 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_comment(create_comment_input: {{ values: {{ content: "{}", status: true }} }}) {{
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

    // Sort by content ascending and id ascending.
    let request = async_graphql::Request::new(
        r#"
            query {
              get_comments(
                get_comments_input: {
                  query: {
                    OR: [
                      { content: "A" },
                      { content: "B" },
                      { content: "C" },
                      { content: "D" },
                      { content: "E" },
                      { content: "F" },
                      { content: "G" },
                      { content: "H" },
                      { content: "I" },
                    ]
                  }
                  opts: {
                    sort: [
                      { field: "comments.content", direction: "ASC" },
                      { field: "comments.id", direction: "DESC" }
                    ]
                  }
                }
              ) {
                data {
                  id
                  content
                }
              }
            }
        "#,
    );

    let response = execute(request, None).await;
    let json = response.data.into_json().unwrap();
    let comments = json
        .get("get_comments")
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap();

    // Check to see if the comments are sorted correctly.
    // The comments should be sorted by content ascending and id descending.
    for i in 0..=8 {
        let content = comments[i].get("content").unwrap().as_str().unwrap();
        let next_content = comments[i + 1].get("content").unwrap().as_str().unwrap();
        assert!(content <= next_content);

        if content == next_content {
            let id = comments[i].get("id").unwrap().as_i64().unwrap();
            let next_id = comments[i + 1].get("id").unwrap().as_i64().unwrap();
            assert!(id >= next_id);
        }
    }
}
