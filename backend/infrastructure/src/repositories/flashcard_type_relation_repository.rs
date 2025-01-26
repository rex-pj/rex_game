use rex_game_domain::{
    entities::{flashcard_type_relation, flashcard_type_relation::Entity as FlashcardTypeRelation},
    repositories::flashcard_type_relation_repository_trait::FlashcardTypeRelationRepositoryTrait,
};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, InsertResult};
use std::sync::Arc;

#[derive(Clone)]
pub struct FlashcardTypeRelationRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl FlashcardTypeRelationRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl FlashcardTypeRelationRepositoryTrait for FlashcardTypeRelationRepository {
    async fn create(
        &self,
        flashcard_type_relations: Vec<flashcard_type_relation::ActiveModel>,
    ) -> Result<InsertResult<flashcard_type_relation::ActiveModel>, DbErr> {
        let db = self._db_connection.as_ref();

        return FlashcardTypeRelation::insert_many(flashcard_type_relations)
            .exec(db)
            .await;
    }
}
