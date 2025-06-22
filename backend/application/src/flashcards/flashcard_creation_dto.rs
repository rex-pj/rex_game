use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct FlashcardCreationDto {
    pub name: String,
    pub file_name: String,
    pub content_type: String,
    pub description: Option<String>,
    pub sub_description: Option<String>,
    pub image_data: Option<Vec<u8>>,
    pub type_ids: Vec<i32>,
    pub created_by_id: i32,
    pub updated_by_id: i32,
}
