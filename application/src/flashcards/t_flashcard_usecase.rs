use std::{future::Future, pin::Pin};

use rex_game_domain::flashcards::flashcard::{
    CreateFlashcardError, CreateFlashcardRequest, Flashcard,
};

use super::flashcard_dto::FlashcardDto;

pub trait TFlashcardUseCase {
    fn create_flashcard<'a>(
        &'a self,
        req: &'a CreateFlashcardRequest,
    ) -> Pin<Box<dyn Future<Output = Result<Flashcard, CreateFlashcardError>> + Send + 'a>>;

    fn get_flashcard<'a>(&'a self) -> Option<FlashcardDto>;
}