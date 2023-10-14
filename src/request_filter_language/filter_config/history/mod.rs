use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HistoryFilterIntervalEnum {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "DAY_OF_YEAR")]
    DayOfYear,
    #[serde(rename = "MONTH")]
    Month,
    #[serde(rename = "DAY_OF_MONTH")]
    DayOfMonth,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "DAY_OF_WEEK")]
    DayOfWeek,
    #[serde(rename = "HOUR")]
    Hour,
    #[serde(rename = "MINUTE")]
    Minute,
    #[serde(rename = "SECOND")]
    Second,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryFilterInput {
    pub interval: Vec<HistoryFilterIntervalEnum>,
    pub interval_key: String,
}
