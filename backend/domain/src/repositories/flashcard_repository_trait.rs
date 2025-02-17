use sea_orm::{DbErr, InsertResult};
use std::future::Future;

use crate::entities::flashcard;

pub trait FlashcardRepositoryTrait {
    fn get_list(
        &self,
        type_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<(Vec<flashcard::Model>, u64), DbErr>>;

    fn create(
        &self,
        flashcard: flashcard::ActiveModel,
    ) -> impl Future<Output = Result<InsertResult<flashcard::ActiveModel>, DbErr>>;

    fn update(
        &self,
        flashcard: flashcard::ActiveModel,
    ) -> impl Future<Output = Result<flashcard::Model, DbErr>>;

    fn get_by_id(&self, id: i32) -> impl Future<Output = Option<flashcard::Model>>;
}
