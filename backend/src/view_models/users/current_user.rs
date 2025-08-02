use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CurrentUser {
    pub id: i32,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
