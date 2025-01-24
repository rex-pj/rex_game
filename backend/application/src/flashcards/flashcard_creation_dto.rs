use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FlashcardCreationDto {
    pub name: String,
    pub file_name: String,
    pub content_type: String,
    pub description: Option<String>,
    pub sub_description: Option<String>,
    pub image_data: Vec<u8>,
}
