#[derive(Clone)]
pub struct UserPermissionCreationDto {
    pub permission_id: i32,
    pub created_by_id: i32,
    pub updated_by_id: i32,
}
