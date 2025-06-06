use chrono::Utc;
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::flashcard_file_model::FlashcardFileModel,
    repositories::flashcard_file_repository_trait::FlashcardFileRepositoryTrait,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::sync::Arc;

use crate::entities::{flashcard_file, prelude::FlashcardFile};

#[derive(Clone)]
pub struct FlashcardFileRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl FlashcardFileRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl FlashcardFileRepositoryTrait for FlashcardFileRepository {
    async fn get_by_id(&self, id: i32) -> Result<FlashcardFileModel, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = FlashcardFile::find_by_id(id).one(db).await;
        let flashcard = match existing {
            Ok(f) => f,
            Err(_) => None,
        };

        match flashcard {
            Some(f) => Ok(FlashcardFileModel {
                file_name: f.file_name,
                content_type: f.content_type,
                data: f.data,
                created_by_id: f.created_by_id,
                updated_by_id: f.updated_by_id,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                id: f.id,
                name: f.name,
            }),
            None => Err(DomainError::new(
                ErrorType::NotFound,
                "Flashcard file not found",
                None,
            )),
        }
    }

    async fn create(&self, flashcard_file_req: FlashcardFileModel) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();

        let new_flashcard_file = flashcard_file::ActiveModel {
            name: Set(flashcard_file_req.name),
            file_name: Set(flashcard_file_req.file_name),
            content_type: Set(flashcard_file_req.content_type),
            data: Set(flashcard_file_req.data),
            created_by_id: Set(flashcard_file_req.created_by_id),
            updated_by_id: Set(flashcard_file_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            ..Default::default()
        };
        match FlashcardFile::insert(new_flashcard_file).exec(db).await {
            Ok(result) => Ok(result.last_insert_id),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    async fn update(&self, flashcard_file_req: FlashcardFileModel) -> Result<bool, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = FlashcardFile::find_by_id(flashcard_file_req.id)
            .one(db)
            .await;
        let flashcard_file_option = match existing {
            Ok(f) => f,
            Err(_) => None,
        };

        let mut flashcard_file: flashcard_file::ActiveModel = match flashcard_file_option {
            Some(f) => f.into(),
            None => {
                return Err(DomainError::new(
                    ErrorType::NotFound,
                    "Flashcard file not found",
                    None,
                ))
            }
        };

        flashcard_file.updated_by_id = Set(flashcard_file_req.updated_by_id);
        flashcard_file.updated_date = Set(Utc::now().fixed_offset());
        flashcard_file.file_name = Set(flashcard_file_req.file_name);
        flashcard_file.content_type = Set(flashcard_file_req.content_type);
        flashcard_file.data = Set(flashcard_file_req.data);
        flashcard_file.name = Set(flashcard_file_req.name);

        match flashcard_file.update(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    async fn delete_by_id(&self, id: i32) -> Result<u64, DomainError> {
        let db = self._db_connection.as_ref();
        match FlashcardFile::delete_by_id(id).exec(db).await {
            Ok(result) => Ok(result.rows_affected),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }
}
