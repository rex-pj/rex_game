use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::flashcard::application::usecases::flashcard_type_dto::FlashcardTypeDto;

#[derive(Serialize, Deserialize)]
pub struct FlashcardDetailDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub sub_description: Option<String>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub image_id: i32,
    pub flashcard_types: Vec<FlashcardTypeDto>,
}
