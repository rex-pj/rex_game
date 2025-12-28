use crate::flashcard::domain::{
    models::flashcard_type_relation_model::FlashcardTypeRelationModel,
    repositories::flashcard_type_relation_repository_trait::FlashcardTypeRelationRepositoryTrait,
};
use rex_game_entities::entities::flashcard_type_relation::{
    self, Entity as FlashcardTypeRelation,
};
use chrono::Utc;
use rex_game_shared::InfraError;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
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
        mut flashcard_type_relations_req: Vec<FlashcardTypeRelationModel>,
    ) -> Result<i32, InfraError> {
        let db = self._db_connection.as_ref();

        let flashcard_type_relations =
            flashcard_type_relations_req
                .iter_mut()
                .map(|f| flashcard_type_relation::ActiveModel {
                    flashcard_id: Set(f.flashcard_id),
                    flashcard_type_id: Set(f.flashcard_type_id),
                    created_by_id: Set(f.created_by_id),
                    updated_by_id: Set(f.updated_by_id),
                    created_date: Set(Utc::now().fixed_offset()),
                    updated_date: Set(Utc::now().fixed_offset()),
                    ..Default::default()
                });

        let inserted = FlashcardTypeRelation::insert_many(flashcard_type_relations)
            .exec(db)
            .await
            .map_err(|err| {
                InfraError::database(err.to_string().as_str())
            })?;

        match inserted.last_insert_id {
            None => Err(InfraError::database("Failed to create flashcard type relations")),
            last_insert_id => match last_insert_id {
                Some(id) => Ok(id),
                None => Ok(0),
            },
        }
    }

    async fn get_by_flashcard_id(
        &self,
        flashcard_id: i32,
    ) -> Result<Vec<FlashcardTypeRelationModel>, InfraError> {
        let db = self._db_connection.as_ref();

        let existing = FlashcardTypeRelation::find()
            .filter(flashcard_type_relation::Column::FlashcardId.eq(flashcard_id))
            .all(db)
            .await
            .map_err(|err| InfraError::database(err.to_string()))?;

        let flashcard_type_relations = existing
            .into_iter()
            .map(|f| FlashcardTypeRelationModel {
                id: f.id,
                flashcard_id: f.flashcard_id,
                flashcard_type_id: f.flashcard_type_id,
                created_by_id: f.created_by_id,
                updated_by_id: f.updated_by_id,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
            })
            .collect::<Vec<FlashcardTypeRelationModel>>();

        return Ok(flashcard_type_relations);
    }

    async fn delete_by_ids(&self, ids: Vec<i32>) -> Result<u64, InfraError> {
        let db = self._db_connection.as_ref();

        FlashcardTypeRelation::delete_many()
            .filter(flashcard_type_relation::Column::Id.is_in(ids))
            .exec(db)
            .await
            .map(|result| result.rows_affected)
            .map_err(|err| InfraError::database(err.to_string()))
    }

    async fn delete_by_flashcard_id(&self, flashcard_id: i32) -> Result<u64, InfraError> {
        let db = self._db_connection.as_ref();

        FlashcardTypeRelation::delete_many()
            .filter(flashcard_type_relation::Column::FlashcardId.eq(flashcard_id))
            .exec(db)
            .await
            .map(|result| result.rows_affected)
            .map_err(|err| InfraError::database(err.to_string()))
    }
}
