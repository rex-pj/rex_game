use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailTemplateDeletionDto {
    pub id: i32,
}
