use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageListModel<T> {
    pub items: Vec<T>,
    pub total_count: u64,
}
