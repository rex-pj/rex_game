use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct FlashcardModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub sub_description: Option<String>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub file_id: i32,
    pub created_by_id: Option<i32>,
    pub updated_by_id: Option<i32>,
}
