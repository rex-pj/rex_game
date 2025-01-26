use rex_game_domain::{
    entities::{
        flashcard::{self, Entity as Flashcard},
        flashcard_type, flashcard_type_relation,
    },
    repositories::flashcard_repository_trait::FlashcardRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, InsertResult, JoinType,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};
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
    async fn get_list(
        &self,
        type_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<flashcard::Model>, u64), DbErr> {
        let db = self._db_connection.as_ref();
        let mut query = Flashcard::find().join(
            JoinType::InnerJoin,
            flashcard::Relation::FlashcardTypeRelation.def(),
        );
        if let Some(i) = type_name {
            query = query
                .join(
                    JoinType::LeftJoin,
                    flashcard_type_relation::Relation::FlashcardType.def(),
                )
                .filter(Condition::all().add(flashcard_type::Column::Name.eq(i)))
        }

        query = query.order_by(flashcard::Column::UpdatedDate, sea_orm::Order::Desc);

        let paginator = query.paginate(db, page_size);

        let num_pages = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<flashcard::Model>, DbErr> {
        let db = self._db_connection.as_ref();
        let flashcard = Flashcard::find_by_id(id).one(db).await;

        return flashcard;
    }

    async fn create(
        &self,
        flashcard: flashcard::ActiveModel,
    ) -> Result<InsertResult<flashcard::ActiveModel>, DbErr> {
        let db = self._db_connection.as_ref();

        return Flashcard::insert(flashcard).exec(db).await;
    }
}
