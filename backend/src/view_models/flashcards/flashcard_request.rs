use crate::validators::{validate_content_type, validate_file_content, validate_file_size};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default)]
pub struct FlashcardRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 1,
        max = 255,
        message = "File name must be between 1 and 255 characters"
    ))]
    pub file_name: String,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Content type must be between 1 and 255 characters"
    ))]
    #[validate(custom(
        function = "validate_content_type",
        message = "Content-Type must be image/jpeg, image/png, image/gif."
    ))]
    pub content_type: String,
    pub description: Option<String>,
    pub sub_description: Option<String>,
    pub type_ids: Vec<i32>,
    #[validate(custom(
        function = "validate_file_size",
        message = "File size must be less than 2MB."
    ))]
    #[validate(custom(
        function = "validate_file_content",
        message = "Content-Type must be image/jpeg, image/png, image/gif."
    ))]
    pub image_data: Option<Vec<u8>>,
}
