use std::future::Future;
use rex_game_shared_kernel::domain::{
    errors::domain_error::DomainError,
    models::page_list_model::PageListModel,
};
use crate::flashcard::domain::models::flashcard_model::FlashcardModel;

pub trait FlashcardRepositoryTrait {
    fn get_list(
        &self,
        type_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageListModel<FlashcardModel>, DomainError>>;

    fn create(&self, flashcard: FlashcardModel) -> impl Future<Output = Result<i32, DomainError>>;

    fn update(&self, flashcard: FlashcardModel) -> impl Future<Output = Result<bool, DomainError>>;

    fn get_by_id(&self, id: i32) -> impl Future<Output = Option<FlashcardModel>>;

    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<u64, DomainError>>;
}
