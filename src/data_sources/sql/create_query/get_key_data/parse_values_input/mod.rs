use bson::Bson;
use log::{debug, error};

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum,
        entities::{service_entity_field::ServiceEntityFieldConfig, ServiceEntityConfig},
    },
    data_sources::sql::{SqlDataSource, SqlValueEnum},
    resolver_type::ResolverType,
};

impl SqlDataSource {
    /// Converts the `input.values` struct provided by the client.
    pub fn parse_values_input(
        value: &Bson,
        mut where_keys: Vec<String>,
        mut where_values: Vec<SqlValueEnum>,
        mut value_keys: Vec<String>,
        mut values: Vec<SqlValueEnum>,
        entity: &ServiceEntityConfig,
        resolver_type: &ResolverType,
        dialect: &DialectEnum,
    ) -> Result<
        (
            Vec<String>,
            Vec<SqlValueEnum>,
            Vec<String>,
            Vec<SqlValueEnum>,
        ),
        async_graphql::Error,
    > {
        debug!("Parsing Values Input: {:?}", value);
        let values_object = value.as_document();

        if values_object.is_none() {
            return Err(async_graphql::Error::new("Invalid Values Object"));
        }

        for (key, value) in values_object.unwrap().iter() {
            debug!("Processing Key: {:?}", key);
            debug!("Processing Value: {:?}", value.to_string());

            //If value == null, skip
            if value.as_null().is_some() {
                continue;
            }

            let field = ServiceEntityConfig::get_field(entity.clone(), key.to_string());

            if field.is_err() {
                error!("Field {} does not exist on entity {}", key, entity.name);
                return Err(async_graphql::Error::new(format!(
                    "Field {} does not exist on entity {}",
                    key, entity.name
                )));
            }

            //NOTE: Since separating logic, it may not be needed to specify this variable.
            let is_where_clause = match resolver_type {
                ResolverType::FindOne | ResolverType::FindMany => true,
                ResolverType::CreateOne | ResolverType::UpdateOne | ResolverType::UpdateMany => {
                    false
                }
                _ => {
                    error!("Resolver type {:?} is not supported", resolver_type);
                    return Err(async_graphql::Error::new(format!(
                        "Resolver type {:?} is not supported",
                        resolver_type
                    )));
                }
            };
            let ServiceEntityFieldConfig { scalar, .. } = field.unwrap();

            //TODO: Deal with the custom dialect enum stuff below.
            let sql_value_enum = scalar.to_sql_value_enum(value, Some(dialect))?;

            if is_where_clause {
                where_keys.push(key.to_string());
                where_values.push(sql_value_enum);
            } else {
                value_keys.push(key.to_string());
                values.push(sql_value_enum);
            }
        }

        debug!("Where Keys: {:?}", where_keys);
        debug!("Where Values: {:?}", where_values);
        debug!("Value Keys: {:?}", value_keys);
        debug!("Values: {:?}", values);

        Ok((where_keys, where_values, value_keys, values))
    }
}
