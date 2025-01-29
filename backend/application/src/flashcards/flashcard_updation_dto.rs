use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct FlashcardUpdationDto {
    pub name: Option<String>,
    pub file_name: Option<String>,
    pub content_type: Option<String>,
    pub description: Option<String>,
    pub sub_description: Option<String>,
    pub image_data: Option<Vec<u8>>,
    pub type_ids: Option<Vec<i32>>,
}
