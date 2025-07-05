use chrono::Utc;
use rex_game_domain::{
    models::{
        flashcard_file_model::FlashcardFileModel, flashcard_model::FlashcardModel,
        flashcard_type_relation_model::FlashcardTypeRelationModel,
    },
    repositories::{
        flashcard_file_repository_trait::FlashcardFileRepositoryTrait,
        flashcard_repository_trait::FlashcardRepositoryTrait,
        flashcard_type_relation_repository_trait::FlashcardTypeRelationRepositoryTrait,
    },
};

use crate::{
    errors::application_error::{ApplicationError, ApplicationErrorKind},
    page_list_dto::PageListDto,
};

use super::{
    flashcard_creation_dto::FlashcardCreationDto, flashcard_dto::FlashcardDto,
    flashcard_file_dto::FlashcardFileDto, flashcard_updation_dto::FlashcardUpdationDto,
    flashcard_usecase_trait::FlashcardUseCaseTrait,
};

#[derive(Clone)]
pub struct FlashcardUseCase<TF, TFF, TFTR>
where
    TF: FlashcardRepositoryTrait,
    TFF: FlashcardFileRepositoryTrait,
    TFTR: FlashcardTypeRelationRepositoryTrait,
{
    _flashcard_repository: TF,
    _flashcard_file_repository: TFF,
    _flashcard_type_relation_repository: TFTR,
}

impl<
        TF: FlashcardRepositoryTrait,
        TFF: FlashcardFileRepositoryTrait,
        TFTR: FlashcardTypeRelationRepositoryTrait,
    > FlashcardUseCase<TF, TFF, TFTR>
{
    pub fn new(
        flashcard_repository: TF,
        flashcard_file_repository: TFF,
        flashcard_type_relation_repository: TFTR,
    ) -> Self {
        Self {
            _flashcard_repository: flashcard_repository,
            _flashcard_file_repository: flashcard_file_repository,
            _flashcard_type_relation_repository: flashcard_type_relation_repository,
        }
    }
}

impl<
        TF: FlashcardRepositoryTrait,
        TFF: FlashcardFileRepositoryTrait,
        TFTR: FlashcardTypeRelationRepositoryTrait,
    > FlashcardUseCaseTrait for FlashcardUseCase<TF, TFF, TFTR>
{
    async fn get_paged_list<'a>(
        &'a self,
        type_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Result<PageListDto<FlashcardDto>, ApplicationError> {
        match self
            ._flashcard_repository
            .get_list(type_name, page, page_size)
            .await
        {
            Ok(page_list) => {
                let items = page_list
                    .items
                    .into_iter()
                    .map(|f| FlashcardDto {
                        id: f.id,
                        name: f.name,
                        description: f.description,
                        sub_description: f.sub_description,
                        created_date: f.created_date.with_timezone(&Utc),
                        updated_date: f.updated_date.with_timezone(&Utc),
                        image_id: f.file_id,
                    })
                    .collect();

                Ok(PageListDto {
                    items,
                    total_count: page_list.total_count,
                    page,
                    page_size,
                })
            }
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Failed to get flashcards",
                None,
            )),
        }
    }

    async fn get_flashcard_by_id<'a>(&'a self, id: i32) -> Option<FlashcardDto> {
        let existing = self._flashcard_repository.get_by_id(id).await;
        match existing {
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
        }
    }

    async fn create_flashcard<'a>(
        &'a self,
        flashcard_req: FlashcardCreationDto,
    ) -> Result<i32, ApplicationError> {
        let active_flashcard_file = FlashcardFileModel {
            name: Some(flashcard_req.name.clone()),
            file_name: flashcard_req.file_name,
            content_type: flashcard_req.content_type,
            data: flashcard_req.image_data.unwrap(),
            created_by_id: flashcard_req.created_by_id,
            updated_by_id: flashcard_req.updated_by_id,
            ..Default::default()
        };

        let new_file_id = self
            ._flashcard_file_repository
            .create(active_flashcard_file)
            .await
            .map_err(|_| {
                ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Failed to create flashcard file",
                    None,
                )
            })?;
        let active_flashcard = FlashcardModel {
            name: flashcard_req.name,
            description: flashcard_req.description,
            sub_description: flashcard_req.sub_description,
            file_id: new_file_id,
            created_by_id: flashcard_req.created_by_id,
            updated_by_id: flashcard_req.updated_by_id,
            ..Default::default()
        };
        let created_id = self
            ._flashcard_repository
            .create(active_flashcard)
            .await
            .map_err(|_| {
                ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Failed to create flashcard",
                    None,
                )
            })?;

        let mut active_type_relations: Vec<FlashcardTypeRelationModel> = Vec::new();
        for type_relation_id in flashcard_req.type_ids.iter() {
            let active_flashcard_type_relation = FlashcardTypeRelationModel {
                flashcard_id: created_id,
                flashcard_type_id: *type_relation_id,
                created_by_id: flashcard_req.created_by_id,
                updated_by_id: flashcard_req.updated_by_id,
                ..Default::default()
            };
            active_type_relations.push(active_flashcard_type_relation);
        }

        let type_relations_created = self
            ._flashcard_type_relation_repository
            .create(active_type_relations)
            .await
            .map_err(|_| {
                ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Failed to create flashcard type relation",
                    None,
                )
            });

        match type_relations_created {
            Ok(_) => Ok(created_id),
            Err(err) => Err(err),
        }
    }

    async fn update_flashcard<'a>(
        &'a self,
        id: i32,
        flashcard_req: FlashcardUpdationDto,
    ) -> Result<bool, ApplicationError> {
        let existing_flashcard = match self._flashcard_repository.get_by_id(id).await {
            Some(flashcard) => flashcard,
            None => {
                return Err(ApplicationError::new(
                    ApplicationErrorKind::NotFound,
                    "Flashcard not found",
                    None,
                ))
            }
        };

        // Updating file
        if let Some(req_file) = flashcard_req.image_data {
            let existing_file = self
                ._flashcard_file_repository
                .get_by_id(existing_flashcard.file_id)
                .await
                .map_err(|_| {
                    ApplicationError::new(
                        ApplicationErrorKind::DatabaseError,
                        "Failed to get flashcard file",
                        None,
                    )
                })?;

            let updating_file = FlashcardFileModel {
                id: existing_file.id,
                name: existing_file.name,
                file_name: existing_file.file_name,
                content_type: existing_file.content_type,
                data: req_file,
                ..Default::default()
            };

            self._flashcard_file_repository
                .update(updating_file)
                .await
                .map_err(|_| {
                    ApplicationError::new(
                        ApplicationErrorKind::DatabaseError,
                        "Failed to update flashcard file",
                        None,
                    )
                })?;
        };

        // Updating flashcard information
        let mut updating_flashcard = FlashcardModel {
            id: existing_flashcard.id,
            name: existing_flashcard.name,
            description: existing_flashcard.description,
            sub_description: existing_flashcard.sub_description,
            file_id: existing_flashcard.file_id,
            created_by_id: existing_flashcard.created_by_id,
            updated_by_id: flashcard_req.updated_by_id,
            created_date: existing_flashcard.created_date,
            updated_date: existing_flashcard.updated_date,
            is_actived: existing_flashcard.is_actived,
        };

        if let Some(name) = flashcard_req.name {
            updating_flashcard.name = name;
        }

        if let Some(description) = flashcard_req.description {
            updating_flashcard.description = Some(description);
        }

        if let Some(sub_description) = flashcard_req.sub_description {
            updating_flashcard.sub_description = Some(sub_description);
        }

        updating_flashcard.updated_by_id = flashcard_req.updated_by_id;

        self._flashcard_repository
            .update(updating_flashcard)
            .await
            .map_err(|_| {
                ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Failed to update flashcard",
                    None,
                )
            })?;

        match flashcard_req.type_ids {
            Some(req_type_ids) => {
                self._flashcard_type_relation_repository
                    .delete_by_flashcard_id(id)
                    .await
                    .map_err(|_| {
                        ApplicationError::new(
                            ApplicationErrorKind::DatabaseError,
                            "Failed to delete flashcard type relation",
                            None,
                        )
                    })?;
                let mut existing_type_relations: Vec<FlashcardTypeRelationModel> = Vec::new();
                for type_relation_id in req_type_ids.iter() {
                    let active_type_relation = FlashcardTypeRelationModel {
                        flashcard_id: id,
                        flashcard_type_id: *type_relation_id,
                        updated_by_id: flashcard_req.updated_by_id,
                        created_by_id: flashcard_req.updated_by_id,
                        ..Default::default()
                    };
                    existing_type_relations.push(active_type_relation);
                }

                self._flashcard_type_relation_repository
                    .create(existing_type_relations)
                    .await
                    .map_err(|_| {
                        ApplicationError::new(
                            ApplicationErrorKind::DatabaseError,
                            "Failed to update flashcard type relation",
                            None,
                        )
                    })?;

                return Ok(true);
            }
            None => Err(ApplicationError::new(
                ApplicationErrorKind::InvalidInput,
                "Type IDs are required for updating flashcard",
                None,
            )),
        }
    }

    async fn get_image_by_file_id<'a>(
        &'a self,
        file_id: i32,
    ) -> Result<FlashcardFileDto, ApplicationError> {
        let existing = self
            ._flashcard_file_repository
            .get_by_id(file_id)
            .await
            .map_err(|_| {
                ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Failed to get flashcard file",
                    None,
                )
            })?;

        Ok(FlashcardFileDto {
            id: existing.id,
            name: existing.name,
            file_name: existing.file_name,
            content_type: existing.content_type,
            data: existing.data,
        })
    }

    async fn delete_flashcard_by_id(&self, id: i32) -> Option<u64> {
        let flashcard = match self._flashcard_repository.get_by_id(id).await {
            Some(f) => f,
            None => return None,
        };

        match self
            ._flashcard_type_relation_repository
            .delete_by_flashcard_id(id)
            .await
        {
            Ok(i) => i,
            Err(_) => return None,
        };

        match self._flashcard_repository.delete_by_id(id).await {
            Ok(i) => i,
            Err(_) => return None,
        };

        match self
            ._flashcard_file_repository
            .delete_by_id(flashcard.file_id)
            .await
        {
            Ok(i) => Some(i),
            Err(_) => None,
        }
    }
}
