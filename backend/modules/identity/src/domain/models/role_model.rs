use chrono::{DateTime, Utc};

#[derive(Default, Clone)]
pub struct RoleModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_by_id: Option<i32>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub updated_by_id: Option<i32>,
    pub is_actived: bool,
}
