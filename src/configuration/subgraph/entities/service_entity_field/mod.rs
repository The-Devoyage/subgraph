use log::debug;
use serde::{Deserialize, Serialize};

use crate::{configuration::subgraph::guard::Guard, graphql::schema::ResolverType};

use super::ScalarOptions;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceEntityField {
    pub name: String,
    pub guards: Option<Vec<Guard>>,
    pub scalar: ScalarOptions,
    pub required: Option<bool>,
    pub exclude_from_input: Option<Vec<ResolverType>>,
    pub exclude_from_output: Option<bool>,
    pub fields: Option<Vec<ServiceEntityField>>,
    pub list: Option<bool>,
}

impl ServiceEntityField {
    /// Get a field from a list of fields.
    /// This function will recursively search for a field in a list of fields.
    /// Ex: `user.name` will search for `user` and then `name` in the `user` field.
    pub fn get_field(
        fields: Vec<ServiceEntityField>,
        field_name: String,
    ) -> Result<ServiceEntityField, async_graphql::Error> {
        debug!("Get Field From Fields: {:?}", field_name);
        if field_name.contains(".") {
            debug!("Field is Nested");
            let mut field_names = ServiceEntityField::split_field_names(&field_name)?;
            let first_field =
                ServiceEntityField::get_field(fields.clone(), field_names[0].to_string())?;
            let nested_fields = first_field.fields;
            if nested_fields.is_none() {
                return Err(async_graphql::Error::new(format!(
                    "Field {} is not a nested field",
                    field_name
                )));
            }
            field_names.remove(0);
            let field =
                ServiceEntityField::get_field(nested_fields.unwrap(), field_names.join("."))?;
            debug!("Found Field: {:?}", field);
            return Ok(field);
        } else {
            for field in fields {
                if field.name == field_name {
                    debug!("Found Field: {:?}", field);
                    return Ok(field);
                }
            }
            Err(async_graphql::Error::new(format!(
                "Field {} not found",
                field_name
            )))
        }
    }

    /// Get fields from a list of fields.
    pub fn get_fields_recursive(
        fields: Vec<ServiceEntityField>,
        field_name: String,
    ) -> Result<Vec<ServiceEntityField>, async_graphql::Error> {
        debug!("Get Field From Fields: {:?}", field_name);
        if field_name.contains(".") {
            debug!("Field is Nested");
            let mut field_names = ServiceEntityField::split_field_names(&field_name)?;
            let first_field =
                ServiceEntityField::get_field(fields.clone(), field_names[0].to_string())?;
            let nested_fields = first_field.fields;
            if nested_fields.is_none() {
                return Err(async_graphql::Error::new(format!(
                    "Field {} is not a nested field",
                    field_name
                )));
            }
            field_names.remove(0);
            let fields = ServiceEntityField::get_fields_recursive(
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
}
