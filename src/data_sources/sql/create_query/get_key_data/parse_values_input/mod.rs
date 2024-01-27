use bson::Bson;
use log::{debug, error, trace};

use crate::{
    configuration::subgraph::{
        data_sources::sql::DialectEnum,
        entities::{service_entity_field::ServiceEntityFieldConfig, ServiceEntityConfig},
    },
    data_sources::sql::SqlDataSource,
    resolver_type::ResolverType,
    sql_value::SqlValue,
};

impl SqlDataSource {
    /// Converts the `input.values` struct provided by the client.
    pub fn parse_values_input(
        value: &Bson,
        mut where_keys: Vec<String>,
        mut where_values: Vec<SqlValue>,
        mut value_keys: Vec<String>,
        mut values: Vec<SqlValue>,
        entity: &ServiceEntityConfig,
        resolver_type: &ResolverType,
        dialect: &DialectEnum,
    ) -> Result<(Vec<String>, Vec<SqlValue>, Vec<String>, Vec<SqlValue>), async_graphql::Error>
    {
        debug!("Parsing Values Input: {:?}", value);
        let values_object = value.as_document();

        if values_object.is_none() {
            return Err(async_graphql::Error::new("Invalid Values Object"));
        }

        for (key, value) in values_object.unwrap().iter() {
            trace!("Processing Key: {:?}", key);
            trace!("Processing Value: {:?}", value.to_string());

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

            let sql_value_enum = scalar.bson_to_sql_value(value, Some(dialect))?;

            if is_where_clause {
                where_keys.push(key.to_string());
                where_values.push(sql_value_enum);
            } else {
                value_keys.push(key.to_string());
                values.push(sql_value_enum);
            }
        }

        trace!("Where Keys: {:?}", where_keys);
        trace!("Where Values: {:?}", where_values);
        trace!("Value Keys: {:?}", value_keys);
        trace!("Values: {:?}", values);

        Ok((where_keys, where_values, value_keys, values))
    }
}
