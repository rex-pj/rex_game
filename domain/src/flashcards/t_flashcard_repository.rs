use super::flashcard::Flashcard;

pub trait TFlashcardRepository {
    fn get_flashcard(&self) -> Option<Flashcard>;
}
