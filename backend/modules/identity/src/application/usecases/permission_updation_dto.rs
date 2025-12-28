#[derive(Default)]
pub struct PermissionUpdationDto {
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub module: Option<String>,
    pub is_actived: Option<bool>,
    pub updated_by_id: i32,
}
