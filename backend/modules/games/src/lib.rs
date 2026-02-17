pub mod flashcard;
pub mod scoring;

// Re-export flashcard module items
pub use flashcard::application::usecases::{
    flashcard_creation_dto::FlashcardCreationDto,
    flashcard_detail_dto::{FlashcardDetailDto, FlashcardGameTypeInfo},
    flashcard_dto::FlashcardDto, flashcard_file_dto::FlashcardFileDto,
    flashcard_type_creation_dto::FlashcardTypeCreationDto, flashcard_type_dto::FlashcardTypeDto,
    flashcard_type_updation_dto::FlashcardTypeUpdationDto, flashcard_updation_dto::FlashcardUpdationDto,
    FlashcardTypeUseCase, FlashcardTypeUseCaseTrait, FlashcardUseCase, FlashcardUseCaseTrait,
};
pub use flashcard::infrastructure::repositories::{
    flashcard_file_repository::FlashcardFileRepository, flashcard_repository::FlashcardRepository,
    flashcard_type_relation_repository::FlashcardTypeRelationRepository,
    flashcard_type_repository::FlashcardTypeRepository,
};

// Re-export scoring module items
pub use scoring::{
    AchievementCreationDto, AchievementDto, AchievementUpdationDto, AdminAchievementDto,
    AdminGameSessionDto, AdminUserStatsDto, CompleteGameSessionDto, GameCompleteResponseDto,
    GameProgressDto, GameSessionDto, GameTypeCreationDto, GameTypeDto, GameTypeUpdationDto,
    LeaderboardEntryDto, SaveGameProgressDto, ScoringRepository, ScoringRepositoryTrait,
    ScoringUseCase, ScoringUseCaseTrait, StartGameSessionDto, UserStatsDto,
};
