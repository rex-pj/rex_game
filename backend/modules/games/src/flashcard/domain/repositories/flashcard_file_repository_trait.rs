use std::future::Future;
use rex_game_shared_kernel::domain::errors::domain_error::DomainError;
use crate::flashcard::domain::models::flashcard_file_model::FlashcardFileModel;

pub trait FlashcardFileRepositoryTrait {
    fn create(
        &self,
        flashcard_file_req: FlashcardFileModel,
    ) -> impl Future<Output = Result<i32, DomainError>>;

    fn update(
        &self,
        flashcard_file_req: FlashcardFileModel,
    ) -> impl Future<Output = Result<bool, DomainError>>;

    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<FlashcardFileModel, DomainError>>;

    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<u64, DomainError>>;
}
