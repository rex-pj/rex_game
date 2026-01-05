use crate::flashcard::domain::{
    models::flashcard_file_model::FlashcardFileModel,
    repositories::flashcard_file_repository_trait::FlashcardFileRepositoryTrait,
};
use chrono::Utc;
use rex_game_shared::InfraError;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use std::sync::Arc;

use rex_game_entities::entities::flashcard_file::{
    self, Entity as FlashcardFile,
};

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
    async fn get_by_id(&self, id: i32) -> Result<FlashcardFileModel, InfraError> {
        let db = self._db_connection.as_ref();

        let flashcard = FlashcardFile::find_by_id(id).one(db).await.map_err(|err| {
            InfraError::database(err.to_string().as_str())
        })?;

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
                is_actived: f.is_actived,
            }),
            None => Err(InfraError::not_found("FlashcardFile", id.to_string())),
        }
    }

    async fn create(&self, flashcard_file_req: FlashcardFileModel) -> Result<i32, InfraError> {
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
            is_actived: Set(true),
            ..Default::default()
        };
        FlashcardFile::insert(new_flashcard_file)
            .exec(db)
            .await
            .map(|result| result.last_insert_id)
            .map_err(|err| InfraError::database(err.to_string()))
    }

    async fn update(&self, flashcard_file_req: FlashcardFileModel) -> Result<bool, InfraError> {
        let db = self._db_connection.as_ref();

        // Find existing flashcard file
        let flashcard_file_option = FlashcardFile::find_by_id(flashcard_file_req.id)
            .one(db)
            .await
            .map_err(|err| {
                InfraError::database(err.to_string().as_str())
            })?;

        let mut flashcard_file: flashcard_file::ActiveModel = match flashcard_file_option {
            Some(f) => f.into(),
            None => {
                return Err(InfraError::not_found("FlashcardFile", flashcard_file_req.id.to_string()))
            }
        };

        flashcard_file.updated_by_id = Set(flashcard_file_req.updated_by_id);
        flashcard_file.updated_date = Set(Utc::now().fixed_offset());
        flashcard_file.file_name = Set(flashcard_file_req.file_name);
        flashcard_file.content_type = Set(flashcard_file_req.content_type);
        flashcard_file.data = Set(flashcard_file_req.data);
        flashcard_file.name = Set(flashcard_file_req.name);

        FlashcardFile::update(flashcard_file)
            .exec(db)
            .await
            .map(|_| true)
            .map_err(|err| {
                eprintln!("Database Error Details:");
                eprintln!("  Error: {:?}", err);
                eprintln!("  Error String: {}", err.to_string());
                InfraError::database(err.to_string())
            })
    }

    async fn delete_by_id(&self, id: i32) -> Result<u64, InfraError> {
        let db = self._db_connection.as_ref();
        FlashcardFile::delete_by_id(id)
            .exec(db)
            .await
            .map(|result| result.rows_affected)
            .map_err(|err| InfraError::database(err.to_string()))
    }
}
