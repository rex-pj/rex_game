use sea_orm::{DbErr, InsertResult};
use std::future::Future;

use crate::entities::flashcard_file;

pub trait FlashcardFileRepositoryTrait {
    fn create(
        &self,
        flashcard: flashcard_file::ActiveModel,
    ) -> impl Future<Output = Result<InsertResult<flashcard_file::ActiveModel>, DbErr>>;

    fn get_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<Option<flashcard_file::Model>, DbErr>>;
}
