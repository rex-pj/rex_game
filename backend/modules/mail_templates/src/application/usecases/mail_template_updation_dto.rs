use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MailTemplateUpdationDto {
    pub id: i32,
    pub name: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub updated_by_id: Option<i32>,
}
