use std::future::Future;
use rex_game_shared::InfraError;
use crate::flashcard::domain::models::flashcard_file_model::FlashcardFileModel;

pub trait FlashcardFileRepositoryTrait {
    fn create(
        &self,
        flashcard_file_req: FlashcardFileModel,
    ) -> impl Future<Output = Result<i32, InfraError>>;

    fn update(
        &self,
        flashcard_file_req: FlashcardFileModel,
    ) -> impl Future<Output = Result<bool, InfraError>>;

    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<FlashcardFileModel, InfraError>>;

    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<u64, InfraError>>;
}
