use rex_game_application::flashcards::{
    flashcard_usecase::FlashcardUseCase, t_flashcard_usecase::TFlashcardUseCase,
};
use rex_game_domain::flashcards::t_flashcard_repository::TFlashcardRepository;
use rex_game_infrastructure::repositories::flashcard_repository::FlashcardRepository;

pub trait AppState: Clone + Send + Sync + 'static {
    type FlashcardRepository: TFlashcardRepository;
    type FlashcardUseCase: TFlashcardUseCase;

    // fn db_connection(&self) -> &self::Arc<DatabaseConnection>;
    fn flashcard_repository(&self) -> &Self::FlashcardRepository;
    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase;
}

#[derive(Clone)]
pub struct RegularAppState {
    // pub db_connection: Arc<DatabaseConnection>,
    pub flashcard_repository: FlashcardRepository,
    pub flashcard_usecase: FlashcardUseCase<FlashcardRepository>,
}

impl AppState for RegularAppState {
    type FlashcardRepository = FlashcardRepository;
    type FlashcardUseCase = FlashcardUseCase<FlashcardRepository>;

    fn flashcard_repository(&self) -> &Self::FlashcardRepository {
        &self.flashcard_repository
    }

    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase {
        &self.flashcard_usecase
    }

    // fn db_connection(&self) -> &self::Arc<DatabaseConnection> {
    //     &self.db_connection
    // }
}
