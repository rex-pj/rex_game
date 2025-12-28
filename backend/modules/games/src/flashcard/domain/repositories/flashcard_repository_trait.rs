use crate::flashcard::domain::models::flashcard_model::FlashcardModel;
use rex_game_shared::domain::models::page_list_model::PageListModel;
use rex_game_shared::InfraError;
use std::future::Future;

pub trait FlashcardRepositoryTrait {
    fn get_list(
        &self,
        type_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageListModel<FlashcardModel>, InfraError>>;

    fn create(&self, flashcard: FlashcardModel) -> impl Future<Output = Result<i32, InfraError>>;

    fn update(&self, flashcard: FlashcardModel) -> impl Future<Output = Result<bool, InfraError>>;

    fn get_by_id(&self, id: i32) -> impl Future<Output = Option<FlashcardModel>>;

    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<u64, InfraError>>;
}
