use std::fmt::Display;

use async_graphql::dynamic::{Enum, EnumItem, InputObject, InputValue, TypeRef};
use serde::{Deserialize, Serialize};

use crate::graphql::schema::ServiceSchema;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum DirectionEnum {
    Asc,
    Desc,
}

//Implement disaplay
impl Display for DirectionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionEnum::Asc => write!(f, "Asc"),
            DirectionEnum::Desc => write!(f, "Desc"),
        }
    }
}

impl std::str::FromStr for DirectionEnum {
    type Err = async_graphql::Error;

    fn from_str(input: &str) -> Result<DirectionEnum, Self::Err> {
        match input {
            "ASC" | "Asc" => Ok(DirectionEnum::Asc),
            "Desc" | "DESC" => Ok(DirectionEnum::Desc),
            _ => Err(async_graphql::Error::new(format!(
                "Invalid DirectionEnum: {}",
                input
            ))),
        }
    }
}

// Accept ASC Asc DESC Desc - Deserializer for DirectionEnum
impl<'de> serde::Deserialize<'de> for DirectionEnum {
    fn deserialize<D>(deserializer: D) -> Result<DirectionEnum, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "ASC" | "Asc" => Ok(DirectionEnum::Asc),
            "Desc" | "DESC" => Ok(DirectionEnum::Desc),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid DirectionEnum: {}",
                s
            ))),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SortInput {
    pub field: String,
    pub direction: DirectionEnum,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OptionsInput {
    pub per_page: Option<i32>,
    pub page: Option<i32>,
    pub sort: Option<Vec<SortInput>>,
}

impl ServiceSchema {
    pub fn create_options_input(mut self) -> Self {
        // Create the sort/order input list
        let mut sort_input = InputObject::new("sort_input");
        sort_input = sort_input.field(InputValue::new("field", TypeRef::named(TypeRef::STRING)));
        sort_input = sort_input.field(InputValue::new(
            "direction",
            TypeRef::named("sort_direction"),
        ));
        self = self.register_inputs(vec![sort_input]);

        // Create shared input, `options_input`
        let mut root_input = InputObject::new("options_input");
        root_input = root_input.field(InputValue::new("per_page", TypeRef::named(TypeRef::INT)));
        root_input = root_input.field(InputValue::new("page", TypeRef::named(TypeRef::INT)));
        root_input = root_input.field(InputValue::new(
            "sort",
            TypeRef::named_nn_list("sort_input"),
        ));
        self = self.register_inputs(vec![root_input]);

        // Create the order enum
        let mut sort_direction = Enum::new("sort_direction");
        sort_direction = sort_direction.items(vec![EnumItem::new("ASC"), EnumItem::new("DESC")]);
        self = self.register_enums(vec![sort_direction]);

        self
    }
}
