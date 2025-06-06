use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct FlashcardFileModel {
    pub id: i32,
    pub name: Option<String>,
    pub file_name: String,
    pub content_type: String,
    pub data: Vec<u8>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub created_by_id: Option<i32>,
    pub updated_by_id: Option<i32>,
}
