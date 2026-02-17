use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct FlashcardTypeModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub created_by_id: Option<i32>,
    pub updated_by_id: Option<i32>,
    pub is_actived: bool,
}
