use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FlashcardDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub sub_description: Option<String>,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub image_id: i32,
    pub is_actived: bool,
    #[serde(default)]
    pub flashcard_type_names: Vec<String>,
}
