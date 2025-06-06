use std::future::Future;

use crate::{
    errors::domain_error::DomainError,
    models::{flashcard_type_model::FlashcardTypeModel, page_list_model::PageListModel},
};

pub trait FlashcardTypeRepositoryTrait {
    fn get_paged_list(
        &self,
        name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageListModel<FlashcardTypeModel>, DomainError>>;

    fn create(
        &self,
        flashcard_type: FlashcardTypeModel,
    ) -> impl Future<Output = Result<i32, DomainError>>;

    fn update(
        &self,
        flashcard_type: FlashcardTypeModel,
    ) -> impl Future<Output = Result<bool, DomainError>>;

    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<FlashcardTypeModel, DomainError>>;
    fn get_by_flashcard_id(
        &self,
        flashcard_id: i32,
    ) -> impl Future<Output = Result<Vec<FlashcardTypeModel>, DomainError>>;
    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<u64, DomainError>>;
}
