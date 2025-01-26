use rex_game_application::{
    flashcard_types::{
        flashcard_type_usecase::FlashcardTypeUseCase,
        flashcard_type_usecase_trait::FlashcardTypeUseCaseTrait,
    },
    flashcards::{
        flashcard_usecase::FlashcardUseCase, flashcard_usecase_trait::FlashcardUseCaseTrait,
    },
};
use rex_game_infrastructure::repositories::{
    flashcard_file_repository::FlashcardFileRepository, flashcard_repository::FlashcardRepository,
    flashcard_type_relation_repository::FlashcardTypeRelationRepository,
    flashcard_type_repository::FlashcardTypeRepository,
};

pub trait AppStateTrait: Clone + Send + Sync + 'static {
    type FlashcardUseCase: FlashcardUseCaseTrait;
    type FlashcardTypeUseCase: FlashcardTypeUseCaseTrait;
    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase;
    fn flashcard_type_usecase(&self) -> &Self::FlashcardTypeUseCase;
}

#[derive(Clone)]
pub struct RegularAppState {
    pub flashcard_usecase: FlashcardUseCase<
        FlashcardRepository,
        FlashcardFileRepository,
        FlashcardTypeRelationRepository,
    >,
    pub flashcard_type_usecase: FlashcardTypeUseCase<FlashcardTypeRepository>,
}

impl AppStateTrait for RegularAppState {
    type FlashcardUseCase = FlashcardUseCase<
        FlashcardRepository,
        FlashcardFileRepository,
        FlashcardTypeRelationRepository,
    >;
    type FlashcardTypeUseCase = FlashcardTypeUseCase<FlashcardTypeRepository>;

    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase {
        &self.flashcard_usecase
    }

    fn flashcard_type_usecase(&self) -> &Self::FlashcardTypeUseCase {
        &self.flashcard_type_usecase
    }
}
