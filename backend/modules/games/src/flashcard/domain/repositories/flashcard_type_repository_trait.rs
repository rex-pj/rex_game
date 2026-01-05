use crate::flashcard::domain::models::flashcard_type_model::FlashcardTypeModel;
use rex_game_shared::domain::models::page_list_model::PageListModel;
use rex_game_shared::InfraError;
use std::future::Future;

pub trait FlashcardTypeRepositoryTrait {
    fn get_paged_list(
        &self,
        name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageListModel<FlashcardTypeModel>, InfraError>>;

    fn create(
        &self,
        flashcard_type: FlashcardTypeModel,
    ) -> impl Future<Output = Result<i32, InfraError>>;

    fn update(
        &self,
        flashcard_type: FlashcardTypeModel,
    ) -> impl Future<Output = Result<bool, InfraError>>;

    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<FlashcardTypeModel, InfraError>>;
    fn get_by_flashcard_id(
        &self,
        flashcard_id: i32,
    ) -> impl Future<Output = Result<Vec<FlashcardTypeModel>, InfraError>>;
    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<u64, InfraError>>;
}
