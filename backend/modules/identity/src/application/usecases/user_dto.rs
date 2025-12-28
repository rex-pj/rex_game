use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub display_name: Option<String>,
    pub created_by_id: Option<i32>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub updated_by_id: Option<i32>,
    pub status_id: i32,
}
