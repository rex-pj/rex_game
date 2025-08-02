use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct LoggedInUserDto {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub display_name: Option<String>,
    pub roles: Vec<LoggedInUserRoleDto>,
    pub permissions: Vec<LoggedInUserPermissonDto>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct LoggedInUserRoleDto {
    pub role_name: String,
    pub role_id: i32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct LoggedInUserPermissonDto {
    pub permisson_code: String,
    pub permisson_id: i32,
    pub permisson_name: String,
}
