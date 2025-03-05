use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginResult {
    pub access_token: String,
}
