use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PermissionDto {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub module: String,
    pub created_by_id: Option<i32>,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub updated_by_id: Option<i32>,
}
