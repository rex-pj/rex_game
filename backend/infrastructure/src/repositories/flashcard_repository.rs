use std::sync::Arc;

use rex_game_domain::flashcards::{
    flashcard::Flashcard, t_flashcard_repository::TFlashcardRepository,
};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct FlashcardRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl FlashcardRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl TFlashcardRepository for FlashcardRepository {
    fn get_flashcard(&self) -> Option<Flashcard> {
        Some(Flashcard::new(
            "What is Rust?",
            "A programming language",
            "sub_de",
        ))
    }
}
