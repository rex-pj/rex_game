use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FlashcardCreationDto {
    pub name: String,
    pub description: Option<String>,
    pub sub_description: String,
    pub image_data: Vec<u8>,
}
