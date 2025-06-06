use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct FlashcardTypeRelationModel {
    pub id: i32,
    pub flashcard_id: i32,
    pub flashcard_type_id: i32,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub created_by_id: Option<i32>,
    pub updated_by_id: Option<i32>,
}
