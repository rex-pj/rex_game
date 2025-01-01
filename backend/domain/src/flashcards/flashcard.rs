use chrono::{DateTime, Utc};

use crate::Entity;

pub struct Flashcard {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub sub_description: String,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

impl Flashcard {
    pub fn new(name: &str, description: &str, sub_description: &str) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            description: description.to_string(),
            sub_description: sub_description.to_string(),
            created_date: Utc::now(),
            updated_date: Utc::now(),
        }
    }
}

impl Entity for Flashcard {}

pub struct CreateFlashcardRequest {
    pub name: String,
    pub description: String,
    pub sub_description: String,
}

impl CreateFlashcardRequest {
    pub fn new(name: &str, description: &str, sub_description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            sub_description: sub_description.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn sub_description(&self) -> &str {
        &self.sub_description
    }
}

#[derive(Debug)]
pub enum CreateFlashcardError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}

#[derive(Debug)]
pub enum GetFlashcardError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}
