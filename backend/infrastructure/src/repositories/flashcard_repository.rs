use rex_game_domain::flashcards::{
    flashcard::{self, Entity as Flashcard},
    flashcard_repository_trait::FlashcardRepositoryTrait,
};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct FlashcardRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl FlashcardRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl FlashcardRepositoryTrait for FlashcardRepository {
    async fn get_flashcards(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<flashcard::Model>, u64), DbErr> {
        let db = self._db_connection.as_ref();
        let paginator = Flashcard::find().paginate(db, page_size);

        let num_pages = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
