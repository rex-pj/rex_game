#[derive(Default)]
pub struct RoleUpdationDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_actived: Option<bool>,
    pub updated_by_id: i32,
}
