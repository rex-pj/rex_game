use sea_orm::{DbErr, InsertResult};
use std::future::Future;

use crate::entities::flashcard_type_relation;

pub trait FlashcardTypeRelationRepositoryTrait {
    fn create(
        &self,
        flashcard_type_relations: Vec<flashcard_type_relation::ActiveModel>,
    ) -> impl Future<Output = Result<InsertResult<flashcard_type_relation::ActiveModel>, DbErr>>;
}
