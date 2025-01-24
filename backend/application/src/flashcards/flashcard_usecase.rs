use chrono::Utc;
use rex_game_domain::{
    entities::{flashcard, flashcard_file},
    repositories::{
        flashcard_file_repository_trait::FlashcardFileRepositoryTrait,
        flashcard_repository_trait::FlashcardRepositoryTrait,
    },
};
use sea_orm::Set;

use super::{
    flashcard_creation_dto::FlashcardCreationDto, flashcard_dto::FlashcardDto,
    flashcard_usecase_trait::FlashcardUseCaseTrait,
};

#[derive(Clone)]
pub struct FlashcardUseCase<TF, TFF>
where
    TF: FlashcardRepositoryTrait,
    TFF: FlashcardFileRepositoryTrait,
{
    _flashcard_repository: TF,
    _flashcard_file_repository: TFF,
}

impl<TF: FlashcardRepositoryTrait, TFF: FlashcardFileRepositoryTrait> FlashcardUseCase<TF, TFF> {
    pub fn new(flashcard_repository: TF, flashcard_file_repository: TFF) -> Self {
        Self {
            _flashcard_repository: flashcard_repository,
            _flashcard_file_repository: flashcard_file_repository,
        }
    }
}

impl<TF: FlashcardRepositoryTrait, TFF: FlashcardFileRepositoryTrait> FlashcardUseCaseTrait
    for FlashcardUseCase<TF, TFF>
{
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
                        image_id: f.file_id,
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
                    image_id: f.file_id,
                }),
                None => None,
            },
            Err(_) => None,
        }
    }

    async fn create_flashcard<'a>(&'a self, flashcard: FlashcardCreationDto) -> Option<i32> {
        let active_flashcard_file = flashcard_file::ActiveModel {
            name: Set(Some(flashcard.name.clone())),
            file_name: Set(flashcard.file_name),
            content_type: Set(flashcard.content_type),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            data: Set(flashcard.image_data),
            ..Default::default()
        };

        let new_file = self
            ._flashcard_file_repository
            .create(active_flashcard_file)
            .await;
        match new_file {
            Err(_) => None,
            Ok(file) => {
                let active_flashcard = flashcard::ActiveModel {
                    name: Set(flashcard.name),
                    description: Set(flashcard.description),
                    sub_description: Set(flashcard.sub_description),
                    created_date: Set(Utc::now().fixed_offset()),
                    updated_date: Set(Utc::now().fixed_offset()),
                    file_id: Set(file.last_insert_id),
                    ..Default::default()
                };
                let existing = self._flashcard_repository.create(active_flashcard).await;
                match existing {
                    Err(_) => None,
                    Ok(i) => Some(i.last_insert_id),
                }
            }
        }
    }

    async fn get_image_by_file_id<'a>(&'a self, file_id: i32) -> Option<Vec<u8>> {
        let existing = self._flashcard_file_repository.get_by_id(file_id).await;
        match existing {
            Ok(i) => match i {
                Some(f) => Some(f.data),
                None => None,
            },
            Err(_) => None,
        }
    }
}
