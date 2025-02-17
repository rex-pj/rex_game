use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserLoginParameter {
    pub email: String,
    pub password: String,
}
