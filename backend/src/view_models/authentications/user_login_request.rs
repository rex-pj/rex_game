use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UserLoginRequest {
    #[validate(email)]
    #[validate(length(
        min = 5,
        max = 200,
        message = "Email must be between 5 and 200 characters"
    ))]
    pub email: String,
    #[validate(length(
        min = 6,
        max = 16,
        message = "Password must be between 6 and 16 characters"
    ))]
    pub password: String,
}
