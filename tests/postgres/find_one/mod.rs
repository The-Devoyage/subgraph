use crate::execute;

#[tokio::test]
async fn find_one() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { id: 1 } }) {
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
async fn find_one_by_string() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { values: { content: "findOneByString", status: true } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { content: "findOneByString" } }) {
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
async fn find_one_by_int() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { values: { content: "findOneByInt", status: true } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { id: 2 } }) {
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
async fn find_one_by_bool() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { values: { content: "findOneByBool", status: true } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { status: true } }) {
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
async fn returns_correct_scalars() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { values: { content: "returnsCorrectScalars", status: true } }) {
                data {
                    id
                }
            }
        }
        "#,
    );
    execute(request, None).await;
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { content: "returnsCorrectScalars" } }) {
                data {
                    id
                    content
                    status
                }
            }
        }
        "#,
    );

    let response = execute(request, None).await;

    let json = response.data.into_json().unwrap();

    let comment = json.get("get_comment").unwrap();
    assert_eq!(
        comment["data"].get("content").unwrap().as_str().unwrap(),
        "returnsCorrectScalars"
    );
    assert_eq!(
        comment["data"].get("status").unwrap().as_bool().unwrap(),
        true
    );
}

#[tokio::test]
async fn returns_correct_scalars_uuid_datetime() {
    let uuid = uuid::Uuid::new_v4().to_string();
    let datetime = chrono::Utc::now();
    let request = async_graphql::Request::new(format!(
        r#"
            mutation {{
                create_reaction(create_reaction_input: {{ values: {{ status: true, uuid: "{}", reaction_date: "{}" }} }}) {{
                    data {{
                        id
                        content
                    }}
                }}
            }}
            "#,
        uuid,
        datetime.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    ));

    execute(request, None).await;

    let request = async_graphql::Request::new(format!(
        r#"
            query {{
                get_reaction(get_reaction_input: {{ query: {{ uuid: "{}" }} }}) {{
                    data {{
                        id
                        content
                        status
                        uuid
                        reaction_date
                    }}
                }}
            }}
            "#,
        uuid
    ));

    let response = execute(request, None).await;

    let json = response.data.into_json().unwrap();
    let reaction = json.get("get_reaction").unwrap();
    let reaction_date_str = reaction["data"]
        .get("reaction_date")
        .unwrap()
        .as_str()
        .unwrap();
    assert_eq!(
        reaction["data"].get("uuid").unwrap().as_str().unwrap(),
        uuid
    );
    assert_eq!(
        reaction_date_str,
        datetime.to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
    );
}

#[tokio::test]
async fn find_one_with_or_filter() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { OR: [{ id: 1 }, { id: 2 }] } }) {
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
async fn find_one_with_and_filter() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { AND: [{ id: 1 }, { content: "This is content test." }] } }) {
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
async fn find_one_with_virtual_field() {
    let request = async_graphql::Request::new(
        r#"
        query {
            get_comment(get_comment_input: { query: { id: 1, virtual_id: "im virtual" } }) {
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
