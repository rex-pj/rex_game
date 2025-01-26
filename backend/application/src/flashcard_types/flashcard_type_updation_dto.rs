use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FlashcardTypeUpdationDto {
    pub name: String,
    pub description: Option<String>,
}
