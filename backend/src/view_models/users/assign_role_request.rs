use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AssignRoleRequest {
    pub role_name: String,
}
