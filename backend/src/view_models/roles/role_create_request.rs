use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoleCreateRequest {
    pub name: String,
    pub description: Option<String>,
}
