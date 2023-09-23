use crate::execute;
//TODO: Determine if possible to generate a window + credentials for testing.
// use web_sys::{window, CredentialCreationOptions};
// use webauthn_rs::prelude::CreationChallengeResponse;

// #[tokio::test]
// async fn register_start() {
//     let random_string = uuid::Uuid::new_v4().to_string();
//     let request = async_graphql::Request::new(format!(
//         r#"
//         mutation {{
//             register_start(identifier: "{}")
//         }}
//         "#,
//         random_string
//     ));
//     let response = execute(request, None).await;
//     assert!(response.is_ok());
// }

// #[tokio::test]
// async fn register_finish() {
//     let random_string = uuid::Uuid::new_v4().to_string();

//     //Create new account
//     let request = async_graphql::Request::new(format!(
//         r#"
//         mutation {{
//             register_start(identifier: "{}")
//         }}
//         "#,
//         random_string
//     ));
//     let response = execute(request, None).await;
//     let json = response.data.into_json().unwrap();
//     let ccr = json.get("register_start").unwrap().clone();
//     let ccr: CreationChallengeResponse = serde_json::from_value(ccr).unwrap();
//     let ccr_js_value = serde_wasm_bindgen::to_value(&ccr).unwrap();
//     let c_options: CredentialCreationOptions = ccr_js_value.into();

//     let pub_key = "test";

//     let window = window().expect("Unable to get window");

//     let promise = window
//         .navigator()
//         .credentials()
//         .create_with_options(&c_options)
//         .expect("Unable to create promise");

//     let fut = wasm_bindgen_futures::JsFuture::from(promise);

//     //Finish Registration with same account.
//     let request = async_graphql::Request::new(format!(
//         r#"
//         mutation {{
//             register_finish(identifier: "{}", pub_key: "{}")
//         }}
//         "#,
//         random_string, pub_key
//     ));

//     let response = execute(request, None).await;

//     assert!(response.is_ok());
// }
