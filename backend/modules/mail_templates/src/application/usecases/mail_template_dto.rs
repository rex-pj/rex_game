use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailTemplateDto {
    pub id: i32,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub created_by_id: Option<i32>,
    pub created_on: String,
    pub updated_on: String,
    pub updated_by_id: Option<i32>,
    pub is_actived: bool,
    pub is_enabled: bool,
}
