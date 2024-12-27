use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FlashcardDto {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub sub_description: String,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

impl FlashcardDto {
    pub fn new() -> FlashcardDto {
        FlashcardDto {
            name: "".to_string(),
            description: "".to_string(),
            sub_description: "".to_string(),
            id: 0,
            created_date: Utc::now(),
            updated_date: Utc::now(),
        }
    }
}
