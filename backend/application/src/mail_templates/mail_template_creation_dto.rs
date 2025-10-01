#[derive(Default)]
pub struct MailTemplateCreationDto {
    pub name: String,
    pub subject: String,
    pub body: String,
    pub created_by_id: Option<i32>,
    pub updated_by_id: Option<i32>,
}
