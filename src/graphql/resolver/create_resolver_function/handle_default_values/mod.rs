use bson::Document;
use evalexpr::{eval_with_context_mut, HashMapContext};
use log::debug;

use crate::{
    configuration::subgraph::entities::ServiceEntityConfig,
    graphql::{resolver::ServiceResolver, schema::ResolverType},
};

impl ServiceResolver {
    pub fn handle_default_values(
        input: &Document,
        entity: &ServiceEntityConfig,
        resolver_type: &ResolverType,
        mut guard_context: HashMapContext,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Handling Default Values");
        let mut input = input.clone();

        if resolver_type != &ResolverType::CreateOne {
            return Ok(input.clone());
        }

        // If input has field `values`, remove the values.
        if input.contains_key("values") {
            let fields = entity.fields.clone();
            let values = input.get_document("values");
            if values.is_ok() {
                let mut values = values.unwrap().clone();
                for field in fields {
                    if field.default_value.is_some() {
                        let default_value_expr = field.default_value.unwrap();
                        let default_value =
                            eval_with_context_mut(&default_value_expr, &mut guard_context);
                        if default_value.is_err() {
                            return Err(async_graphql::Error::new(format!(
                                "Invalid Default Value Expression: {}",
                                default_value_expr
                            )));
                        }

                        values.remove(&field.name);

                        let default_value = default_value.unwrap();

                        let default_value = if default_value.is_tuple() {
                            let default_value = default_value.as_tuple().unwrap();
                            if default_value.len() > 1 {
                                return Err(async_graphql::Error::new(format!(
                                    "Invalid Default Value Expression, results in more than one result: {}",
                                    default_value_expr
                                )));
                            }
                            default_value[0].clone()
                        } else {
                            default_value
                        };

                        if default_value.is_string() {
                            let default_value = default_value.as_string().unwrap();
                            values.insert(&field.name, default_value);
                            continue;
                        }

                        if default_value.is_int() {
                            let default_value = default_value.as_int().unwrap();
                            values.insert(&field.name, default_value);
                            continue;
                        }

                        if default_value.is_boolean() {
                            let default_value = default_value.as_boolean().unwrap();
                            values.insert(&field.name, default_value);
                            continue;
                        }

                        if default_value.is_float() {
                            let default_value = default_value.as_float().unwrap();
                            values.insert(&field.name, default_value);
                            continue;
                        }

                        if default_value.is_number() {
                            let default_value = default_value.as_number().unwrap();
                            values.insert(&field.name, default_value);
                            continue;
                        }
                    }
                }
                input.insert("values", values.clone());
                debug!("Default Values Inserted: {:?}", input);
                return Ok(input.clone());
            }
        }

        debug!("Default Values Inserted: {:?}", input);
        Ok(input.clone())
    }
}
