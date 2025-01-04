use std::future::Future;

use super::flashcard_dto::FlashcardDto;

pub trait FlashcardUseCaseTrait {
    fn get_flashcards<'a>(&'a self) -> impl Future<Output = Option<Vec<FlashcardDto>>>;
}
