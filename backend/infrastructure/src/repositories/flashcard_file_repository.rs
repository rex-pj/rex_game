use rex_game_domain::{
    entities::{flashcard_file, flashcard_file::Entity as FlashcardFile},
    repositories::flashcard_file_repository_trait::FlashcardFileRepositoryTrait,
};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, InsertResult};
use std::sync::Arc;

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
    async fn get_by_id(&self, id: i32) -> Result<Option<flashcard_file::Model>, DbErr> {
        let db = self._db_connection.as_ref();
        let flashcard = FlashcardFile::find_by_id(id).one(db).await;

        return flashcard;
    }

    async fn create(
        &self,
        flashcard: flashcard_file::ActiveModel,
    ) -> Result<InsertResult<flashcard_file::ActiveModel>, DbErr> {
        let db = self._db_connection.as_ref();

        return FlashcardFile::insert(flashcard).exec(db).await;
    }
}
