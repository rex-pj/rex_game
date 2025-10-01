use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct MailTemplateCreateRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 10,
        max = 255,
        message = "Subject must be between 10 and 255 characters"
    ))]
    pub subject: String,
    #[validate(length(min = 10, message = "Body must be at least 10 characters long"))]
    pub body: String,
}
