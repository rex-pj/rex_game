use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct RolePermissionModel {
    pub id: i32,
    pub role_id: i32,
    pub permission_id: i32,
    pub permission_name: String,
    pub permission_code: String,
    pub permission_module: String,
    pub created_by_id: i32,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub updated_by_id: i32,
    pub is_actived: bool,
}
