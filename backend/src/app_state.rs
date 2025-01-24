use rex_game_application::flashcards::{
    flashcard_usecase::FlashcardUseCase, flashcard_usecase_trait::FlashcardUseCaseTrait,
};
use rex_game_infrastructure::repositories::{
    flashcard_file_repository::FlashcardFileRepository, flashcard_repository::FlashcardRepository,
};

pub trait AppStateTrait: Clone + Send + Sync + 'static {
    type FlashcardUseCase: FlashcardUseCaseTrait;
    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase;
}

#[derive(Clone)]
pub struct RegularAppState {
    pub flashcard_usecase: FlashcardUseCase<FlashcardRepository, FlashcardFileRepository>,
}

impl AppStateTrait for RegularAppState {
    type FlashcardUseCase = FlashcardUseCase<FlashcardRepository, FlashcardFileRepository>;

    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase {
        &self.flashcard_usecase
    }
}
