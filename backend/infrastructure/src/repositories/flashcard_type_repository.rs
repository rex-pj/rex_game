use chrono::Utc;
use rex_game_domain::{
    entities::flashcard_type::{self, Entity as FlashcardType},
    repositories::flashcard_type_repository_trait::FlashcardTypeRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, InsertResult, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct FlashcardTypeRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl FlashcardTypeRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl FlashcardTypeRepositoryTrait for FlashcardTypeRepository {
    async fn get_list(
        &self,
        name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<flashcard_type::Model>, u64), DbErr> {
        let db = self._db_connection.as_ref();
        let mut query = FlashcardType::find();
        if let Some(i) = name {
            query = query.filter(Condition::all().add(flashcard_type::Column::Name.eq(i)))
        }

        query = query.order_by(flashcard_type::Column::UpdatedDate, sea_orm::Order::Desc);

        let paginator = query.paginate(db, page_size);

        let num_pages = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<flashcard_type::Model>, DbErr> {
        let db = self._db_connection.as_ref();
        let flashcard_type = FlashcardType::find_by_id(id).one(db).await;

        return flashcard_type;
    }

    async fn create(
        &self,
        mut flashcard_type: flashcard_type::ActiveModel,
    ) -> Result<InsertResult<flashcard_type::ActiveModel>, DbErr> {
        let db = self._db_connection.as_ref();

        flashcard_type.created_date = Set(Utc::now().fixed_offset());
        flashcard_type.updated_date = Set(Utc::now().fixed_offset());
        return FlashcardType::insert(flashcard_type).exec(db).await;
    }

    async fn update(
        &self,
        mut flashcard_type: flashcard_type::ActiveModel,
    ) -> Result<flashcard_type::Model, DbErr> {
        let db = self._db_connection.as_ref();

        flashcard_type.updated_date = Set(Utc::now().fixed_offset());
        return FlashcardType::update(flashcard_type).exec(db).await;
    }
}
