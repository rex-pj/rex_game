use chrono::Utc;
use rex_game_domain::{
    entities::flashcard_type,
    repositories::flashcard_type_repository_trait::FlashcardTypeRepositoryTrait,
};
use sea_orm::Set;

use super::{
    flashcard_type_creation_dto::FlashcardTypeCreationDto, flashcard_type_dto::FlashcardTypeDto,
    flashcard_type_updation_dto::FlashcardTypeUpdationDto,
    flashcard_type_usecase_trait::FlashcardTypeUseCaseTrait,
};

#[derive(Clone)]
pub struct FlashcardTypeUseCase<TFT>
where
    TFT: FlashcardTypeRepositoryTrait,
{
    _flashcard_type_repository: TFT,
}

impl<TFT: FlashcardTypeRepositoryTrait> FlashcardTypeUseCase<TFT> {
    pub fn new(flashcard_type_repository: TFT) -> Self {
        Self {
            _flashcard_type_repository: flashcard_type_repository,
        }
    }
}

impl<TFT: FlashcardTypeRepositoryTrait> FlashcardTypeUseCaseTrait for FlashcardTypeUseCase<TFT> {
    async fn get_flashcard_types<'a>(
        &'a self,
        name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Option<Vec<FlashcardTypeDto>> {
        let existing = self
            ._flashcard_type_repository
            .get_list(name, page, page_size)
            .await;
        if let Ok(i) = existing {
            Some(
                i.0.into_iter()
                    .map(|f| FlashcardTypeDto {
                        id: f.id,
                        name: f.name,
                        description: f.description,
                        created_date: f.created_date.with_timezone(&Utc),
                        updated_date: f.updated_date.with_timezone(&Utc),
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    async fn get_flashcard_type_by_id<'a>(&'a self, id: i32) -> Option<FlashcardTypeDto> {
        let existing = self._flashcard_type_repository.get_by_id(id).await;
        match existing {
            Ok(i) => match i {
                Some(f) => Some(FlashcardTypeDto {
                    id: f.id,
                    name: f.name,
                    description: f.description,
                    created_date: f.created_date.with_timezone(&Utc),
                    updated_date: f.updated_date.with_timezone(&Utc),
                }),
                None => None,
            },
            Err(_) => None,
        }
    }

    async fn create_flashcard_type<'a>(
        &'a self,
        flashcard_type_req: FlashcardTypeCreationDto,
    ) -> Option<i32> {
        let active_flashcard_type = flashcard_type::ActiveModel {
            name: Set(flashcard_type_req.name),
            description: Set(flashcard_type_req.description),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            created_by_id: Set(flashcard_type_req.created_by_id),
            updated_by_id: Set(flashcard_type_req.updated_by_id),
            ..Default::default()
        };
        let created = self
            ._flashcard_type_repository
            .create(active_flashcard_type)
            .await;
        match created {
            Err(_) => None,
            Ok(i) => Some(i.last_insert_id),
        }
    }

    async fn update_flashcard_type<'a>(
        &'a self,
        id: i32,
        flashcard_type_req: FlashcardTypeUpdationDto,
    ) -> Option<FlashcardTypeDto> {
        let existing = self._flashcard_type_repository.get_by_id(id).await;
        match existing {
            Ok(exist) => match exist {
                Some(data) => {
                    let mut updating: flashcard_type::ActiveModel = data.into();
                    updating.name = Set(flashcard_type_req.name);
                    updating.description = Set(flashcard_type_req.description);
                    updating.updated_date = Set(Utc::now().fixed_offset());
                    updating.updated_by_id = Set(flashcard_type_req.updated_by_id);
                    let updated = self._flashcard_type_repository.update(updating).await;
                    match updated {
                        Ok(i) => Some(FlashcardTypeDto {
                            id: i.id,
                            name: i.name,
                            created_date: i.created_date.with_timezone(&Utc),
                            description: i.description,
                            updated_date: i.updated_date.with_timezone(&Utc),
                        }),
                        Err(_) => None,
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }
}
