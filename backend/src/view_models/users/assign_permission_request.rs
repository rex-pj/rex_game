use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AssignPermissionRequest {
    pub permission_code: String,
}
