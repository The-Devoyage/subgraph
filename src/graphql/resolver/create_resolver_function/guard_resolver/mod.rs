use async_graphql::{ErrorExtensions, SelectionField};
use bson::{to_document, Document};
use evalexpr::{eval_with_context_mut, HashMapContext};
use http::HeaderMap;
use log::{debug, error, trace};
use serde_json::Value;

use crate::{
    configuration::subgraph::{
        entities::{service_entity_field::ServiceEntityFieldConfig, ServiceEntityConfig},
        guard::{
            guard_data_context::{GuardDataContext, VariablePair},
            Guard,
        },
        SubGraphConfig,
    },
    data_sources::{sql::services::ResponseRow, DataSources},
    graphql::{
        entity::create_return_types::ResolverResponse, schema::create_auth_service::TokenData,
    },
    resolver_type::ResolverType,
    utils::clean_string::clean_string,
};

use super::ServiceResolver;

impl ServiceResolver {
    pub async fn guard_resolver_function(
        selection_fields: Vec<SelectionField<'_>>,
        input_document: &Document,
        entity: &ServiceEntityConfig,
        service_guards: Option<Vec<Guard>>,
        resolver_type: &ResolverType,
        headers: HeaderMap,
        token_data: &Option<TokenData>,
        data_sources: &DataSources,
        subgraph_config: &SubGraphConfig,
    ) -> Result<HashMapContext, async_graphql::Error> {
        debug!("Guard Resolver Function");

        // Get the guards
        let resolver_guards =
            match ServiceEntityConfig::get_resolver(&entity, resolver_type.clone()) {
                Some(resolver) => {
                    debug!(
                        "Guarding resolver {:?} for entity {:?}",
                        resolver_type, entity.name
                    );
                    resolver.guards
                }
                None => None,
            };
        let field_guards = ServiceResolver::get_field_guards(selection_fields, &entity, None)
            .map_err(|e| {
                error!("Error getting field guards: {:?}", e);
                async_graphql::Error::new("Error getting field guards")
            })?;

        // Handle guard data contexts to inject data from the graph into the eval context.
        let guard_data_contexts = Guard::get_guard_data_contexts(
            service_guards.clone(),
            entity.guards.clone(),
            resolver_guards.clone(),
            Some(field_guards.clone()),
        );

        // Create the context to parse the variables in the the `if_expr` and the `query`
        let mut guard_context = Guard::create_guard_context(
            headers.clone(),
            token_data.clone(),
            input_document.clone(),
            resolver_type.to_string(),
            None,
            Some(guard_data_contexts.clone()),
            subgraph_config.clone(),
        )
        .map_err(|e| {
            error!("Error creating guard context: {:?}", e);
            async_graphql::Error::new("Error creating guard context")
        })?;

        // Fetch the data from the data source and return it as json
        let data_context = ServiceResolver::execute_data_context(
            guard_data_contexts.clone(),
            data_sources,
            subgraph_config,
            guard_context,
            headers.clone(),
            token_data,
            resolver_type,
            input_document.clone(),
        )
        .await
        .map_err(|e| {
            error!("Error executing data context: {:?}", e);
            async_graphql::Error::new("Error executing data context")
        })?;

        // Re-create the evalexpr context including the data context
        guard_context = Guard::create_guard_context(
            headers,
            token_data.clone(),
            input_document.clone(),
            resolver_type.to_string(),
            Some(data_context),
            Some(guard_data_contexts),
            subgraph_config.clone(),
        )?;

        // Execute the guards
        match entity.guards.clone() {
            Some(guards) => Guard::check(&guards, &mut guard_context.clone())?,
            None => (),
        };
        if service_guards.is_some() {
            Guard::check(&service_guards.unwrap(), &mut guard_context.clone())?;
        }
        if resolver_guards.is_some() {
            Guard::check(&resolver_guards.unwrap(), &mut guard_context.clone())?;
        }
        if field_guards.len() > 0 {
            Guard::check(&field_guards, &mut guard_context.clone())?;
        }

        Ok(guard_context)
    }

    /// Get entity field guards, recursively.
    pub fn get_field_guards(
        selection_fields: Vec<SelectionField>,
        entity: &ServiceEntityConfig,
        mut prefix: Option<String>,
    ) -> Result<Vec<Guard>, async_graphql::Error> {
        debug!("Getting field guards");
        let mut field_guards = vec![];

        for selection_field in selection_fields {
            if selection_field.name() == "__typename" {
                continue;
            }

            let field_name = format!(
                "{}{}",
                prefix.clone().unwrap_or("".to_string()),
                selection_field.name()
            );
            let fields = ServiceEntityFieldConfig::get_fields_recursive(
                entity.fields.clone(),
                field_name.to_string(),
            )?;

            let field = fields.iter().last().unwrap(); // Last field found will be the one we want.

            if selection_field.selection_set().count() > 0 && field.as_type.is_none() {
                prefix = Some(format!("{}.", field_name));
                for selection_field in selection_field.selection_set().into_iter() {
                    // Call this function recursively to get the field guards.
                    let mut nested_field_guards = ServiceResolver::get_field_guards(
                        vec![selection_field],
                        &entity,
                        prefix.clone(),
                    )?;
                    field_guards.append(&mut nested_field_guards);
                }
            }
        }
        trace!("Field guards: {:?}", field_guards);
        Ok(field_guards)
    }

    /// Fetches from the DB the requested data for context.
    /// Adds the data to the context.
    /// Returns the context.
    pub async fn execute_data_context(
        guard_data_contexts: Vec<GuardDataContext>,
        data_sources: &DataSources,
        subgraph_config: &SubGraphConfig,
        mut guard_context: HashMapContext,
        headers: HeaderMap,
        token_data: &Option<TokenData>,
        resolver_type: &ResolverType,
        input: Document,
    ) -> Result<Value, async_graphql::Error> {
        debug!(
            "Execute Data Contexts: {:?}",
            guard_data_contexts.iter().map(|g| {
                if g.name.is_some() {
                    g.name.clone().unwrap()
                } else {
                    g.entity_name.clone()
                }
            })
        );
        let mut data_context = serde_json::json!({});
        for guard_data_context in guard_data_contexts.clone() {
            let input_name = format!("get_{}s_input", guard_data_context.entity_name);

            // Get the entity to determine data_source.
            let entity = match subgraph_config
                .clone()
                .get_entity(&guard_data_context.entity_name)
            {
                Some(entity) => entity,
                None => {
                    return Err(async_graphql::Error::new(
                        "Can't use an entity that does not exist in guard data context."
                            .to_string(),
                    ))
                }
            };
            let mut query = guard_data_context.query.clone();
            let variables = guard_data_context.variables;

            query = ServiceResolver::replace_variables_in_query(query, variables, guard_context)?;

            let json: Value = match serde_json::from_str(&query) {
                Ok(json) => json,
                Err(error) => {
                    error!(
                        "Can't parse the guard data context query. Failed to parse JSON: {:?}",
                        error
                    );
                    return Err(async_graphql::Error::new(
                        "Can't parse the guard data context query. Failed to parse JSON."
                            .to_string(),
                    )
                    .extend_with(|_, e| e.set("error", error.to_string())));
                }
            };

            let input_document = to_document(&json).map_err(|error| {
                error!(
                    "Can't parse the guard data context query. Failed to convert to document: {:?}",
                    error
                );
                async_graphql::Error::new(
                    "Can't parse the guard data context query. Failed to convert to document."
                        .to_string(),
                )
                .extend_with(|_, e| e.set("error", error.to_string()))
            })?;

            let input_query = input_document.get(&input_name);

            if input_query.is_none() {
                error!("Can't parse the guard data context query. Failed to get input query.");
                return Err(async_graphql::Error::new(
                    "Can't parse the guard data context query. Failed to get input query."
                        .to_string(),
                ));
            }

            let input_query_document = input_query.unwrap().as_document();

            if input_query_document.is_none() {
                return Err(async_graphql::Error::new(
                    "Can't parse the guard data context query. Failed to get input query document."
                        .to_string(),
                ));
            }

            let has_selection_set = true;

            //Execute the operation to get the data.
            let results = DataSources::execute(
                &data_sources,
                input_query_document.unwrap().to_owned(),
                entity.clone(),
                ResolverType::FindMany,
                subgraph_config,
                token_data,
                has_selection_set,
            )
            .await?;

            if results.is_none() {
                debug!("No results found for data context query.");
                return Ok(data_context);
            }

            let results = results.unwrap();

            let downcasted = results.try_downcast_ref::<ResolverResponse>()?;

            // iterate and turn all to json so it can be parsed when guarding.
            let mut results_json = serde_json::json!([]);
            for result in &downcasted.data {
                let downcasted = result.try_downcast_ref::<Option<Document>>();
                if downcasted.is_err() {
                    debug!(
                        "Attempting to downcast result to Option<Document> failed. Attempting SQL Downcast.",
                    );
                    let downcasted = result.try_downcast_ref::<Option<ResponseRow>>();
                    if downcasted.is_err() {
                        error!(
                            "Failed to get result from data context query. Error: {:?}",
                            downcasted.err()
                        );
                        return Err(async_graphql::Error::new(
                            "Can't parse the guard data context query. Failed to get result."
                                .to_string(),
                        ));
                    }
                    let result = downcasted.unwrap();
                    if result.is_none() {
                        debug!("Data Context Result is none.");
                        continue;
                    }
                    let response_row = result.as_ref().unwrap();

                    let mut json_obj = serde_json::json!({});

                    for field in entity.fields.iter() {
                        if field.as_type.is_some() && field.join_from.is_some() {
                            continue;
                        }
                        if field.is_virtual.unwrap_or(false) {
                            continue;
                        }
                        let json_value = field
                            .scalar
                            .clone()
                            .rr_to_serde_json_value(response_row, &field.name)?;
                        json_obj[field.name.clone()] = json_value;
                    }

                    results_json
                        .as_array_mut()
                        .unwrap()
                        .push(serde_json::json!(json_obj));

                    continue;
                }
                let result = downcasted.unwrap().to_owned();
                if result.is_none() {
                    continue;
                }
                let result = result.unwrap();
                let json = serde_json::to_string(&result);
                if json.is_err() {
                    error!(
                        "Failed to get result from data context query. Error: {:?}",
                        json.err()
                    );
                    return Err(async_graphql::Error::new(
                        "Can't parse the guard data context query. Failed to get result."
                            .to_string(),
                    ));
                }
                let json = json.unwrap();
                results_json
                    .as_array_mut()
                    .unwrap()
                    .push(serde_json::from_str(&json).unwrap());
            }

            // If provded with `name`, use this as the key in the data context.
            // Otherwise, use the entity name.
            let key_name = if let Some(key_name) = guard_data_context.name {
                key_name
            } else {
                guard_data_context.entity_name.clone()
            };
            data_context[key_name] = results_json;

            let query_input = input_document.get(&input_name);

            if query_input.is_none() {
                return Err(async_graphql::Error::new(
                    "Can't parse the guard data context query. Failed to get input query."
                        .to_string(),
                ));
            }

            let query_input = query_input.unwrap().as_document();
            if query_input.is_none() {
                return Err(async_graphql::Error::new(
                    "Can't parse the guard data context query. Failed to get input query document."
                        .to_string(),
                ));
            }

            // Recreate the context to include the data context for subsequent guards.
            let updated_guard_context =
                match Guard::create_guard_context(
                    headers.clone(),
                    token_data.clone(),
                    input.clone(),
                    resolver_type.to_string().clone(),
                    Some(data_context.clone()),
                    Some(guard_data_contexts.clone()),
                    subgraph_config.clone(),
                ) {
                    Ok(guard_context) => guard_context,
                    Err(_) => return Err(async_graphql::Error::new(
                        "Can't parse the guard data context query. Failed to create guard context."
                            .to_string(),
                    )),
                };
            // replace the guard context with the updated one.
            guard_context = updated_guard_context;
        }

        debug!("Data Context: {:?}", data_context);
        Ok(data_context)
    }

    /// Replaces the variables in a query string with the values from the data context.
    pub fn replace_variables_in_query(
        mut query: String,
        variables: Vec<VariablePair>,
        mut guard_context: HashMapContext,
    ) -> Result<String, async_graphql::Error> {
        debug!("Creating query string for data context: {:?}", query);
        for variable in variables {
            let key = variable.0;
            let value = variable.1;

            let value = eval_with_context_mut(&value, &mut guard_context)?;

            if value.is_empty() {
                return Err(async_graphql::Error::new(
                    "Can't parse the guard data context query. Failed to parse empty value."
                        .to_string(),
                ));
            }

            // If value is a tuple, map it to json array.
            // Else simply replace.
            if value.is_string() || value.is_number() {
                let str_value = value.to_string();
                query = query.replace(&key, &str_value);
            }
            if value.is_tuple() {
                if value.as_tuple().unwrap().len() == 0 {
                    error!(
                        "Replace Variables In Query: Tuple length is 0. Query: {:?}",
                        query
                    );
                    return Err(async_graphql::Error::new(format!(
                        "Can't parse the guard data context query. Variable `{key}` not provided.",
                    ))
                    .extend_with(|_, e| e.set("query", query.clone())));
                }
                let mut json_array = serde_json::json!([]);
                // For each tuple, we need to push it into the json array as an object.
                for tuple in value.as_tuple().unwrap() {
                    let mut json_object = serde_json::json!({});

                    if tuple.is_string() {
                        let tuple_value = tuple.to_string();
                        json_object[key.clone().replace("{{", "").replace("}}", "")] =
                            serde_json::json!(clean_string(&tuple_value, None));
                    } else if tuple.is_boolean() {
                        let tuple_value = tuple.as_boolean();
                        json_object[key.clone().replace("{{", "").replace("}}", "")] =
                            serde_json::json!(tuple_value.unwrap());
                    } else if tuple.is_int() {
                        let tuple_value = tuple.as_int();
                        json_object[key.clone().replace("{{", "").replace("}}", "")] =
                            serde_json::json!(tuple_value.unwrap());
                    }

                    json_array.as_array_mut().unwrap().push(json_object);
                }
                query = query.replace(&key, &json_array.to_string());
            }
        }
        debug!("Query String with Variables Replaced: {:?}", query);
        Ok(query)
    }
}
