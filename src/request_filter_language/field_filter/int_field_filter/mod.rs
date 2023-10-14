use super::{GroupName, IntFilterByOptions, OperatorOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IntFieldFilter {
    pub int: i32,
    #[serde(rename = "filterBy")]
    pub filter_by: Option<IntFilterByOptions>,
    pub operator: Option<OperatorOptions>,
    pub groups: Option<Vec<GroupName>>,
}
