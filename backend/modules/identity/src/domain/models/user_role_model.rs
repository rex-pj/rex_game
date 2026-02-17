use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct UserRoleModel {
    pub id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub role_id: i32,
    pub role_name: String,
    pub created_by_id: i32,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub updated_by_id: i32,
    pub is_actived: bool,
}
