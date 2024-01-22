use async_graphql::{Error, ErrorExtensions};
use bson::{doc, Document};
use evalexpr::*;
use guard_data_context::GuardDataContext;
use http::HeaderMap;
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};

use crate::{
    configuration::subgraph::{entities::ServiceEntityConfig, SubGraphConfig},
    filter_operator::FilterOperator,
    graphql::schema::create_auth_service::TokenData,
    scalar_option::ScalarOption,
    utils::clean_string::clean_string,
};

pub mod guard_data_context;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guard {
    pub name: String,
    pub if_expr: String,
    pub then_msg: String,
    pub context: Option<Vec<GuardDataContext>>,
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
                return Err(Error::new(guard.then_msg.clone()).extend_with(|_err, e| {
                    e.set(
                        "guard_creation_error",
                        should_guard.err().unwrap().to_string(),
                    );
                }));
            }
            if should_guard.unwrap() {
                debug!("Guarding");
                errors.push((guard.name.clone(), guard.then_msg.clone()));
            }
        }

        if errors.len() > 0 {
            debug!("Errors: {:?}", errors);
            // Use the first error message as the main error message.
            let mut error_response = Error::new(errors[0].1.clone());

            for (name, message) in errors {
                error_response = error_response.extend_with(|_err, e| e.set(name, message));
            }

            return Err(error_response);
        }

        Ok(())
    }

    pub fn extract_input_values(input_document: Document) -> Result<Document, EvalexprError> {
        let exclude_keys = FilterOperator::list()
            .iter()
            .map(|op| op.as_str().to_string())
            .collect::<Vec<String>>();

        let values_input = match input_document.get("values") {
            Some(values_input) => values_input.as_document(),
            None => {
                error!("Missing input property `values`.");
                return Err(EvalexprError::CustomMessage(
                    "Missing input property `values`.".to_string(),
                ));
            }
        };

        match values_input {
            Some(values_input) => {
                let mut cloned_values = values_input.clone();
                for key in &exclude_keys {
                    if cloned_values.contains_key(key) {
                        cloned_values.remove(key);
                    }
                }
                Ok(cloned_values)
            }
            None => {
                error!("Failed to parse `values` input.");
                return Err(EvalexprError::CustomMessage(
                    "Failed to parse `values` input.".to_string(),
                ));
            }
        }
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
    pub fn extract_input_queries(input_document: Document) -> Result<Vec<Document>, EvalexprError> {
        debug!("Extracting Input Queries: {:?}", input_document);
        let mut documents = vec![];

        let query_document = match input_document.get("query") {
            Some(q) => q,
            None => {
                error!("Can't find property `query` when parsing query in input guard.");
                return Err(EvalexprError::CustomMessage(
                    "Can't find property `query` when parsing query in input guard.".to_string(),
                ));
            }
        };

        let query_document = query_document.as_document().unwrap();

        let and_queries = query_document.get(FilterOperator::And.as_str());
        let or_queries = query_document.get(FilterOperator::Or.as_str());

        let mut initial_query = query_document.clone();
        initial_query.remove(FilterOperator::And.as_str());
        initial_query.remove(FilterOperator::Or.as_str());

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
                let query_doc = doc! {
                    "query": query.as_document().unwrap().clone()
                };
                let nested = Guard::extract_input_queries(query_doc)?;
                for query in nested {
                    documents.push(query);
                }
            }
        }

        debug!("Extracted Queries: {:?}", documents);
        Ok(documents)
    }

    pub fn create_guard_context(
        headers: HeaderMap,
        token_data: Option<TokenData>,
        input_document: Document,
        resolver_type: String,
        data_context: Option<serde_json::Value>,
        data_contexts: Option<Vec<GuardDataContext>>,
        subgraph_config: SubGraphConfig,
    ) -> Result<HashMapContext, async_graphql::Error> {
        debug!("Creating Guard Context");

        let context = context_map! {
            "input" => Function::new(move |argument| {
                debug!("Input Argument: {:?}", argument);
                let arguments = argument.as_fixed_len_tuple(2)?;
                if let (Value::String(root), key) = (&arguments[0].clone(), &arguments[1].clone()) {
                    let matching_keys = vec!["query", "values"];

                    if !matching_keys.contains(&root.as_str()) {
                        error!("First key in input guard must be a key of the input object.");
                        return Err(EvalexprError::CustomMessage("First key in input guard must be a key of the input object.".to_string()))
                    }

                    if let Value::String(key) = key {
                        let documents;

                        if root == "query" {
                            documents = Guard::extract_input_queries(input_document.clone())?
                        } else {
                            let values_document = Guard::extract_input_values(input_document.clone())?;
                            documents = vec![values_document];
                        }

                        let mut values_tuple = vec![];
                        let is_nested = key.contains(".");
                        let excluded_keys = FilterOperator::list().iter().map(|op| op.as_str().to_string()).collect::<Vec<String>>();

                        for input_document in documents {
                            let json = serde_json::to_value(input_document.clone()).unwrap();
                            trace!("Input Json Document: {:?}", json);

                            // If the specified input is nested, extract the nested value.
                            if is_nested {
                                let keys: Vec<&str> = key.split(".").collect();
                                trace!("Input Nested Keys: {:?}", keys);
                                let mut value = &json[keys[0]];
                                for key in keys.iter().skip(1) {
                                    trace!("Input Nested Key: {:?}", key);
                                    if excluded_keys.contains(&key.to_string()) {
                                        continue;
                                    }
                                    value = &value[key];
                                }

                                if value.is_null() {
                                    continue;
                                }

                                let value = value.to_string();

                                let value = Value::String(clean_string(&value, None));

                                values_tuple.push(value);
                            } else { // Else extract the value directly.
                                if excluded_keys.contains(&key.to_string()) {
                                    continue;
                                }
                                let value = json.get(key.clone());
                                let value = match value {
                                    Some(value) => Value::String(clean_string(&value.to_string(), None)),
                                    None => continue,
                                };
                                values_tuple.push(value);
                            };
                        }

                        // Return a tuple of the found values so that they be used in guard checks.
                        let values = Value::from(values_tuple);
                        debug!("Input Value Tuples: {:?}", values);
                        Ok(values)
                    } else {
                        error!("Arguments [1] incorrect.");
                        Err(EvalexprError::expected_string(arguments[1].clone()))
                    }
                } else {
                    error!("Arguments [0] incorrect.");
                    Err(EvalexprError::expected_string(arguments[0].clone()))
                }
            }),
            "context" => Function::new(move |argument| {
                debug!("Context Argument: {:?}", argument);
                let mut values_tuple = vec![];
                let data_context = match &data_context {
                    Some(data_context) => data_context,
                    None => {
                        error!("Data Context not found.");
                        return Err(EvalexprError::CustomMessage("Data Context not found.".to_string()))
                    }
                };
                let key = argument.as_string()?;
                let cleaned_key = clean_string(&key, None);

                let root_key = cleaned_key.split(".").collect::<Vec<&str>>()[0];
                let root_value = data_context.get(root_key);

                if root_value.is_none() {
                    return Ok(Value::Tuple(vec![]));
                }

                let data_contexts = match &data_contexts {
                    Some(data_contexts) => data_contexts,
                    None => {
                        error!("Data Contexts not found.");
                        return Err(EvalexprError::CustomMessage("Data Contexts not found.".to_string()))
                    }
                };

                let guard_data_context = &data_contexts.iter().find(|data_context| {
                    if data_context.name.is_some() {
                        return data_context.name.clone().unwrap() == root_key.to_string()
                    } else {
                        return data_context.entity_name == root_key.to_string()
                    }
                });

                if guard_data_context.is_none() {
                    return Err(EvalexprError::CustomMessage("Data Context not found.".to_string()))
                }

                let entity_name = guard_data_context.unwrap().entity_name.clone();

                let entity = SubGraphConfig::get_entity(subgraph_config.clone(), &entity_name);
                if entity.is_none() {
                    return Err(EvalexprError::CustomMessage("Entity not found.".to_string()))
                }
                let entity = entity.unwrap();

                // Root value should be a vector of entities, loop through each one and extract the
                // value. Add the value to the values_tuple.
                if root_value.unwrap().is_array() {
                    // remove the first key/root key from the cleaned key.
                    let cleaned_key = cleaned_key.split(".").collect::<Vec<&str>>()[1..].join(".");
                    let is_nested = cleaned_key.contains(".");

                    // For each value of the array, extract the key's value from the entity.
                    for value in root_value.unwrap().as_array().unwrap() {
                        if is_nested {
                            debug!("Nested Context Key: {:?}", cleaned_key);
                            let keys: Vec<&str> = cleaned_key.split(".").collect();
                            let mut value = &value[keys[0]];
                            for key in keys.iter().skip(1) {
                                value = &value[key];
                            }

                            if value.is_null() {
                                return Ok(Value::Tuple(vec![]));
                            }

                            let value = value.to_string();

                            let value = Value::String(value);
                            let cleaned = clean_string(&value.to_string(), None);
                            debug!("Context Value: {:?}", cleaned);

                            values_tuple.push(value);
                        } else {
                            debug!("Context Key: {:?}", cleaned_key.clone());
                            let value = value.get(cleaned_key.clone());
                            let field = match ServiceEntityConfig::get_field(entity.clone(), cleaned_key.clone()) {
                                Ok(field) => field,
                                Err(e)=> {
                                    error!("Field not found: {:?}", e);
                                    return Err(EvalexprError::CustomMessage("Failed to parse context: Field not found.".to_string()))
                                }
                            };

                            debug!("Context Value: {:?}", value);
                            let value = match value {
                                Some(value) => {
                                    if value.is_null() {
                                        return Ok(Value::Empty);
                                    }
                                    match field.scalar {
                                        ScalarOption::String => Value::String(clean_string(&value.to_string(), None)),
                                        ScalarOption::Int => Value::Int(value.as_i64().unwrap()),
                                        ScalarOption::Boolean => Value::Boolean(value.as_bool().unwrap()),
                                        ScalarOption::DateTime => Value::String(value.to_string()),
                                        ScalarOption::UUID => Value::String(clean_string(&value.to_string(), None)),
                                        ScalarOption::ObjectID => {
                                            let object_id = value.get("$oid");
                                            if object_id.is_none() {
                                                return Err(EvalexprError::CustomMessage("ObjectID not found.".to_string()))
                                            }
                                            let object_id = object_id.unwrap().as_str().unwrap();
                                            Value::String(object_id.to_string())
                                        },
                                        _ => return Err(EvalexprError::CustomMessage("Scalar is not supported in context.".to_string()))
                                    }

                                },
                                None => Value::Empty,
                            };
                            debug!("Context Value: {:?}", value);

                            values_tuple.push(value);
                        }
                    }

                } else {
                    return Err(EvalexprError::CustomMessage("Context must be an array.".to_string()))
                }


                // Return a tuple of the found values so that they be used in guard checks.
                let values = Value::from(values_tuple);
                debug!("Context Value Tuples: {:?}", values);
                Ok(values)
            }),
            "headers" => Function::new(move |argument| {
                let key = argument.as_string()?;
                let cleaned_key = clean_string(&key, None);
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
                    None => {
                        error!("Token Data not found.");
                        return Err(EvalexprError::CustomMessage("Token Data not found.".to_string()))
                    }
                };
                let key = argument.as_string()?;
                let cleaned_key = clean_string(&key, None);
                let json = match serde_json::to_value(token_data) {
                    Ok(json) => json,
                    Err(_) => {
                        error!("Token Data not found.");
                        return Err(EvalexprError::CustomMessage("Failed to serialize token data.".to_string()))
                    }
                };
                let value = json.get(cleaned_key);
                    debug!("Token Data Value: {:?}", value);
                    match value {
                        Some(value) => Ok(Value::String(value.as_str().unwrap().to_string())),
                        None => {
                            error!("Token Data Value not found.");
                            Err(EvalexprError::CustomMessage("Token Data Value not found.".to_string()))
                        }
                    }
            }),
            "resolver_type" => Function::new(move |_| {
                Ok(Value::String(resolver_type.clone()))
            }),
            "every" => Function::new(move |argument| {
                debug!("Guard Function - Every: {:?}", argument);
                let arguments = argument.as_fixed_len_tuple(2)?;
                if let (Value::Tuple(a), b) = (&arguments[0].clone(), &arguments[1].clone()) {
                    if let Value::String(_) | Value::Int(_) | Value::Float(_) | Value::Boolean(_) = b {
                        if a.len() == 0 {
                            return Ok(Value::Boolean(false));
                        }
                        Ok(a.iter().all(|x| x == b).into())
                    } else {
                        error!("Invalid type.");
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
                    Ok(Value::Boolean(false))
                }
            })
        };
        debug!("Guard Context: {:?}", context);
        match context {
            Ok(context) => Ok(context),
            Err(e) => {
                error!("Error parsing guard context: {:?}", e);
                Err(async_graphql::Error::new(e.to_string()))
            }
        }
    }

    /// Provided the guards from the service, entities, resolvers, and fields, this function will
    /// return a list of all the data contexts.
    pub fn get_guard_data_contexts(
        service_guards: Option<Vec<Guard>>,
        entity_guards: Option<Vec<Guard>>,
        resolver_guards: Option<Vec<Guard>>,
        field_guards: Option<Vec<Guard>>,
    ) -> Vec<GuardDataContext> {
        debug!("Getting Guard Data Contexts");
        let mut contexts = vec![];

        if let Some(service_guards) = service_guards {
            for guard in service_guards {
                if let Some(context) = guard.context {
                    for context in context {
                        contexts.push(context);
                    }
                }
            }
        }

        if let Some(entity_guards) = entity_guards {
            for guard in entity_guards {
                if let Some(context) = guard.context {
                    for context in context {
                        contexts.push(context);
                    }
                }
            }
        }

        if let Some(resolver_guards) = resolver_guards {
            for guard in resolver_guards {
                if let Some(context) = guard.context {
                    for context in context {
                        contexts.push(context);
                    }
                }
            }
        }

        if let Some(field_guards) = field_guards {
            for guard in field_guards {
                if let Some(context) = guard.context {
                    for context in context {
                        contexts.push(context);
                    }
                }
            }
        }

        debug!("Guard Data Contexts: {:?}", contexts);
        contexts
    }
}
