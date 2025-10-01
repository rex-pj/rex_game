use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct PermissionCreateRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Code must be between 1 and 100 characters"
    ))]
    pub code: String,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Module must be between 1 and 100 characters"
    ))]
    pub module: String,
    pub description: Option<String>,
}
