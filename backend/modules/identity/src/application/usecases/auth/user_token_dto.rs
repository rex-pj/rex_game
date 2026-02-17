use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct UserTokenDto {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expiration: i32,
    pub created_by_id: i32,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub updated_by_id: i32,
    pub is_actived: bool,
    pub purpose: i32,
}
