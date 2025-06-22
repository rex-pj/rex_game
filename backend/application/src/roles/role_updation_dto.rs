#[derive(Default)]
pub struct RoleUpdationDto {
    pub name: String,
    pub description: Option<String>,
    pub updated_by_id: Option<i32>,
}
