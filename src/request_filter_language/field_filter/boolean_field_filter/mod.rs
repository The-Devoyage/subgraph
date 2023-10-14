use serde::{Deserialize, Serialize};

use super::{BooleanFilterByOptions, GroupName, OperatorOptions};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BooleanFieldFilter {
    pub boolean: bool,
    #[serde(rename = "filterBy")]
    pub filter_by: Option<BooleanFilterByOptions>,
    pub operator: Option<OperatorOptions>,
    pub groups: Option<Vec<GroupName>>,
}
