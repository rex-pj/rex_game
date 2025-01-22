use chrono::Utc;
use rex_game_domain::{
    entities::flashcard, repositories::flashcard_repository_trait::FlashcardRepositoryTrait,
};
use sea_orm::Set;

use super::{
    flashcard_creation_dto::FlashcardCreationDto, flashcard_dto::FlashcardDto,
    flashcard_usecase_trait::FlashcardUseCaseTrait,
};

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
    async fn get_flashcards<'a>(&'a self, page: u64, page_size: u64) -> Option<Vec<FlashcardDto>> {
        let existing = self._flashcard_repository.get_list(page, page_size).await;
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

    async fn get_flashcard_by_id<'a>(&'a self, id: i32) -> Option<FlashcardDto> {
        let existing = self._flashcard_repository.get_by_id(id).await;
        match existing {
            Ok(i) => match i {
                Some(f) => Some(FlashcardDto {
                    id: f.id,
                    name: f.name,
                    description: f.description,
                    sub_description: f.sub_description,
                    created_date: f.created_date.with_timezone(&Utc),
                    updated_date: f.updated_date.with_timezone(&Utc),
                }),
                None => None,
            },
            Err(_) => None,
        }
    }

    async fn create_flashcard<'a>(&'a self, flashcard: FlashcardCreationDto) -> Option<i32> {
        let active_flashcard = flashcard::ActiveModel {
            name: Set(flashcard.name),
            description: Set(flashcard.description),
            sub_description: Set(flashcard.sub_description),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            image_data: Set(flashcard.image_data),
            ..Default::default()
        };
        let existing = self._flashcard_repository.create(active_flashcard).await;
        match existing {
            Err(_) => None,
            Ok(i) => Some(i.last_insert_id),
        }
    }
}
