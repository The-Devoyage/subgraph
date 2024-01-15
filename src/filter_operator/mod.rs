use log::{debug, trace};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FilterOperator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}

impl FilterOperator {
    pub fn as_str(&self) -> &str {
        match self {
            FilterOperator::And => "AND",
            FilterOperator::Or => "OR",
        }
    }

    /// Returns a list of all available filter operators.
    pub fn list() -> Vec<FilterOperator> {
        debug!("Listing Available Filter Operators");
        let list = vec![FilterOperator::And, FilterOperator::Or];
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
            _ => None,
        };
        trace!("Filter Operator: {:?}", filter_operator);
        filter_operator
    }
}
