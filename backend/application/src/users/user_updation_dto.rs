#[derive(Default)]
pub struct UserUpdationDto {
    pub email: Option<String>,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub updated_by_id: Option<i32>,
    pub status_id: Option<i32>,
}
