use bson::Bson;
use log::{debug, error, trace};

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum,
        entities::{service_entity_field::ServiceEntityFieldConfig, ServiceEntityConfig},
        SubGraphConfig,
    },
    data_sources::sql::{create_query::JoinClauses, SqlDataSource, SqlValueEnum},
};

mod get_query_where_values;

impl SqlDataSource {
    /// Creates vectors of keys and values parsed from the user provided input.
    pub fn parse_query_input(
        value: &Bson,
        mut where_keys: Vec<String>,
        mut where_values: Vec<SqlValueEnum>,
        dialect: &DialectEnum,
        entity: &ServiceEntityConfig,
        subgraph_config: &SubGraphConfig,
    ) -> Result<(Vec<String>, Vec<SqlValueEnum>, JoinClauses), async_graphql::Error> {
        debug!("Parse Query Input");
        trace!("Input: {:?}", value);
        let query_object = value.as_document();
        let mut join_clauses = JoinClauses(Vec::new());

        if query_object.is_none() {
            error!("Invalid Query Object: {:?}", value);
            return Err(async_graphql::Error::new("Invalid Query Object"));
        }

        let excluded_keys = vec!["OR".to_string(), "AND".to_string()];

        // Iterate through the query object and create a vector of keys and values
        for (key, value) in query_object.unwrap().iter() {
            if excluded_keys.contains(&key) {
                continue;
            }

            trace!("Parsing Query Key: {:?}", key);
            let field = ServiceEntityConfig::get_field(entity.clone(), key.to_string())?;

            // Get the where key prefix
            let where_key_prefix =
                SqlDataSource::get_where_key_prefix(&field, &entity, &subgraph_config)?;

            // Get the join clause and push it to the join clauses vector
            let join_clause = SqlDataSource::get_join_clause(&field, &entity, subgraph_config)?;
            if join_clause.is_some() {
                join_clauses.0.push(join_clause.unwrap());
            }

            // If the field is eager loaded, we can assume it is a object with many fields. Iterate
            // over the fields and return the keys.
            // Else, just return the key as is.
            if field.eager.is_some() {
                trace!("Parsing Eager Loaded Field");
                let eager_input = match value.as_document() {
                    Some(v) => Some(v),
                    None => {
                        continue;
                    }
                };
                let as_type = field.as_type;
                for (key, nested_value) in eager_input.unwrap().iter() {
                    let (wk, wv, jc) = SqlDataSource::get_query_where_values(
                        nested_value,
                        dialect,
                        key,
                        as_type.clone(),
                        subgraph_config,
                    )?;
                    for value in wv.into_iter() {
                        where_values.push(value);
                    }
                    for key in wk {
                        if key.contains(".") {
                            where_keys.push(key);
                            continue;
                        }
                        let where_key = format!("{}.{}", where_key_prefix, key.to_string());
                        where_keys.push(where_key);
                    }
                    for join_clause in jc.0 {
                        join_clauses.0.push(join_clause);
                    }
                }
            } else {
                let (parsed_where_keys, parsed_where_values, parsed_join_clauses) =
                    SqlDataSource::get_query_where_values(
                        value,
                        dialect,
                        key,
                        None,
                        subgraph_config,
                    )?;
                for key in parsed_where_keys {
                    let key = format!("{}.{}", where_key_prefix, key.to_string());
                    where_keys.push(key);
                }
                for value in parsed_where_values {
                    where_values.push(value);
                }
                for join_clause in parsed_join_clauses.0 {
                    join_clauses.0.push(join_clause);
                }
            }
        }

        trace!("Where Keys: {:?}", where_keys);
        trace!("Where Values: {:?}", where_values);
        trace!("Join Clauses: {:?}", join_clauses);

        Ok((where_keys, where_values, join_clauses))
    }

    pub fn get_join_clause(
        field: &ServiceEntityFieldConfig,
        parent_entity: &ServiceEntityConfig,
        subgraph_config: &SubGraphConfig,
    ) -> Result<Option<String>, async_graphql::Error> {
        debug!("Get Join Clause");
        trace!("Field: {:?}", field);
        trace!("Entity: {:?}", parent_entity);

        let join_clause = if field.eager.is_some() {
            if field.as_type.is_none() {
                error!("As type required for eager loading: {:?}", field);
                return Err(async_graphql::Error::new(format!(
                    "As type required for eager loading: {:?}",
                    field,
                )));
            }
            let child_entity = match subgraph_config
                .clone()
                .get_entity(&field.as_type.clone().unwrap())
            {
                Some(entity) => entity,
                None => {
                    error!("Entity not found: {:?}", field.as_type.clone().unwrap());
                    return Err(async_graphql::Error::new(format!(
                        "Entity not found: {:?}",
                        field.as_type.clone().unwrap()
                    )));
                }
            };

            let table_name = if let Some(ds) = child_entity.data_source {
                if ds.table.is_some() {
                    ds.table.unwrap()
                } else {
                    child_entity.name.clone()
                }
            } else {
                child_entity.name.clone()
            };

            // Create the join clauses, to be used later.
            let join_clause = format!(
                " JOIN {} ON {}.{} = {}.{} ",
                table_name,
                table_name.clone(),
                field.join_on.clone().unwrap(),
                parent_entity.name.clone(),
                field.join_from.clone().unwrap_or(field.name.clone())
            );
            Some(join_clause)
        } else {
            None
        };

        trace!("Join Clause: {:?}", join_clause);
        Ok(join_clause)
    }

    /// Gets the prefix to be used in the where clause for each key.
    /// If database table is specified, use the table name as the prefix in the search
    /// query: SELECT * FROM todo WHERE todo_table.id = $1;
    pub fn get_where_key_prefix(
        field: &ServiceEntityFieldConfig,
        entity: &ServiceEntityConfig,
        subgraph_config: &SubGraphConfig,
    ) -> Result<String, async_graphql::Error> {
        debug!("Get Where Key Prefix");

        let mut where_key_prefix = if entity.data_source.is_some() {
            let ds = entity.data_source.clone().unwrap();
            if ds.table.is_some() {
                ds.table.unwrap()
            } else {
                entity.clone().name
            }
        } else {
            entity.clone().name
        };

        // If the field is a eager loaded field, get the correct prefix
        if field.eager.is_some() {
            if field.as_type.is_none() {
                error!("As type required for eager loading: {:?}", field);
                return Err(async_graphql::Error::new(format!(
                    "As type required for eager loading: {:?}",
                    field,
                )));
            }
            let child_entity = match subgraph_config
                .clone()
                .get_entity(&field.as_type.clone().unwrap())
            {
                Some(entity) => entity,
                None => {
                    error!("Entity not found: {:?}", field.as_type.clone().unwrap());
                    return Err(async_graphql::Error::new(format!(
                        "Entity not found: {:?}",
                        field.as_type.clone().unwrap()
                    )));
                }
            };
            let table_name = if let Some(ds) = child_entity.data_source {
                if ds.table.is_some() {
                    ds.table.unwrap()
                } else {
                    child_entity.name.clone()
                }
            } else {
                child_entity.name.clone()
            };
            where_key_prefix = table_name;
        }
        trace!("Where Key Prefix: {:?}", where_key_prefix);
        Ok(where_key_prefix)
    }
}
