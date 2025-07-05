use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PermissionCreateRequest {
    pub name: String,
    pub code: String,
    pub module: String,
    pub description: Option<String>,
}
