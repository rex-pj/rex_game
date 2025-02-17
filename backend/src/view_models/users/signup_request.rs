use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub name: String,
    pub display_name: Option<String>,
    pub password: String,
    pub password_confirm: String,
}
