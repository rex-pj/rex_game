pub struct UserRoleCreationDto {
    pub user_id: i32,
    pub role_name: String,
    pub created_by_id: Option<i32>,
    pub updated_by_id: Option<i32>,
}
