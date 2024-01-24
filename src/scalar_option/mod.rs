use serde::{Deserialize, Serialize};

pub mod to_async_graphql_value;
pub mod to_bson_element_type;
pub mod to_evalexpr;
pub mod to_serde_json_value;
pub mod to_sql_value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScalarOption {
    String,
    Int,
    Boolean,
    ObjectID,
    Object,
    UUID,
    DateTime,
}
