use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FlashcardTypeCreateRequest {
    pub name: String,
    pub description: Option<String>,
}
