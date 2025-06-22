use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct LoggedInUserDto {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub display_name: Option<String>,
    pub roles: Vec<LoggedInUserRoleDto>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct LoggedInUserRoleDto {
    pub role_name: String,
    pub role_id: i32,
}
