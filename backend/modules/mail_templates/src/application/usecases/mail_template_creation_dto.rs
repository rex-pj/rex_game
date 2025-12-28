use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MailTemplateCreationDto {
    pub name: String,
    pub subject: String,
    pub body: String,
    pub created_by_id: Option<i32>,
}
