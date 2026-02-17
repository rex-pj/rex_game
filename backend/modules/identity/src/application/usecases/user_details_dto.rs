use chrono::{DateTime, Utc};

pub struct UserDetailsDto {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub display_name: Option<String>,
    pub password_hash: String,
    pub security_stamp: String,
    pub created_by_id: Option<i32>,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub updated_by_id: Option<i32>,
    pub status_id: i32,
}
