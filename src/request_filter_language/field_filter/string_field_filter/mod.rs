use serde::{Deserialize, Serialize};

use super::{GroupName, OperatorOptions, StringFilterByOptions};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StringFieldFilter {
    pub string: String,
    #[serde(rename = "filterBy")]
    pub filter_by: Option<StringFilterByOptions>,
    pub operator: Option<OperatorOptions>,
    pub groups: Option<Vec<GroupName>>,
}
