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

use super::{
    flashcard_creation_dto::FlashcardCreationDto, flashcard_dto::FlashcardDto,
    flashcard_updation_dto::FlashcardUpdationDto, flashcard_usecase_trait::FlashcardUseCaseTrait,
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
    async fn get_flashcards<'a>(
        &'a self,
        type_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Option<Vec<FlashcardDto>> {
        let existing = self
            ._flashcard_repository
            .get_list(type_name, page, page_size)
            .await;
        if let Ok(i) = existing {
            Some(
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
            )
        } else {
            None
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

    async fn create_flashcard<'a>(&'a self, flashcard_req: FlashcardCreationDto) -> Option<i32> {
        let active_flashcard_file = flashcard_file::ActiveModel {
            name: Set(Some(flashcard_req.name.clone())),
            file_name: Set(flashcard_req.file_name),
            content_type: Set(flashcard_req.content_type),
            data: Set(flashcard_req.image_data.unwrap()),
            ..Default::default()
        };

        let new_file = self
            ._flashcard_file_repository
            .create(active_flashcard_file)
            .await;
        match new_file {
            Ok(file) => {
                let active_flashcard = flashcard::ActiveModel {
                    name: Set(flashcard_req.name),
                    description: Set(flashcard_req.description),
                    sub_description: Set(flashcard_req.sub_description),
                    file_id: Set(file.last_insert_id),
                    ..Default::default()
                };
                let created = self._flashcard_repository.create(active_flashcard).await;
                match created {
                    Err(_) => None,
                    Ok(i) => {
                        let mut active_type_relations: Vec<flashcard_type_relation::ActiveModel> =
                            Vec::new();
                        for type_relation_id in flashcard_req.type_ids.iter() {
                            let active_flashcard_type_relation =
                                flashcard_type_relation::ActiveModel {
                                    flashcard_id: Set(i.last_insert_id),
                                    flashcard_type_id: Set(*type_relation_id),
                                    ..Default::default()
                                };
                            active_type_relations.push(active_flashcard_type_relation);
                        }

                        let type_relations_created = self
                            ._flashcard_type_relation_repository
                            .create(active_type_relations)
                            .await;

                        match type_relations_created {
                            Err(_) => return None,
                            Ok(_) => Some(i.last_insert_id),
                        }
                    }
                }
            }
            Err(_) => None,
        }
    }

    async fn update_flashcard<'a>(
        &'a self,
        id: i32,
        flashcard_req: FlashcardUpdationDto,
    ) -> Option<i32> {
        let existing_flashcard = self._flashcard_repository.get_by_id(id).await;
        if let Some(flashcard) = existing_flashcard {
            if let Some(req_file) = flashcard_req.image_data {
                let existing_file = self
                    ._flashcard_file_repository
                    .get_by_id(flashcard.file_id)
                    .await;

                if let Some(f) = existing_file {
                    let mut updating: flashcard_file::ActiveModel = f.into();
                    updating.content_type = Set(flashcard_req.content_type.unwrap());
                    updating.file_name = Set(flashcard_req.file_name.unwrap());
                    updating.name = Set(flashcard_req.name.clone());
                    updating.data = Set(req_file);

                    let updated = self._flashcard_file_repository.update(updating).await;
                    if let Ok(_) = updated {}
                }
            }

            let mut updating_flashcard: flashcard::ActiveModel = flashcard.into();
            if let Some(name) = flashcard_req.name {
                updating_flashcard.name = Set(name);
            }

            if let Some(description) = flashcard_req.description {
                updating_flashcard.description = Set(Some(description));
            }

            if let Some(sub_description) = flashcard_req.sub_description {
                updating_flashcard.sub_description = Set(Some(sub_description));
            }

            if let Some(req_type_ids) = flashcard_req.type_ids {
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
                        let deleted = self
                            ._flashcard_type_relation_repository
                            .delete_by_ids(unused_relation_type_ids)
                            .await;

                        if let Ok(_) = deleted {}
                    }
                }
                let mut existing_type_relations: Vec<flashcard_type_relation::ActiveModel> =
                    Vec::new();
                for type_relation_id in req_type_ids.iter() {
                    let active_type_relation = flashcard_type_relation::ActiveModel {
                        flashcard_id: Set(id),
                        flashcard_type_id: Set(*type_relation_id),
                        ..Default::default()
                    };
                    existing_type_relations.push(active_type_relation);
                }

                let type_relations_created = self
                    ._flashcard_type_relation_repository
                    .create(existing_type_relations)
                    .await;

                if let Ok(_) = type_relations_created {}
            }
        }

        Some(id)
    }

    async fn get_image_by_file_id<'a>(&'a self, file_id: i32) -> Option<Vec<u8>> {
        let existing = self._flashcard_file_repository.get_by_id(file_id).await;
        match existing {
            Some(f) => Some(f.data),
            None => None,
        }
    }
}
