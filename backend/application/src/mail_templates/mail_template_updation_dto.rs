#[derive(Default)]
pub struct MailTemplateUpdationDto {
    pub name: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub is_actived: Option<bool>,
    pub is_enabled: Option<bool>,
    pub updated_by_id: i32,
}
