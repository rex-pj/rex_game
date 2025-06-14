use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct UseRoleDto {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub role_name: String,
    pub created_by_id: Option<i32>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub updated_by_id: Option<i32>,
}
