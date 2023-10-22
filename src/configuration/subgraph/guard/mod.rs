use async_graphql::{Error, ErrorExtensions};
use bson::{doc, Document};
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
            let mut error_response = Error::new("Access Denied");

            for (name, message) in errors {
                error_response = error_response.extend_with(|_err, e| e.set(name, message));
            }

            return Err(error_response);
        }

        Ok(())
    }

    /// The input object provided will have a recursive shape.
    /// {
    ///   "query": {
    ///   ...values,
    ///   AND: [{...typeof_query}],
    ///   OR: [{...typeof_query}]
    ///   }
    /// }
    ///
    /// Returns a vec of queries.
    pub fn extract_input_queries(
        input_document: Document,
    ) -> Result<Vec<Document>, async_graphql::Error> {
        let mut documents = vec![];

        let query_document = match input_document.get("query") {
            Some(q) => q,
            None => {
                error!("Can't find query in document.");
                return Err(async_graphql::Error::new("Can't find query in document."));
            }
        };

        let query_document = query_document.as_document().unwrap();

        let and_queries = query_document.get("AND");
        let or_queries = query_document.get("OR");

        let mut initial_query = query_document.clone();
        initial_query.remove("AND");
        initial_query.remove("OR");

        if !initial_query.is_empty() {
            documents.push(initial_query)
        }

        if and_queries.is_some() {
            for query in and_queries.unwrap().as_array().unwrap() {
                let query_doc = doc! {
                    "query": query.as_document().unwrap().clone()
                };
                let nested = Guard::extract_input_queries(query_doc)?;
                for query in nested {
                    documents.push(query);
                }
            }
        }

        if or_queries.is_some() {
            for query in or_queries.unwrap().as_array().unwrap() {
                let nested = Guard::extract_input_queries(query.as_document().unwrap().clone())?;
                for query in nested {
                    documents.push(query);
                }
            }
        }

        Ok(documents)
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

                let query_documents = match Guard::extract_input_queries(input_document.clone()) {
                    Ok(query_documents) => query_documents,
                    Err(err) => {
                        error!("Error extracting input queries: {:?}", err);
                        return Err(EvalexprError::expected_string(argument.clone()));
                    }
                };

                let mut values_tuple = vec![];
                let is_nested = key.contains(".");

                for input_document in query_documents {
                    let json = serde_json::to_value(input_document.clone()).unwrap();

                    // If the specified input is nested, extract the nested value.
                    if is_nested {
                        let keys: Vec<&str> = key.split(".").collect();
                        let mut value = &json[keys[0]];
                        for key in keys.iter().skip(1) {
                            value = &value[key];
                        }
                        debug!("Input Value: {:?}", value);

                          if value.is_null() {
                            return Ok(Value::Empty);
                        }

                        //TEST: Does this work with bools??
                        let value = Value::String(value.as_str().unwrap().to_string());

                        values_tuple.push(value);
                    } else { // Else extract the value directly.
                        let value = json.get(key.clone());
                        debug!("Input Value: {:?}", value);
                        let value = match value {
                            Some(value) => Value::String(value.as_str().unwrap().to_string()),
                            None => Value::Empty
                        };
                        values_tuple.push(value);
                    };
                }

                // Return a tuple of the found values so that they be used in guard checks.
                let values = Value::from(values_tuple);
                Ok(values)
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
            "every" => Function::new(move |argument| {
                let arguments = argument.as_fixed_len_tuple(2)?;
                if let (Value::Tuple(a), b) = (&arguments[0].clone(), &arguments[1].clone()) {
                    if let Value::String(_) | Value::Int(_) | Value::Float(_) | Value::Boolean(_) = b {
                        Ok(a.iter().all(|x| x == b).into())
                    } else {
                        Err(EvalexprError::type_error(
                            b.clone(),
                            vec![
                                ValueType::String,
                                ValueType::Int,
                                ValueType::Float,
                                ValueType::Boolean,
                            ],
                        ))
                    }
                } else {
                    Err(EvalexprError::expected_tuple(arguments[0].clone()))
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
