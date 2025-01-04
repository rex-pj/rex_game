use super::flashcard;
use sea_orm::DbErr;
use std::future::Future;

pub trait FlashcardRepositoryTrait {
    fn get_flashcards(
        &self,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<(Vec<flashcard::Model>, u64), DbErr>>;
}
