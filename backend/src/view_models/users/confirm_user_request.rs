use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct ConfirmUserRequest {
    #[validate(length(min = 1, message = "Title must be at least 1 characters long"))]
    pub token: Option<String>,
}
