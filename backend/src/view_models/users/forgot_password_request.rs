use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email)]
    #[validate(length(
        min = 5,
        max = 200,
        message = "Email must be between 5 and 200 characters"
    ))]
    pub email: String,
}
