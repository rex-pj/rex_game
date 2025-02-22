use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginResult {
    pub token: String,
    pub refresh_token: String,
}
