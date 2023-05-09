use async_graphql::{Error, ErrorExtensions};
use bson::{Bson, Document};
use evalexpr::*;
use http::HeaderMap;
use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::configuration::subgraph::entities::{ScalarOptions, ServiceEntity};

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
                return Err(Error::new("Guard Creation Error")
                    .extend_with(|_err, e| e.set(guard.name.clone(), "Required when guarding.")));
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

    pub fn get_from_document(
        input_document: Document,
        cleaned_key: String,
        scalar: ScalarOptions,
        is_list: bool,
        argument: &Value,
    ) -> Result<Value, EvalexprError> {
        debug!("Getting Value from Document");
        debug!("Document: {:?}", input_document);
        debug!("Key: {:?}", cleaned_key);
        debug!("Scalar: {:?}", scalar);
        debug!("Is List: {:?}", is_list);
        if is_list {
            if let Some(Bson::Array(documents)) = input_document.get(&cleaned_key) {
                let mut values = Vec::new();
                for document in documents {
                    match scalar {
                        ScalarOptions::String | ScalarOptions::ObjectID => {
                            println!("Document: {:?}", document);
                            let cleaned_string = document.to_string().replace("\"", "");
                            values.push(Value::String(cleaned_string))
                        }
                        ScalarOptions::Int => {
                            values.push(Value::Int(document.as_i32().unwrap() as i64))
                        }
                        ScalarOptions::Boolean => {
                            values.push(Value::Boolean(document.as_bool().unwrap()))
                        }
                        _ => unreachable!(),
                    }
                }
                debug!("Values: {:?}", values);
                Ok(Value::from(values))
            } else {
                return Err(EvalexprError::expected_string(argument.clone()));
            }
        } else {
            match input_document.clone().get(&cleaned_key) {
                Some(v) => {
                    match scalar {
                        ScalarOptions::String | ScalarOptions::ObjectID => {
                            Ok(Value::String(v.to_string()))
                        }
                        ScalarOptions::Int => Ok(Value::Int(v.as_i32().unwrap() as i64)),
                        ScalarOptions::Boolean => Ok(Value::Boolean(v.as_bool().unwrap())),
                        //TODO: Implement Object scalars
                        _ => unreachable!(),
                    }
                }
                //TODO: Eval Scalar Error
                None => Err(EvalexprError::expected_string(argument.clone())),
            }
        }
    }

    pub fn create_guard_context<'a>(
        headers: HeaderMap,
        input_document: Document,
        entity: ServiceEntity,
    ) -> Result<HashMapContext, EvalexprError> {
        debug!("Creating Guard Context");

        let context = context_map! {
            "input" => Function::new(move |argument| {
                debug!("Input Argument: {:?}", argument);
                let key = argument.to_string();
                let cleaned_key = key.replace("\"", "");
                let field = ServiceEntity::get_field(&entity.clone(), &cleaned_key);
                if field.is_none() {
                    return Err(EvalexprError::expected_string(argument.clone()))
                }
                let scalar = field.clone().unwrap().scalar;
                let is_list = field.unwrap().list.unwrap_or(false);
                Guard::get_from_document(input_document.clone(), cleaned_key, scalar, is_list, argument)
            }),
            "headers" => Function::new(move |argument| {
                let key = argument.to_string();
                let cleaned_key = key.replace("\"", "");
                let value = headers.get(&cleaned_key);
                if value.is_none() {
                    Err(EvalexprError::expected_string(argument.clone()))
                } else {
                    let value = value.unwrap().to_str();
                    if let Ok(value) = value {
                        Ok(Value::String(value.to_string()))
                    }else {
                        Err(EvalexprError::expected_string(argument.clone()))
                    }
                }
            }),
        };
        debug!("Guard Context: {:?}", context);
        context
    }
}
