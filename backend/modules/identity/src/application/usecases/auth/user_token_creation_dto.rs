use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct UserTokenCreationDto {
    pub user_id: i32,
    pub token: String,
    pub expiration: i32,
    pub created_by_id: i32,
    pub updated_by_id: i32,
    pub purpose: i32,
}
