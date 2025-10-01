use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct SignupRequest {
    #[validate(email)]
    #[validate(length(
        min = 5,
        max = 200,
        message = "Email must be between 5 and 200 characters"
    ))]
    pub email: String,
    #[validate(length(
        min = 3,
        max = 100,
        message = "Name must be between 3 and 100 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 3,
        max = 100,
        message = "Display name must be between 3 and 100 characters"
    ))]
    pub display_name: Option<String>,
    #[validate(length(
        min = 6,
        max = 16,
        message = "Password must be between 6 and 16 characters"
    ))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Password confirmation does not match"))]
    pub password_confirm: String,
}
