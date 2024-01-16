use async_graphql::dynamic::TypeRef;
use log::{debug, trace};
use serde::{Deserialize, Serialize};

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
}
