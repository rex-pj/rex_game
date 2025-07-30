use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AssignPermissionRequest {
    pub permission_codes: Option<Vec<String>>,
}
