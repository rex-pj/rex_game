use chrono::Utc;
use rex_game_domain::{
    entities::{flashcard, flashcard_file, flashcard_type_relation},
    repositories::{
        flashcard_file_repository_trait::FlashcardFileRepositoryTrait,
        flashcard_repository_trait::FlashcardRepositoryTrait,
        flashcard_type_relation_repository_trait::FlashcardTypeRelationRepositoryTrait,
    },
};
use sea_orm::Set;

use crate::{
    errors::application_error::{ApplicationError, ErrorKind},
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
                ErrorKind::DatabaseError,
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
        let active_flashcard_file = flashcard_file::ActiveModel {
            name: Set(Some(flashcard_req.name.clone())),
            file_name: Set(flashcard_req.file_name),
            content_type: Set(flashcard_req.content_type),
            data: Set(flashcard_req.image_data.unwrap()),
            created_by_id: Set(flashcard_req.created_by_id),
            updated_by_id: Set(flashcard_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            ..Default::default()
        };

        let new_file = match self
            ._flashcard_file_repository
            .create(active_flashcard_file)
            .await
        {
            Ok(file) => file,
            Err(_) => {
                return Err(ApplicationError::new(
                    ErrorKind::DatabaseError,
                    "Failed to create flashcard file",
                    None,
                ))
            }
        };
        let active_flashcard = flashcard::ActiveModel {
            name: Set(flashcard_req.name),
            description: Set(flashcard_req.description),
            sub_description: Set(flashcard_req.sub_description),
            file_id: Set(new_file.last_insert_id),
            created_by_id: Set(flashcard_req.created_by_id),
            updated_by_id: Set(flashcard_req.updated_by_id),
            ..Default::default()
        };
        let created = match self._flashcard_repository.create(active_flashcard).await {
            Ok(i) => i,
            Err(_) => {
                return Err(ApplicationError::new(
                    ErrorKind::DatabaseError,
                    "Failed to create flashcard",
                    None,
                ))
            }
        };

        let mut active_type_relations: Vec<flashcard_type_relation::ActiveModel> = Vec::new();
        for type_relation_id in flashcard_req.type_ids.iter() {
            let active_flashcard_type_relation = flashcard_type_relation::ActiveModel {
                flashcard_id: Set(created.last_insert_id),
                flashcard_type_id: Set(*type_relation_id),
                created_by_id: Set(flashcard_req.created_by_id),
                updated_by_id: Set(flashcard_req.updated_by_id),
                ..Default::default()
            };
            active_type_relations.push(active_flashcard_type_relation);
        }

        let type_relations_created = self
            ._flashcard_type_relation_repository
            .create(active_type_relations)
            .await;

        match type_relations_created {
            Ok(_) => Ok(created.last_insert_id),
            Err(_) => {
                return Err(ApplicationError::new(
                    ErrorKind::DatabaseError,
                    "Failed to create flashcard type relation",
                    None,
                ))
            }
        }
    }

    async fn update_flashcard<'a>(
        &'a self,
        id: i32,
        flashcard_req: FlashcardUpdationDto,
    ) -> Result<(), ApplicationError> {
        let existing_flashcard = match self._flashcard_repository.get_by_id(id).await {
            Some(flashcard) => flashcard,
            None => {
                return Err(ApplicationError::new(
                    ErrorKind::NotFound,
                    "Flashcard not found",
                    None,
                ))
            }
        };

        // Updating file
        if let Some(req_file) = flashcard_req.image_data {
            let existing_file = match self
                ._flashcard_file_repository
                .get_by_id(existing_flashcard.file_id)
                .await
            {
                Some(existing_model) => existing_model,
                None => {
                    return Err(ApplicationError::new(
                        ErrorKind::NotFound,
                        "Flashcard file not found",
                        None,
                    ))
                }
            };

            let mut updating_file: flashcard_file::ActiveModel = existing_file.into();
            updating_file.content_type = Set(flashcard_req.content_type.unwrap());
            updating_file.file_name = Set(flashcard_req.file_name.unwrap());
            updating_file.name = Set(flashcard_req.name.clone());
            updating_file.updated_by_id = Set(flashcard_req.updated_by_id);
            updating_file.data = Set(req_file);

            match self._flashcard_file_repository.update(updating_file).await {
                Ok(_) => {}
                Err(_) => {
                    return Err(ApplicationError::new(
                        ErrorKind::NotFound,
                        "Flashcard file update failed",
                        None,
                    ))
                }
            };
        }

        // Updating flashcard information
        let mut updating_flashcard: flashcard::ActiveModel = existing_flashcard.into();

        updating_flashcard.updated_date = Set(Utc::now().fixed_offset());
        if let Some(name) = flashcard_req.name {
            updating_flashcard.name = Set(name);
        }

        if let Some(description) = flashcard_req.description {
            updating_flashcard.description = Set(Some(description));
        }

        if let Some(sub_description) = flashcard_req.sub_description {
            updating_flashcard.sub_description = Set(Some(sub_description));
        }

        if let Some(updated_by_id) = flashcard_req.updated_by_id {
            updating_flashcard.updated_by_id = Set(Some(updated_by_id));
        }

        match self._flashcard_repository.update(updating_flashcard).await {
            Ok(updated) => updated,
            Err(_) => {
                return Err(ApplicationError::new(
                    ErrorKind::DatabaseError,
                    "Failed to update flashcard",
                    None,
                ))
            }
        };

        match flashcard_req.type_ids {
            Some(req_type_ids) => {
                let existing_types = self
                    ._flashcard_type_relation_repository
                    .get_by_flashcard_id(id)
                    .await;

                if let Ok(types) = existing_types {
                    let unused_relation_type_ids: Vec<i32> = types
                        .iter()
                        .filter(|p| !req_type_ids.contains(&p.flashcard_type_id))
                        .map(|f| f.flashcard_type_id)
                        .collect();

                    if unused_relation_type_ids.len() > 0 {
                        match self
                            ._flashcard_type_relation_repository
                            .delete_by_ids(unused_relation_type_ids)
                            .await
                        {
                            Ok(_) => {}
                            Err(_) => {
                                return Err(ApplicationError::new(
                                    ErrorKind::DatabaseError,
                                    "Failed to delete flashcard type relation",
                                    None,
                                ))
                            }
                        };
                    }
                }

                let mut existing_type_relations: Vec<flashcard_type_relation::ActiveModel> =
                    Vec::new();
                for type_relation_id in req_type_ids.iter() {
                    let active_type_relation = flashcard_type_relation::ActiveModel {
                        flashcard_id: Set(id),
                        flashcard_type_id: Set(*type_relation_id),
                        updated_by_id: Set(flashcard_req.updated_by_id),
                        ..Default::default()
                    };
                    existing_type_relations.push(active_type_relation);
                }

                match self
                    ._flashcard_type_relation_repository
                    .create(existing_type_relations)
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => {
                        return Err(ApplicationError::new(
                            ErrorKind::DatabaseError,
                            "Failed to update flashcard type relation",
                            None,
                        ))
                    }
                }
            }
            None => Ok(()),
        }
    }

    async fn get_image_by_file_id<'a>(
        &'a self,
        file_id: i32,
    ) -> Result<FlashcardFileDto, ApplicationError> {
        let existing = self._flashcard_file_repository.get_by_id(file_id).await;
        match existing {
            Some(f) => Ok(FlashcardFileDto {
                id: f.id,
                name: f.name,
                file_name: f.file_name,
                content_type: f.content_type,
                data: f.data,
            }),
            None => Err(ApplicationError::new(
                ErrorKind::NotFound,
                "File not found",
                None,
            )),
        }
    }
}
