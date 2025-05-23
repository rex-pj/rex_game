use sea_orm::{DbErr, DeleteResult, InsertResult};
use std::future::Future;

use crate::entities::{flashcard_type, page_list::PageList};

pub trait FlashcardTypeRepositoryTrait {
    fn get_paged_list(
        &self,
        name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageList<flashcard_type::Model>, DbErr>>;

    fn create(
        &self,
        flashcard_type: flashcard_type::ActiveModel,
    ) -> impl Future<Output = Result<InsertResult<flashcard_type::ActiveModel>, DbErr>>;

    fn update(
        &self,
        flashcard_type: flashcard_type::ActiveModel,
    ) -> impl Future<Output = Result<flashcard_type::Model, DbErr>>;

    fn get_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<Option<flashcard_type::Model>, DbErr>>;
    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<DeleteResult, DbErr>>;
}
