use rex_game_domain::flashcards::{
    flashcard::Flashcard, t_flashcard_repository::TFlashcardRepository,
};

#[derive(Clone, Copy)]
pub struct FlashcardRepository {}

impl TFlashcardRepository for FlashcardRepository {
    fn get_flashcard(&self) -> Option<Flashcard> {
        // Đây là logic lấy flashcard
        Some(Flashcard::new(
            "What is Rust?",
            "A programming language",
            "sub_de",
        ))
    }
}
