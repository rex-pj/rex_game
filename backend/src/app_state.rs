use rex_game_application::flashcards::{
    flashcard_usecase::FlashcardUseCase, flashcard_usecase_trait::FlashcardUseCaseTrait,
};
use rex_game_domain::repositories::flashcard_repository_trait::FlashcardRepositoryTrait;
use rex_game_infrastructure::repositories::flashcard_repository::FlashcardRepository;

pub trait AppStateTrait: Clone + Send + Sync + 'static {
    type FlashcardRepository: FlashcardRepositoryTrait;
    type FlashcardUseCase: FlashcardUseCaseTrait;

    fn flashcard_repository(&self) -> &Self::FlashcardRepository;
    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase;
}

#[derive(Clone)]
pub struct RegularAppState {
    pub flashcard_repository: FlashcardRepository,
    pub flashcard_usecase: FlashcardUseCase<FlashcardRepository>,
}

impl AppStateTrait for RegularAppState {
    type FlashcardRepository = FlashcardRepository;
    type FlashcardUseCase = FlashcardUseCase<FlashcardRepository>;

    fn flashcard_repository(&self) -> &Self::FlashcardRepository {
        &self.flashcard_repository
    }

    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase {
        &self.flashcard_usecase
    }
}
