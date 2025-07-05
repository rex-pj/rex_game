#[derive(Default)]
pub struct PermissionCreationDto {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub module: String,
    pub created_by_id: Option<i32>,
    pub updated_by_id: Option<i32>,
}
