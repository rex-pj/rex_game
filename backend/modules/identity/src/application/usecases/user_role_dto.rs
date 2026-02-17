use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UserRoleDto {
    pub id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub role_id: i32,
    pub role_name: String,
    pub created_by_id: Option<i32>,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub updated_by_id: Option<i32>,
}
