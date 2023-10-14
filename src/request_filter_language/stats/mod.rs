use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stats {
    pub remaing: i32,
    pub total: i32,
    pub page: i32,
    //TODO: Ensure datetime.
    pub prev_cursor: String,
    pub cursor: String,
    pub history: Vec<HistoricStats>,
    pub per_page: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoricStats {
    _id: HistoricStatsId,
    total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoricStatsId {
    #[serde(rename = "YEAR")]
    year: i32,
    #[serde(rename = "DAY_OF_YEAR")]
    day_of_year: i32,
    #[serde(rename = "MONTH")]
    month: i32,
    #[serde(rename = "DAY_OF_MONTH")]
    day_of_month: i32,
    #[serde(rename = "WEEK")]
    week: i32,
    #[serde(rename = "DAY_OF_WEEK")]
    day_of_week: i32,
    #[serde(rename = "HOUR")]
    hour: i32,
    #[serde(rename = "MINUTE")]
    minute: i32,
    #[serde(rename = "SECOND")]
    second: i32,
}
