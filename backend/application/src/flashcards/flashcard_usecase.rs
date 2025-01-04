use chrono::Utc;
use rex_game_domain::flashcards::flashcard_repository_trait::FlashcardRepositoryTrait;

use super::{flashcard_dto::FlashcardDto, flashcard_usecase_trait::FlashcardUseCaseTrait};

#[derive(Clone)]
pub struct FlashcardUseCase<TF>
where
    TF: FlashcardRepositoryTrait,
{
    _flashcard_repository: TF,
}

impl<TF: FlashcardRepositoryTrait> FlashcardUseCase<TF> {
    pub fn new(flashcard_repository: TF) -> Self {
        Self {
            _flashcard_repository: flashcard_repository,
        }
    }
}

impl<TF: FlashcardRepositoryTrait> FlashcardUseCaseTrait for FlashcardUseCase<TF> {
    async fn get_flashcards<'a>(&'a self) -> Option<Vec<FlashcardDto>> {
        let existing = self._flashcard_repository.get_flashcards(1, 10).await;
        match existing {
            Err(_) => None,
            Ok(i) => Some(
                i.0.into_iter()
                    .map(|f| FlashcardDto {
                        id: f.id,
                        name: f.name,
                        description: f.description,
                        sub_description: f.sub_description,
                        created_date: f.created_date.with_timezone(&Utc),
                        updated_date: f.updated_date.with_timezone(&Utc),
                    })
                    .collect(),
            ),
        }
    }
}
