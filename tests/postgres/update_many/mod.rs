use crate::execute;

#[tokio::test]
async fn update_many() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            update_comments(update_comments_input: { values: { content: "update_many test", status: false }, query: { id: 2 } }) {
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
async fn update_many_returns_correct_results() {
    let uuid = format!("test_{}", uuid::Uuid::new_v4().to_string());
    let mut ids = vec![];
    for _ in 0..4 {
        let request = async_graphql::Request::new(format!(
            r#"
                mutation {{
                    create_comment(create_comment_input: {{ values: {{ content: "{}", status: true }} }}) {{
                        data {{
                            id
                            content
                        }}
                    }}
                }}
                "#,
            uuid
        ));
        let response = execute(request, None).await;
        let data = response.data.into_json().unwrap();
        let data = data.get("create_comment").unwrap();
        let data = data.get("data").unwrap();
        let data = data.get("id").unwrap();
        let id = data.as_i64().unwrap();
        ids.push(id);
    }

    let request = async_graphql::Request::new(format!(
        r#"
            mutation {{
                update_comments(update_comments_input: {{ values: {{ content: "update_many test", status: false }}, query: {{ OR: [{{ content: "{}" }}, {{ content: "idontexist" }}] }} }}) {{
                    data {{
                        id
                    }}
                }}
            }}
            "#,
        uuid
    ));

    let response = execute(request, None).await;
    assert!(response.is_ok());
    let data = response.data.into_json().unwrap();
    let data = data.get("update_comments").unwrap();
    let data = data.get("data").unwrap();
    let data = data.as_array().unwrap();
    assert_eq!(data.len(), 4);
    for id in ids {
        let mut found = false;
        for item in data {
            let item = item.as_object().unwrap();
            let item_id = item.get("id").unwrap().as_i64().unwrap();
            if id == item_id {
                found = true;
                break;
            }
        }
        assert!(found);
    }
}
