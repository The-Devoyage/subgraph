use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::{configuration::subgraph::guard::Guard, graphql::schema::ExcludeFromInput};

use super::ScalarOptions;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityFieldConfig {
    pub name: String,
    pub guards: Option<Vec<Guard>>,
    pub scalar: ScalarOptions,
    pub required: Option<bool>,
    pub exclude_from_input: Option<Vec<ExcludeFromInput>>,
    pub exclude_from_output: Option<bool>,
    pub fields: Option<Vec<ServiceEntityFieldConfig>>,
    pub list: Option<bool>,
    pub as_type: Option<String>,
    pub join_on: Option<String>,
    pub join_from: Option<String>,
    pub default_value: Option<String>,
    pub eager: Option<bool>,
    pub is_virtual: Option<bool>,
    pub primary_key: Option<bool>,
}

impl ServiceEntityFieldConfig {
    /// Get a field from a list of fields.
    /// This function will recursively search for a field in a list of fields.
    /// Ex: `user.name` will search for `user` and then `name` in the `user` field.
    pub fn get_field(
        fields: Vec<ServiceEntityFieldConfig>,
        field_name: String,
    ) -> Result<ServiceEntityFieldConfig, async_graphql::Error> {
        debug!("Get Field From Fields: {:?} in {:?}", field_name, fields);
        if field_name.contains(".") {
            debug!("Field is Nested");
            let mut field_names = ServiceEntityFieldConfig::split_field_names(&field_name)?;
            let first_field =
                ServiceEntityFieldConfig::get_field(fields.clone(), field_names[0].to_string())?;
            let nested_fields = first_field.fields;
            if nested_fields.is_none() {
                return Err(async_graphql::Error::new(format!(
                    "Field {} is not a nested field",
                    field_name
                )));
            }
            field_names.remove(0);
            let field =
                ServiceEntityFieldConfig::get_field(nested_fields.unwrap(), field_names.join("."))?;
            debug!("Found Field: {:?}", field);
            return Ok(field);
        } else {
            for field in fields {
                if field.name == field_name {
                    debug!("Found Field: {:?}", field);
                    return Ok(field);
                }
            }
            error!("Field {} not found when executing get_field.", field_name);
            Err(async_graphql::Error::new(format!(
                "Field {} not found",
                field_name
            )))
        }
    }

    /// Get fields from a list of fields.
    pub fn get_fields_recursive(
        fields: Vec<ServiceEntityFieldConfig>,
        field_name: String,
    ) -> Result<Vec<ServiceEntityFieldConfig>, async_graphql::Error> {
        debug!("Get Field From Fields: {:?}", field_name);
        if field_name.contains(".") {
            debug!("Field is Nested");
            let mut field_names = ServiceEntityFieldConfig::split_field_names(&field_name)?;
            let first_field =
                ServiceEntityFieldConfig::get_field(fields.clone(), field_names[0].to_string())?;
            let nested_fields = first_field.fields;
            if nested_fields.is_none() {
                return Err(async_graphql::Error::new(format!(
                    "Field {} is not a nested field",
                    field_name
                )));
            }
            field_names.remove(0);
            let fields = ServiceEntityFieldConfig::get_fields_recursive(
                nested_fields.unwrap(),
                field_names.join("."),
            )?;
            debug!("Found Fields: {:?}", fields);
            return Ok(fields);
        } else {
            for field in fields {
                if field.name == field_name {
                    debug!("Found Field: {:?}", field);
                    return Ok(vec![field]);
                }
            }
            Err(async_graphql::Error::new(format!(
                "Field {} not found",
                field_name
            )))
        }
    }

    pub fn get_guards(field: ServiceEntityFieldConfig) -> Option<Vec<Guard>> {
        debug!("Get Guards From Field: {:?}", field);
        let field_guards = field.guards.clone();
        if field_guards.is_none() {
            return None;
        }
        Some(field.guards.unwrap())
    }

    /// Split field names into a vector of field names.
    /// Ex: `user.name` will return `["user", "name"]`
    pub fn split_field_names(field_name: &str) -> Result<Vec<&str>, async_graphql::Error> {
        let field_names: Vec<&str> = field_name.split(".").collect();
        if field_names.len() < 2 {
            return Err(async_graphql::Error::new(format!(
                "Field name must be nested. Ex: user.name"
            )));
        }
        Ok(field_names)
    }

    pub fn is_excluded_input_field(
        entity_field: &ServiceEntityFieldConfig,
        excluded: Option<ExcludeFromInput>,
    ) -> bool {
        debug!("Validate Exclude From Input");
        let mut is_excluded = false;
        let exclude_from_input = entity_field.exclude_from_input.clone();

        if exclude_from_input.is_some() {
            is_excluded = match exclude_from_input {
                Some(exclude_from_input) => {
                    if exclude_from_input.contains(&ExcludeFromInput::All) {
                        return true;
                    }
                    if exclude_from_input.contains(&excluded.unwrap()) {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            };
        }

        debug!("Is Excluded: {:?}", is_excluded);
        is_excluded
    }
}
