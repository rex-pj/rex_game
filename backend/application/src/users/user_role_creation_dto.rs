#[derive(Clone)]
pub struct UserRoleCreationDto {
    pub role_id: i32,
    pub created_by_id: i32,
    pub updated_by_id: i32,
}
