use crate::execute;

#[tokio::test]
async fn create_one() {
    let request = async_graphql::Request::new(
        r#"
        mutation {
            create_comment(create_comment_input: { values: { content: "create_one test", status: true } }) {
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
async fn create_one_with_default_value() {
    let uuid = uuid::Uuid::new_v4().to_string();
    let datetime = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
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
        uuid, datetime
    ));

    let response = execute(request, None).await;
    assert!(response.is_ok());
    let data = response.data.into_json().unwrap();
    let user_id = data["create_reaction"]["data"]["content"].as_str().unwrap();
    assert_eq!(user_id, "like");
}
