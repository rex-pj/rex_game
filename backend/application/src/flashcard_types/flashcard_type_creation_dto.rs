use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FlashcardTypeCreationDto {
    pub name: String,
    pub description: Option<String>,
}
