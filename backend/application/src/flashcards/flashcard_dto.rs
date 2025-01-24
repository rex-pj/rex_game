use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FlashcardDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub sub_description: Option<String>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub image_id: i32,
}
