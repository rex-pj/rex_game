use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PageListDto<T> {
    pub items: Vec<T>,
    pub page_size: u64,
    pub page: u64,
    pub total_count: u64,
}

impl<T> PageListDto<T> {
    pub fn new(items: Vec<T>, page_size: u64, page: u64, total_count: u64) -> Self {
        Self {
            items,
            page_size,
            page,
            total_count,
        }
    }
}
