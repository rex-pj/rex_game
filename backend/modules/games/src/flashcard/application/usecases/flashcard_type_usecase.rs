use crate::flashcard::domain::{
    models::flashcard_type_model::FlashcardTypeModel,
    repositories::flashcard_type_repository_trait::FlashcardTypeRepositoryTrait,
};
use chrono::Utc;

use rex_game_shared::{domain::models::page_list_model::PageListModel, ApplicationError};

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
    ) -> Result<PageListModel<FlashcardTypeDto>, ApplicationError> {
        match self
            ._flashcard_type_repository
            .get_paged_list(name, page, page_size)
            .await
        {
            Ok(i) => {
                let items = i
                    .items
                    .into_iter()
                    .map(|f| FlashcardTypeDto {
                        id: f.id,
                        name: f.name,
                        description: f.description,
                        created_date: f.created_date.with_timezone(&Utc),
                        updated_date: f.updated_date.with_timezone(&Utc),
                    })
                    .collect();
                Ok(PageListModel {
                    items,
                    total_count: i.total_count,
                })
            }
            Err(err) => Err(ApplicationError::Infrastructure(err)),
        }
    }

    async fn get_flashcard_type_by_id<'a>(&'a self, id: i32) -> Option<FlashcardTypeDto> {
        let existing = self._flashcard_type_repository.get_by_id(id).await;
        match existing {
            Ok(f) => Some(FlashcardTypeDto {
                id: f.id,
                name: f.name,
                description: f.description,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
            }),
            Err(_) => None,
        }
    }

    async fn get_flashcard_type_by_flashcard_id<'a>(
        &'a self,
        flashcard_id: i32,
    ) -> Option<Vec<FlashcardTypeDto>> {
        let flashcard_types = self
            ._flashcard_type_repository
            .get_by_flashcard_id(flashcard_id)
            .await;
        match flashcard_types {
            Ok(i) => {
                let result = i
                    .into_iter()
                    .map(|f| FlashcardTypeDto {
                        id: f.id,
                        name: f.name,
                        description: f.description,
                        created_date: f.created_date.with_timezone(&Utc),
                        updated_date: f.updated_date.with_timezone(&Utc),
                    })
                    .collect();
                return Some(result);
            }
            Err(_) => None,
        }
    }

    async fn create_flashcard_type<'a>(
        &'a self,
        flashcard_type_req: FlashcardTypeCreationDto,
    ) -> Option<i32> {
        let active_flashcard_type = FlashcardTypeModel {
            name: flashcard_type_req.name,
            description: flashcard_type_req.description,
            created_by_id: flashcard_type_req.created_by_id,
            updated_by_id: flashcard_type_req.updated_by_id,
            ..Default::default()
        };
        let created = self
            ._flashcard_type_repository
            .create(active_flashcard_type)
            .await;
        match created {
            Err(_) => None,
            Ok(i) => Some(i),
        }
    }

    async fn update_flashcard_type<'a>(
        &'a self,
        id: i32,
        flashcard_type_req: FlashcardTypeUpdationDto,
    ) -> Option<bool> {
        let existing = self._flashcard_type_repository.get_by_id(id).await;
        match existing {
            Ok(exist) => {
                let updating = FlashcardTypeModel {
                    id: exist.id,
                    name: flashcard_type_req.name,
                    description: flashcard_type_req.description,
                    updated_by_id: flashcard_type_req.updated_by_id,
                    ..Default::default()
                };
                let updated = self._flashcard_type_repository.update(updating).await;
                match updated {
                    Ok(i) => Some(i),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    async fn delete_flashcard_type_by_id(&self, id: i32) -> Option<u64> {
        let deleted = self._flashcard_type_repository.delete_by_id(id).await;
        match deleted {
            Ok(i) => Some(i),
            Err(_) => None,
        }
    }
}
