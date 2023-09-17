use async_graphql::{Error, ErrorExtensions};
use bson::Document;
use evalexpr::*;
use http::HeaderMap;
use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::{
    configuration::subgraph::entities::{ScalarOptions, ServiceEntityConfig},
    utils::{self, document::get_from_document::GetDocumentResultType},
};

use super::entities::service_entity_field::ServiceEntityFieldConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guard {
    pub name: String,
    pub if_expr: String,
    pub then_msg: String,
}

impl Guard {
    pub fn check(guards: &Vec<Guard>, guard_context: &HashMapContext) -> Result<(), Error> {
        debug!("Checking Guards");

        let mut errors = Vec::new();

        for guard in guards {
            debug!("Checking Item Guard: {:?}", guard);
            let should_guard = eval_boolean_with_context(guard.if_expr.as_str(), guard_context);
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

    pub fn get_input_value(
        input_document: Document,
        mut fields: Vec<ServiceEntityFieldConfig>,
    ) -> Result<Value, EvalexprError> {
        debug!("Getting from Document");
        let document_value =
            utils::document::DocumentUtils::get_from_document(&input_document, &fields[0]);

        if document_value.is_err() {
            return Err(EvalexprError::CustomMessage(
                "Input field not found.".to_string(),
            ));
        }

        match fields[0].scalar {
            ScalarOptions::String
            | ScalarOptions::Int
            | ScalarOptions::Boolean
            | ScalarOptions::ObjectID => {
                if fields.len() > 1 {
                    return Err(EvalexprError::CustomMessage(
                        "Can not access property from primitive.".to_string(),
                    ));
                } else {
                    match document_value.unwrap() {
                        GetDocumentResultType::String(value) => {
                            debug!("Value: {:?}", value);
                            return Ok(Value::String(value));
                        }
                        GetDocumentResultType::StringArray(value) => {
                            debug!("Value: {:?}", value);
                            if value.len() == 0 {
                                return Err(EvalexprError::CustomMessage(
                                    "Input Value Required".to_string(),
                                ));
                            }
                            return Ok(Value::Tuple(
                                value.into_iter().map(Value::String).collect(),
                            ));
                        }
                        GetDocumentResultType::Int(value) => {
                            debug!("Value: {:?}", value);
                            return Ok(Value::Int(value as i64));
                        }
                        GetDocumentResultType::IntArray(value) => {
                            debug!("Value: {:?}", value);
                            if value.len() == 0 {
                                return Err(EvalexprError::CustomMessage(
                                    "Input Value Required".to_string(),
                                ));
                            }
                            return Ok(Value::Tuple(
                                value.into_iter().map(|x| Value::Int(x as i64)).collect(),
                            ));
                        }
                        GetDocumentResultType::Boolean(value) => {
                            debug!("Value: {:?}", value);
                            return Ok(Value::Boolean(value));
                        }
                        GetDocumentResultType::BooleanArray(value) => {
                            debug!("Value: {:?}", value);
                            if value.len() == 0 {
                                return Err(EvalexprError::CustomMessage(
                                    "Input Value Required".to_string(),
                                ));
                            }
                            return Ok(Value::Tuple(
                                value.into_iter().map(Value::Boolean).collect(),
                            ));
                        }
                        _ => {
                            return Err(EvalexprError::CustomMessage(
                                "Value is not primitive.".to_string(),
                            ));
                        }
                    }
                }
            }
            ScalarOptions::Object => {
                if fields[0].list.unwrap_or(false) {
                    let mut values = vec![];
                    match document_value.unwrap() {
                        GetDocumentResultType::DocumentArray(document) => {
                            fields.remove(0);
                            for document in document {
                                values.push(
                                    Guard::get_input_value(document, fields.clone()).unwrap(),
                                );
                            }
                            debug!("Values: {:?}", values);
                            Ok(Value::Tuple(values))
                        }
                        _ => {
                            return Err(EvalexprError::CustomMessage(
                                "Expected document array.".to_string(),
                            ));
                        }
                    }
                } else {
                    fields.remove(0);
                    match document_value.unwrap() {
                        GetDocumentResultType::Document(value) => {
                            Ok(Guard::get_input_value(value, fields).unwrap())
                        }
                        _ => {
                            return Err(EvalexprError::CustomMessage(
                                "Expected document object.".to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }

    pub fn create_guard_context<'a>(
        headers: HeaderMap,
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
                    Ok(Value::String(value.as_str().unwrap().to_string()))
                } else {
                    let value = json.get(key);
                    debug!("Input Value: {:?}", value);
                    match value {
                        Some(value) => Ok(Value::String(value.as_str().unwrap().to_string())),
                        None => Err(EvalexprError::expected_string(argument.clone()))
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
            })
        };
        debug!("Guard Context: {:?}", context);
        match context {
            Ok(context) => Ok(context),
            Err(e) => Err(async_graphql::Error::new(e.to_string())),
        }
    }
}
