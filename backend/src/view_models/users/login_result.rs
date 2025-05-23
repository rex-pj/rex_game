use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginResult {
    pub access_token: String,
    pub expiration: DateTime<Utc>,
    // pub refresh_token: String,
}
