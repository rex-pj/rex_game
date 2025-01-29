use sea_orm::{DbErr, DeleteResult, InsertResult};
use std::future::Future;

use crate::entities::flashcard_type_relation;

pub trait FlashcardTypeRelationRepositoryTrait {
    fn create(
        &self,
        flashcard_type_relations: Vec<flashcard_type_relation::ActiveModel>,
    ) -> impl Future<Output = Result<InsertResult<flashcard_type_relation::ActiveModel>, DbErr>>;

    fn get_by_flashcard_id(
        &self,
        flashcard_id: i32,
    ) -> impl Future<Output = Result<Vec<flashcard_type_relation::Model>, DbErr>>;

    fn delete_by_ids(&self, ids: Vec<i32>) -> impl Future<Output = Result<DeleteResult, DbErr>>;
}
