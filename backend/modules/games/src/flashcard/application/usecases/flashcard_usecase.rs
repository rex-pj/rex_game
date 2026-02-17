use crate::flashcard::domain::{
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
use chrono::Utc;
use rex_game_shared::ApplicationError;

use rex_game_shared::domain::models::page_list_model::PageListModel;

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
        game_type_code: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Result<PageListModel<FlashcardDto>, ApplicationError> {
        match self
            ._flashcard_repository
            .get_list(game_type_code, page, page_size)
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
                        created_on: f.created_on.with_timezone(&Utc),
                        updated_on: f.updated_on.with_timezone(&Utc),
                        image_id: f.file_id,
                        is_actived: f.is_actived,
                        flashcard_type_names: vec![],
                    })
                    .collect();

                Ok(PageListModel {
                    items,
                    total_count: page_list.total_count,
                })
            }
            Err(err) => Err(ApplicationError::Infrastructure(err)),
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
                created_on: f.created_on.with_timezone(&Utc),
                updated_on: f.updated_on.with_timezone(&Utc),
                image_id: f.file_id,
                is_actived: f.is_actived,
                flashcard_type_names: vec![],
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
            .map_err(|err| ApplicationError::Infrastructure(err))?;
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
            .map_err(|err| ApplicationError::Infrastructure(err))?;

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
            .map_err(|err| ApplicationError::Infrastructure(err));

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
                return Err(ApplicationError::not_found(
                    "Flashcard not found",
                    id.to_string(),
                ))
            }
        };

        // Updating file
        if let Some(req_file) = flashcard_req.image_data {
            let existing_file = self
                ._flashcard_file_repository
                .get_by_id(existing_flashcard.file_id)
                .await
                .map_err(|_| ApplicationError::EntityNotFound {
                    entity: "Flashcard file".to_string(),
                    id: existing_flashcard.file_id.to_string(),
                })?;

            let updating_file = FlashcardFileModel {
                data: req_file,
                id: existing_file.id,
                name: existing_file.name,
                file_name: existing_file.file_name,
                content_type: existing_file.content_type,
                updated_by_id: flashcard_req.updated_by_id,
                ..Default::default()
            };

            self._flashcard_file_repository
                .update(updating_file)
                .await
                .map_err(|_| ApplicationError::business_rule("Failed to update flashcard file"))?;
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
            created_on: existing_flashcard.created_on,
            updated_on: existing_flashcard.updated_on,
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
            .map_err(|_| ApplicationError::business_rule("Failed to update flashcard"))?;

        match flashcard_req.type_ids {
            Some(req_type_ids) => {
                self._flashcard_type_relation_repository
                    .delete_by_flashcard_id(id)
                    .await
                    .map_err(|_| {
                        ApplicationError::business_rule("Failed to delete flashcard type relation")
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
                        ApplicationError::business_rule("Failed to update flashcard type relation")
                    })?;

                return Ok(true);
            }
            None => Err(ApplicationError::business_rule(
                "Type IDs are required for updating flashcard",
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
            .map_err(|_| ApplicationError::EntityNotFound {
                entity: "Flashcard_file".to_string(),
                id: file_id.to_string(),
            })?;

        Ok(FlashcardFileDto {
            id: existing.id,
            name: existing.name,
            file_name: existing.file_name,
            content_type: existing.content_type,
            data: existing.data,
        })
    }

    async fn delete_flashcard_by_id(&self, id: i32) -> Result<u64, ApplicationError> {
        let flashcard = match self._flashcard_repository.get_by_id(id).await {
            Some(f) => f,
            None => {
                return Err(ApplicationError::not_found(
                    "Flashcard not found",
                    id.to_string(),
                ))
            }
        };

        match self
            ._flashcard_type_relation_repository
            .delete_by_flashcard_id(id)
            .await
        {
            Ok(i) => i,
            Err(_) => {
                return Err(ApplicationError::business_rule(
                    "Failed to delete flashcard type relations",
                ))
            }
        };

        match self._flashcard_repository.delete_by_id(id).await {
            Ok(i) => i,
            Err(_) => {
                return Err(ApplicationError::business_rule(
                    "Failed to delete flashcard",
                ))
            }
        };

        match self
            ._flashcard_file_repository
            .delete_by_id(flashcard.file_id)
            .await
        {
            Ok(i) => Ok(i),
            Err(_) => {
                return Err(ApplicationError::business_rule(
                    "Failed to delete flashcard file",
                ))
            }
        }
    }

    async fn toggle_flashcard_active(
        &self,
        id: i32,
        updated_by_id: i32,
    ) -> Result<bool, ApplicationError> {
        let existing = match self._flashcard_repository.get_by_id(id).await {
            Some(f) => f,
            None => {
                return Err(ApplicationError::not_found(
                    "Flashcard not found",
                    id.to_string(),
                ))
            }
        };

        let new_status = !existing.is_actived;
        let updating = FlashcardModel {
            id: existing.id,
            name: existing.name,
            description: existing.description,
            sub_description: existing.sub_description,
            file_id: existing.file_id,
            created_by_id: existing.created_by_id,
            updated_by_id,
            created_on: existing.created_on,
            updated_on: existing.updated_on,
            is_actived: new_status,
        };

        self._flashcard_repository
            .update(updating)
            .await
            .map_err(|_| ApplicationError::business_rule("Failed to toggle flashcard active status"))?;

        Ok(new_status)
    }
}
