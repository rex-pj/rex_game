use chrono::Utc;
use rex_game_domain::{
    entities::{
        flashcard::{self, Entity as Flashcard},
        flashcard_type, flashcard_type_relation,
        page_list::PageList,
    },
    repositories::flashcard_repository_trait::FlashcardRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, InsertResult, JoinType,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
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
    ) -> Result<PageList<flashcard::Model>, DbErr> {
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

        let total_count = query.clone().count(db).await?;
        let page_list = query.paginate(db, page_size).fetch_page(page - 1).await;
        match page_list {
            Ok(items) => {
                return Ok(PageList { items, total_count });
            }
            Err(err) => return Err(err),
        }
    }

    async fn get_by_id(&self, id: i32) -> Option<flashcard::Model> {
        let db = self._db_connection.as_ref();
        let existing = Flashcard::find_by_id(id).one(db).await;

        match existing {
            Ok(i) => i,
            Err(_) => None,
        }
    }

    async fn create(
        &self,
        mut flashcard: flashcard::ActiveModel,
    ) -> Result<InsertResult<flashcard::ActiveModel>, DbErr> {
        let db = self._db_connection.as_ref();

        flashcard.created_date = Set(Utc::now().fixed_offset());
        flashcard.updated_date = Set(Utc::now().fixed_offset());
        return Flashcard::insert(flashcard).exec(db).await;
    }

    async fn update(
        &self,
        mut flashcard: flashcard::ActiveModel,
    ) -> Result<flashcard::Model, DbErr> {
        let db = self._db_connection.as_ref();

        flashcard.updated_date = Set(Utc::now().fixed_offset());
        return Flashcard::update(flashcard).exec(db).await;
    }
}
