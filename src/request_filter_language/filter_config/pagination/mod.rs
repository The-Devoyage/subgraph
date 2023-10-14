use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    limit: Option<i32>,
    reverse: Option<bool>,
    //TODO: Make sure this is a date
    date_cursor: Option<String>,
    date_key: Option<String>,
}
