use async_graphql::{Error, ErrorExtensions};
use bson::Document;
use evalexpr::*;
use http::HeaderMap;
use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::graphql::schema::create_auth_service::TokenData;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guard {
    pub name: String,
    pub if_expr: String,
    pub then_msg: String,
}

impl Guard {
    pub fn check(guards: &Vec<Guard>, guard_context: &mut HashMapContext) -> Result<(), Error> {
        debug!("Checking Guards");

        let mut errors = Vec::new();

        for guard in guards {
            debug!("Checking Item Guard: {:?}", guard);
            let should_guard = eval_boolean_with_context_mut(guard.if_expr.as_str(), guard_context);
            debug!("Should Guard: {:?}", should_guard);
            if should_guard.is_err() {
                error!("Guard Creation Error, {:?}", should_guard);
                return Err(Error::new("Guard Creation Error").extend_with(|_err, e| {
                    e.set(guard.name.clone(), should_guard.err().unwrap().to_string())
                }));
            }
            if should_guard.unwrap() {
                debug!("Guarding");
                errors.push((guard.name.clone(), guard.then_msg.clone()));
            }
        }

        if errors.len() > 0 {
            debug!("Errors: {:?}", errors);
            let mut error_response = Error::new("Guard Error");

            for (name, message) in errors {
                error_response = error_response.extend_with(|_err, e| e.set(name, message));
            }

            return Err(error_response);
        }

        Ok(())
    }

    pub fn create_guard_context(
        headers: HeaderMap,
        token_data: Option<TokenData>,
        input_document: Document,
    ) -> Result<HashMapContext, async_graphql::Error> {
        debug!("Creating Guard Context");

        let context = context_map! {
            "input" => Function::new(move |argument| {
                debug!("Input Argument: {:?}", argument);
                let key = argument.as_string()?;

                let json = serde_json::to_value(input_document.clone()).unwrap();

                let input_value = if key.contains(".") {
                    let keys: Vec<&str> = key.split(".").collect();
                    let mut value = &json[keys[0]];
                    for key in keys.iter().skip(1) {
                        value = &value[key];
                    }
                    debug!("Input Value: {:?}", value);

                    if value.is_null() {
                        return Ok(Value::Empty);
                    }

                    Ok(Value::String(value.as_str().unwrap().to_string()))
                } else {
                    let value = json.get(key);
                    debug!("Input Value: {:?}", value);
                    match value {
                        Some(value) => Ok(Value::String(value.as_str().unwrap().to_string())),
                        None => Ok(Value::Empty)
                    }
                };
                input_value
            }),
            "headers" => Function::new(move |argument| {
                let key = argument.as_string()?;
                let cleaned_key = key.replace("\"", "");
                let value = headers.get(&cleaned_key);
                if value.is_none() {
                    Err(EvalexprError::expected_string(argument.clone()))
                } else {
                    let value = value.unwrap().to_str();
                    if let Ok(value) = value {
                        debug!("Header Value: {:?}", value);
                        Ok(Value::String(value.to_string()))
                    }else {
                        Err(EvalexprError::expected_string(argument.clone()))
                    }
                }
            }),
            "token_data" => Function::new(move |argument| {
                let token_data = match &token_data {
                    Some(token_data) => token_data,
                    None => return Err(EvalexprError::expected_string(argument.clone()))
                };
                let key = argument.as_string()?;
                let cleaned_key = key.replace("\"", "");
                let json = serde_json::to_value(token_data).unwrap();
                let value = json.get(cleaned_key);
                    debug!("Token Data Value: {:?}", value);
                    match value {
                        Some(value) => Ok(Value::String(value.as_str().unwrap().to_string())),
                        None => Err(EvalexprError::expected_string(argument.clone()))
                    }
            }),
        };
        debug!("Guard Context: {:?}", context);
        match context {
            Ok(context) => Ok(context),
            Err(e) => Err(async_graphql::Error::new(e.to_string())),
        }
    }
}
