use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct UserModel {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub display_name: Option<String>,
    pub password_hash: String,
    pub security_stamp: String,
    pub created_by_id: Option<i32>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub updated_by_id: Option<i32>,
    pub status_id: i32,
}
