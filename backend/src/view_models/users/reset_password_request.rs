use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 1, message = "Title must be at least 1 characters long"))]
    pub token: Option<String>,
    #[validate(length(
        min = 6,
        max = 16,
        message = "Password must be between 6 and 16 characters"
    ))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Password confirmation does not match"))]
    pub password_confirm: String,
}
