use crate::entities::{
    flashcard::{self, Entity as Flashcard},
    prelude::FlashcardTypeRelation,
};
use chrono::Utc;
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::{flashcard_model::FlashcardModel, page_list_model::PageListModel},
    repositories::flashcard_repository_trait::FlashcardRepositoryTrait,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};
use std::sync::Arc;

use crate::entities::{flashcard_type, flashcard_type_relation};

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
    ) -> Result<PageListModel<FlashcardModel>, DomainError> {
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

        query = query
            .order_by(flashcard::Column::UpdatedDate, sea_orm::Order::Desc)
            .distinct();

        let paginator = query.paginate(db, page_size);
        let total_count = paginator.num_items().await.map_err(|err| {
            DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
        })?;

        let page_list = paginator.fetch_page(page - 1).await.map_err(|err| {
            DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
        })?;

        let items = page_list
            .into_iter()
            .map(|i| FlashcardModel {
                id: i.id,
                name: i.name,
                description: i.description,
                sub_description: i.sub_description,
                created_date: i.created_date.with_timezone(&Utc),
                updated_date: i.updated_date.with_timezone(&Utc),
                created_by_id: i.created_by_id,
                updated_by_id: i.updated_by_id,
                file_id: i.file_id,
                is_actived: i.is_actived,
            })
            .collect::<Vec<FlashcardModel>>();
        return Ok(PageListModel { items, total_count });
    }

    async fn get_by_id(&self, id: i32) -> Option<FlashcardModel> {
        let db = self._db_connection.as_ref();
        let existing = Flashcard::find_by_id(id).one(db).await;

        match existing {
            Ok(i) => match i {
                Some(f) => Some(FlashcardModel {
                    id: f.id,
                    name: f.name,
                    description: f.description,
                    sub_description: f.sub_description,
                    created_date: f.created_date.with_timezone(&Utc),
                    updated_date: f.updated_date.with_timezone(&Utc),
                    created_by_id: f.created_by_id,
                    updated_by_id: f.updated_by_id,
                    file_id: f.file_id,
                    is_actived: f.is_actived,
                }),
                None => None,
            },
            Err(_) => None,
        }
    }

    async fn create(&self, flashcard: FlashcardModel) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();

        let new_flashcard = flashcard::ActiveModel {
            name: Set(flashcard.name),
            description: Set(flashcard.description),
            sub_description: Set(flashcard.sub_description),
            file_id: Set(flashcard.file_id),
            created_by_id: Set(flashcard.created_by_id),
            updated_by_id: Set(flashcard.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            ..Default::default()
        };

        match Flashcard::insert(new_flashcard).exec(db).await {
            Ok(result) => Ok(result.last_insert_id),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    async fn update(&self, flashcard_req: FlashcardModel) -> Result<bool, DomainError> {
        let db = self._db_connection.as_ref();

        let existing = Flashcard::find_by_id(flashcard_req.id).one(db).await;
        let flashcard_option = match existing {
            Ok(f) => f,
            Err(_) => None,
        };

        let mut flashcard: flashcard::ActiveModel = match flashcard_option {
            Some(f) => f.into(),
            None => {
                return Err(DomainError::new(
                    ErrorType::NotFound,
                    "Flashcard file not found",
                    None,
                ))
            }
        };

        flashcard.updated_by_id = Set(flashcard_req.updated_by_id);
        flashcard.updated_date = Set(Utc::now().fixed_offset());
        flashcard.description = Set(flashcard_req.description);
        flashcard.sub_description = Set(flashcard_req.sub_description);
        flashcard.file_id = Set(flashcard_req.file_id);
        flashcard.name = Set(flashcard_req.name);

        match flashcard.update(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    async fn delete_by_id(&self, id: i32) -> Result<u64, DomainError> {
        let db = self._db_connection.as_ref();
        FlashcardTypeRelation::delete_many()
            .filter(flashcard_type_relation::Column::FlashcardId.eq(id))
            .exec(db)
            .await
            .map_err(|err| {
                DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
            })?;
        match Flashcard::delete_by_id(id).exec(db).await {
            Ok(result) => Ok(result.rows_affected),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }
}
