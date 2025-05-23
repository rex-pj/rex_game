use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PageListDto<T> {
    pub items: Vec<T>,
    pub page_size: u64,
    pub page: u64,
    pub total_count: u64,
}
