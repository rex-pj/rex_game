use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FlashcardTypeCreationDto {
    pub name: String,
    pub description: Option<String>,
    pub created_by_id: i32,
    pub updated_by_id: i32,
}
