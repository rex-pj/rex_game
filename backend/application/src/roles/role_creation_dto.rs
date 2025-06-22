#[derive(Default)]
pub struct RoleCreationDto {
    pub name: String,
    pub description: Option<String>,
    pub created_by_id: Option<i32>,
    pub updated_by_id: Option<i32>,
}
