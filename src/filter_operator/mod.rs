use async_graphql::dynamic::TypeRef;
use bson::{Document, Regex};
use log::{debug, trace};
use serde::{Deserialize, Serialize};

use crate::utils::clean_string::clean_string;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FilterOperator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
    #[serde(rename = "LIKE")]
    Like,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "GT")]
    Gt,
}

impl FilterOperator {
    pub fn as_str(&self) -> &str {
        match self {
            FilterOperator::And => "AND",
            FilterOperator::Or => "OR",
            FilterOperator::Like => "LIKE",
            FilterOperator::Lt => "LT",
            FilterOperator::Gt => "GT",
        }
    }

    /// Returns a list of all available filter operators.
    pub fn list() -> Vec<FilterOperator> {
        debug!("Listing Available Filter Operators");
        let list = vec![
            FilterOperator::And,
            FilterOperator::Or,
            FilterOperator::Like,
            FilterOperator::Lt,
            FilterOperator::Gt,
        ];
        trace!("{:?}", list);
        list
    }

    /// Get the filter operator from a string.
    pub fn from_str(s: &str) -> Option<FilterOperator> {
        debug!("Getting Filter Operator from String");
        trace!("String: {}", s);
        let filter_operator = match s {
            "AND" => Some(FilterOperator::And),
            "OR" => Some(FilterOperator::Or),
            "LIKE" => Some(FilterOperator::Like),
            "LT" => Some(FilterOperator::Lt),
            "GT" => Some(FilterOperator::Gt),
            "$and" => Some(FilterOperator::And),
            "$or" => Some(FilterOperator::Or),
            "$regex" => Some(FilterOperator::Like),
            "$lt" => Some(FilterOperator::Lt),
            "$gt" => Some(FilterOperator::Gt),
            _ => None,
        };
        trace!("Filter Operator: {:?}", filter_operator);
        filter_operator
    }

    /// Get the graphql typeref associated with the filter operator.
    pub fn get_graphql_typeref(&self, input_name: &str) -> TypeRef {
        debug!("Getting GraphQL TypeRef for Filter Operator");
        trace!("Filter Operator: {:?}", self);
        // TypeRef::named_nn_list(&self.input_name),
        let graphql_typedef = match self {
            FilterOperator::And | FilterOperator::Or => TypeRef::named_nn_list(input_name),
            FilterOperator::Like => TypeRef::named(input_name),
            FilterOperator::Lt => TypeRef::named(input_name),
            FilterOperator::Gt => TypeRef::named(input_name),
        };
        trace!("GraphQL TypeDef: {}", graphql_typedef);
        graphql_typedef
    }

    /// Get the SQL Operator based on the filter operator.
    pub fn get_sql_operator(&self) -> &str {
        debug!("Getting SQL Operator for Filter Operator");
        trace!("Filter Operator: {:?}", self);
        let sql_operator = match self {
            FilterOperator::Like => " LIKE ",
            FilterOperator::Lt => " < ",
            FilterOperator::Gt => " > ",
            _ => " = ",
        };
        trace!("SQL Operator: {}", sql_operator);
        sql_operator
    }

    /// Get the Mongo Operator based on the filter operator.
    pub fn get_mongo_operator(&self) -> &str {
        debug!("Getting Mongo Operator for Filter Operator");
        trace!("Filter Operator: {:?}", self);
        let mongo_operator = match self {
            FilterOperator::Like => "$regex",
            FilterOperator::Lt => "$lt",
            FilterOperator::Gt => "$gt",
            FilterOperator::And => "$and",
            FilterOperator::Or => "$or",
        };
        trace!("Mongo Operator: {}", mongo_operator);
        mongo_operator
    }

    pub fn list_mongo_operators() -> Vec<String> {
        debug!("Listing Mongo Operators");
        let filter_operators = FilterOperator::list();
        let mut list = Vec::new();
        for filter_operator in filter_operators {
            list.push(filter_operator.get_mongo_operator().to_string());
        }
        trace!("{:?}", list);
        list
    }

    /// Convert provided value to expected value for mongo filters.
    pub fn convert_value_to_mongo(&self, key: &str, value: &bson::Bson) -> Document {
        debug!("Converting Value to Mongo Filter Value");
        trace!("Filter Operator: {:?}", self);
        trace!("Value: {}", value);
        let mut filter = Document::new();
        match self {
            FilterOperator::Like => {
                let value = clean_string(&value.to_string(), None).to_string();
                let value = value.to_string().trim_start_matches("/").to_string();
                let values: Vec<String> = value
                    .split("/")
                    .map(|s| clean_string(&s.to_string(), None))
                    .collect();

                let pattern = clean_string(&values[0].to_string(), None).to_string();
                let options = values.get(1).unwrap_or(&"i".to_string()).to_string();

                let regex = Regex {
                    pattern: pattern.to_string(),
                    options: options.to_string(),
                };

                trace!("{:?}", regex);

                filter.insert(FilterOperator::Like.get_mongo_operator(), regex);
            }
            FilterOperator::Lt => {
                filter.insert(FilterOperator::Lt.get_mongo_operator(), value);
            }
            FilterOperator::Gt => {
                filter.insert(FilterOperator::Gt.get_mongo_operator(), value);
            }
            _ => {
                filter.insert(key, value);
            }
        }
        trace!("Mongo Filter: {:?}", filter);
        filter
    }
}
