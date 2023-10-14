use serde::{Deserialize, Serialize};

pub mod boolean_field_filter;
pub mod date_field_filter;
pub mod int_field_filter;
pub mod string_array_field_filter;
pub mod string_field_filter;

pub enum FieldFilter {
    StringFieldFilter(string_field_filter::StringFieldFilter),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum StringFilterByOptions {
    #[serde(rename = "MATCH")]
    Match,
    #[serde(rename = "OBJECTID")]
    ObjectId,
    #[serde(rename = "REGEX")]
    Regex,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum IntFilterByOptions {
    GT,
    GTE,
    LT,
    LTE,
    EQ,
    NE,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum BooleanFilterByOptions {
    EQ,
    NE,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DateFilterByOptions {
    GT,
    GTE,
    LT,
    LTE,
    EQ,
    NE,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum OperatorOptions {
    AND,
    OR,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum GroupName {
    And(String),
    Or(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ArrayFilterByOptions {
    IN,
    NIN,
}
