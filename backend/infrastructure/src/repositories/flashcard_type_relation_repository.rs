use chrono::Utc;
use rex_game_domain::{
    entities::{flashcard_type_relation, flashcard_type_relation::Entity as FlashcardTypeRelation},
    repositories::flashcard_type_relation_repository_trait::FlashcardTypeRelationRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, DeleteResult, EntityTrait, InsertResult,
    QueryFilter, Set,
};
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
        mut flashcard_type_relations: Vec<flashcard_type_relation::ActiveModel>,
    ) -> Result<InsertResult<flashcard_type_relation::ActiveModel>, DbErr> {
        let db = self._db_connection.as_ref();

        flashcard_type_relations.iter_mut().for_each(|f| {
            f.created_date = Set(Utc::now().fixed_offset());
            f.updated_date = Set(Utc::now().fixed_offset());
        });
        return FlashcardTypeRelation::insert_many(flashcard_type_relations)
            .exec(db)
            .await;
    }

    async fn get_by_flashcard_id(
        &self,
        flashcard_id: i32,
    ) -> Result<Vec<flashcard_type_relation::Model>, DbErr> {
        let db = self._db_connection.as_ref();

        return FlashcardTypeRelation::find()
            .filter(
                Condition::all().add(flashcard_type_relation::Column::FlashcardId.eq(flashcard_id)),
            )
            .all(db)
            .await;
    }

    async fn delete_by_ids(&self, ids: Vec<i32>) -> Result<DeleteResult, DbErr> {
        let db = self._db_connection.as_ref();

        FlashcardTypeRelation::delete_many()
            .filter(flashcard_type_relation::Column::Id.is_in(ids))
            .exec(db)
            .await
    }
}
