use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginResult {
    pub token: String,
    pub refresh_token: String,
    pub user_id: i32,
    pub user_email: String,
    pub display_name: Option<String>,
}
