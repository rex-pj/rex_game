use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct PermissionModel {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub module: String,
    pub created_by_id: Option<i32>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub updated_by_id: Option<i32>,
    pub is_actived: bool,
}
