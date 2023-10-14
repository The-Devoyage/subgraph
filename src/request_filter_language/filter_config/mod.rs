use serde::{Deserialize, Serialize};

use self::{history::HistoryFilterInput, pagination::Pagination};

pub mod history;
pub mod pagination;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    pub pagination: Option<Pagination>,
    pub history: Option<HistoryFilterInput>,
}
