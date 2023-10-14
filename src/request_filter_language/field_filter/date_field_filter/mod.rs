use serde::{Deserialize, Serialize};

use super::{DateFilterByOptions, GroupName, OperatorOptions};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DateFieldFilter {
    //TODO: Make sure this is a date
    pub date: String,
    #[serde(rename = "filterBy")]
    pub filter_by: Option<DateFilterByOptions>,
    pub operator: Option<OperatorOptions>,
    pub groups: Option<Vec<GroupName>>,
}
