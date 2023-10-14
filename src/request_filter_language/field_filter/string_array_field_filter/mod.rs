use serde::{Deserialize, Serialize};

use super::{ArrayFilterByOptions, GroupName, OperatorOptions, StringFilterByOptions};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StringArrayFieldFilter {
    pub strings: Vec<String>,
    #[serde(rename = "filterBy")]
    pub filter_by: Option<StringFilterByOptions>,
    pub operator: Option<OperatorOptions>,
    pub groups: Option<Vec<GroupName>>,
    #[serde(rename = "arrayOptions")]
    pub array_options: ArrayFilterByOptions,
}
