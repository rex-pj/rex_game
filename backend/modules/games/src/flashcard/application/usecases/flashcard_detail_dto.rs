use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::flashcard::application::usecases::flashcard_type_dto::FlashcardTypeDto;

#[derive(Serialize, Deserialize)]
pub struct FlashcardDetailDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub sub_description: Option<String>,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub image_id: i32,
    pub flashcard_types: Vec<FlashcardTypeDto>,
    pub game_types: Vec<FlashcardGameTypeInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct FlashcardGameTypeInfo {
    pub id: i32,
    pub code: String,
    pub name: String,
}
