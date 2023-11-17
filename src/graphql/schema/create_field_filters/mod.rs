use std::fmt::{Display, Formatter};

use async_graphql::dynamic::{Enum, EnumItem, InputObject, InputValue, TypeRef};
use log::debug;

use super::ServiceSchemaBuilder;

pub enum OperatorTypes {
    OperatorFieldConfigEnum,
}

impl Display for OperatorTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorTypes::OperatorFieldConfigEnum => write!(f, "OperatorFilterByEnum"),
        }
    }
}

pub enum OperatorFieldConfigEnum {
    AND,
    OR,
}

impl Display for OperatorFieldConfigEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorFieldConfigEnum::AND => write!(f, "AND"),
            OperatorFieldConfigEnum::OR => write!(f, "OR"),
        }
    }
}

pub enum FieldFilter {
    StringFieldFilter,
}

impl Display for FieldFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldFilter::StringFieldFilter => write!(f, "StringFieldFilter"),
        }
    }
}

pub enum FilterByType {
    StringFilterByEnum,
}

impl Display for FilterByType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterByType::StringFilterByEnum => write!(f, "StringFilterByEnum"),
        }
    }
}

pub enum FilterByEnum {
    MATCH,
    REGEX,
    OBJECTID,
}

impl Display for FilterByEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterByEnum::MATCH => write!(f, "MATCH"),
            FilterByEnum::REGEX => write!(f, "REGEX"),
            FilterByEnum::OBJECTID => write!(f, "OBJECTID"),
        }
    }
}

impl ServiceSchemaBuilder {
    pub fn create_field_filters(mut self) -> Self {
        debug!("Creating Field Filters");

        let string_filter_by_enum = Enum::new(FilterByType::StringFilterByEnum.to_string())
            .item(EnumItem::new(FilterByEnum::MATCH.to_string()))
            .item(EnumItem::new(FilterByEnum::REGEX.to_string()))
            .item(EnumItem::new(FilterByEnum::OBJECTID.to_string()));
        let operator_field_config_enum =
            Enum::new(OperatorTypes::OperatorFieldConfigEnum.to_string())
                .item(EnumItem::new(OperatorFieldConfigEnum::AND.to_string()))
                .item(EnumItem::new(OperatorFieldConfigEnum::OR.to_string()));

        let string_field_filter = InputObject::new(FieldFilter::StringFieldFilter.to_string())
            .field(InputValue::new(
                "string",
                TypeRef::named_nn(TypeRef::STRING),
            ))
            .field(InputValue::new(
                "filterBy",
                TypeRef::named(FilterByType::StringFilterByEnum.to_string()),
            ))
            .field(InputValue::new(
                "operator",
                TypeRef::named(OperatorTypes::OperatorFieldConfigEnum.to_string()),
            ))
            .field(InputValue::new(
                "groups",
                TypeRef::named_nn_list(TypeRef::STRING),
            ));

        self = self.register_enums(vec![string_filter_by_enum, operator_field_config_enum]);
        self = self.register_inputs(vec![string_field_filter]);
        self
    }
}
