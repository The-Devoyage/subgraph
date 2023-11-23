use async_graphql::{ErrorExtensions, SelectionField};
use bson::{to_document, Document};
use evalexpr::{eval_with_context_mut, HashMapContext};
use http::HeaderMap;
use log::{debug, error};
use serde_json::Value;

use crate::{
    configuration::subgraph::{
        entities::{service_entity_field::ServiceEntityFieldConfig, ServiceEntityConfig},
        guard::{guard_data_context::GuardDataContext, Guard},
        SubGraphConfig,
    },
    data_sources::{sql::services::ResponseRow, DataSources},
    graphql::schema::{create_auth_service::TokenData, ResolverType},
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
        token_data: Option<TokenData>,
        data_sources: &DataSources,
        subgraph_config: &SubGraphConfig,
    ) -> Result<(), async_graphql::Error> {
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
        let field_guards = ServiceResolver::get_field_guards(selection_fields, &entity)?;

        // Handle guard data contexts to inject data from the graph into the eval context.
        let guard_data_contexts = Guard::get_guard_data_contexts(
            service_guards.clone(),
            entity.guards.clone(),
            resolver_guards.clone(),
            Some(field_guards.clone()),
        );

        // Create the context to parse the variables in the the `if_expr` and the `query`
        let mut guard_context = Guard::create_guard_context(
            headers,
            token_data,
            input_document.clone(),
            resolver_type.to_string(),
        )?;

        let data_context = ServiceResolver::execute_data_context(
            guard_data_contexts,
            data_sources,
            subgraph_config,
            &mut guard_context,
        )
        .await?;

        debug!("Guard Data Context: {:?}", data_context);

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

        Ok(())
    }

    /// Get entity field guards, recursively.
    pub fn get_field_guards(
        selection_fields: Vec<SelectionField>,
        entity: &ServiceEntityConfig,
    ) -> Result<Vec<Guard>, async_graphql::Error> {
        let mut field_guards = vec![];

        for selection_field in selection_fields {
            if selection_field.name() == "__typename" {
                continue;
            }

            let field_name = selection_field.name();
            let fields = ServiceEntityFieldConfig::get_fields_recursive(
                entity.fields.clone(),
                field_name.to_string(),
            )?;
            let field = fields
                .iter()
                .find(|field| field.name == field_name)
                .unwrap();

            if selection_field.selection_set().count() > 0 && field.as_type.is_none() {
                for selection_field in selection_field.selection_set().into_iter() {
                    // Call this function recursively to get the field guards.
                    let mut nested_field_guards =
                        ServiceResolver::get_field_guards(vec![selection_field], &entity)?;
                    field_guards.append(&mut nested_field_guards);
                }
            }
        }
        Ok(field_guards)
    }

    pub async fn execute_data_context(
        guard_data_contexts: Vec<GuardDataContext>,
        data_sources: &DataSources,
        subgraph_config: &SubGraphConfig,
        guard_context: &mut HashMapContext,
    ) -> Result<Value, async_graphql::Error> {
        debug!("Execute Data Context");
        let mut data_context = serde_json::json!({});
        for guard_data_context in guard_data_contexts {
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

            // for each key/value in each variable, parse and replace in query
            for variable in variables {
                let key = variable.0;
                let value = variable.1;

                let value = eval_with_context_mut(&value, guard_context)?;

                // If value is a tuple, map it to json array.
                // Else simply replace.
                if value.is_string() || value.is_number() {
                    let str_value = value.as_string();
                    if str_value.is_err() {
                        return Err(async_graphql::Error::new(
                            "Can't parse the guard data context query. Failed to parse string/int."
                                .to_string(),
                        ));
                    }
                    query = query.replace(&key, &str_value.unwrap());
                }
                if value.is_tuple() {
                    debug!("Value is tuple");
                    let mut json_array = serde_json::json!([]);
                    // For each tuple, we need to push it into the json array as an object.
                    for tuple in value.as_tuple().unwrap() {
                        let mut json_object = serde_json::json!({});
                        let tuple_value = tuple.as_string();

                        if tuple_value.is_err() {
                            return Err(async_graphql::Error::new(
                                "Can't parse the guard data context query. Failed to parse tuple."
                                    .to_string(),
                            ));
                        }
                        let tuple_value = tuple_value.unwrap();

                        json_object[key.clone().replace("{{", "").replace("}}", "")] =
                            serde_json::json!(tuple_value);
                        json_array.as_array_mut().unwrap().push(json_object);
                    }
                    query = query.replace(&key, &json_array.to_string());
                }
            }

            let json: Value = match serde_json::from_str(&query) {
                Ok(json) => json,
                Err(error) => {
                    return Err(async_graphql::Error::new(
                        "Can't parse the guard data context query. Failed to parse JSON."
                            .to_string(),
                    )
                    .extend_with(|_, e| e.set("error", error.to_string())))
                }
            };

            let input_document = to_document(&json).map_err(|error| {
                async_graphql::Error::new(
                    "Can't parse the guard data context query. Failed to convert to document."
                        .to_string(),
                )
                .extend_with(|_, e| e.set("error", error.to_string()))
            })?;

            let input_query = input_document.get(&input_name);

            if input_query.is_none() {
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

            debug!("Input Document: {:?}", input_document);

            //Execute the operation to get the data.
            let results = DataSources::execute(
                &data_sources,
                input_query_document.unwrap().to_owned(),
                entity,
                ResolverType::FindMany,
            )
            .await?;

            if results.is_none() {
                return Ok(data_context);
            }

            let results = results.unwrap();

            let results = results.try_to_list();

            if results.is_err() {
                error!(
                    "Failed to get results from data context query. Error: {:?}",
                    results.err()
                );
                return Err(async_graphql::Error::new(
                    "Can't parse the guard data context query. Failed to get results.".to_string(),
                ));
            }

            let results_list = results.unwrap();

            // iterate and turn all to json
            let mut results_json = serde_json::json!([]);
            for result in results_list {
                let mut result = result.try_downcast_ref::<Option<Document>>();
                if result.is_err() {
                    // result = result.try_downcast_ref::<Option<ResponseRow>>();
                    error!(
                        "Failed to get result from data context query. Error: {:?}",
                        result.err()
                    );
                    return Err(async_graphql::Error::new(
                        "Can't parse the guard data context query. Failed to get result."
                            .to_string(),
                    ));
                }
                let result = result.unwrap().to_owned();
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
                    .push(serde_json::json!(json));
            }
            data_context[guard_data_context.entity_name.clone()] = results_json;
        }
        println!("Data Context: {:?}", data_context);
        Ok(data_context)
    }
}
