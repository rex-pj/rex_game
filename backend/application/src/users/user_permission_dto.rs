use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct UserPermissionDto {
    pub id: i32,
    pub user_id: i32,
    pub permission_id: i32,
    pub permission_name: String,
    pub permission_code: String,
    pub permission_module: String,
    pub created_by_id: Option<i32>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub updated_by_id: Option<i32>,
}
