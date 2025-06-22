use crate::entities::{
    flashcard_type::{self, Entity as FlashcardType},
    flashcard_type_relation,
    prelude::FlashcardTypeRelation,
};
use chrono::Utc;
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::{flashcard_type_model::FlashcardTypeModel, page_list_model::PageListModel},
    repositories::flashcard_type_repository_trait::FlashcardTypeRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait, Set,
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
    async fn get_paged_list(
        &self,
        name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Result<PageListModel<FlashcardTypeModel>, DomainError> {
        let db = self._db_connection.as_ref();
        let mut query = FlashcardType::find();
        if let Some(i) = name {
            query = query.filter(Condition::all().add(flashcard_type::Column::Name.eq(i)))
        }

        query = query.order_by(flashcard_type::Column::UpdatedDate, sea_orm::Order::Desc);

        let total_count = match query.clone().count(db).await {
            Ok(count) => count,
            Err(err) => {
                return Err(DomainError::new(
                    ErrorType::DatabaseError,
                    err.to_string().as_str(),
                    None,
                ))
            }
        };
        let page_list = query
            .paginate(db, page_size)
            .fetch_page(page - 1)
            .await
            .map_err(|err| {
                DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
            });
        match page_list {
            Ok(items) => {
                let list = items
                    .into_iter()
                    .map(|i| FlashcardTypeModel {
                        id: i.id,
                        name: i.name,
                        description: i.description,
                        created_date: i.created_date.with_timezone(&Utc),
                        updated_date: i.updated_date.with_timezone(&Utc),
                        created_by_id: i.created_by_id,
                        updated_by_id: i.updated_by_id,
                        is_actived: i.is_actived,
                    })
                    .collect::<Vec<FlashcardTypeModel>>();
                return Ok(PageListModel {
                    items: list,
                    total_count,
                });
            }
            Err(err) => Err(err),
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<FlashcardTypeModel, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = FlashcardType::find_by_id(id).one(db).await;

        match existing {
            Ok(i) => match i {
                Some(f) => Ok(FlashcardTypeModel {
                    id: f.id,
                    name: f.name,
                    description: f.description,
                    created_date: f.created_date.with_timezone(&Utc),
                    updated_date: f.updated_date.with_timezone(&Utc),
                    created_by_id: f.created_by_id,
                    updated_by_id: f.updated_by_id,
                    is_actived: f.is_actived,
                }),
                None => Err(DomainError::new(
                    ErrorType::NotFound,
                    "Flashcard type not found",
                    None,
                )),
            },
            Err(err) => {
                return Err(DomainError::new(
                    ErrorType::DatabaseError,
                    err.to_string().as_str(),
                    None,
                ))
            }
        }
    }

    async fn get_by_flashcard_id(
        &self,
        flashcard_id: i32,
    ) -> Result<Vec<FlashcardTypeModel>, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = match FlashcardType::find()
            .join(
                JoinType::InnerJoin,
                flashcard_type::Relation::FlashcardTypeRelation.def(),
            )
            .filter(flashcard_type_relation::Column::FlashcardId.eq(flashcard_id))
            .all(db)
            .await
        {
            Ok(f) => f,
            Err(err) => {
                return Err(DomainError::new(
                    ErrorType::DatabaseError,
                    err.to_string().as_str(),
                    None,
                ));
            }
        };

        let flashcard_types = existing
            .into_iter()
            .map(|i| FlashcardTypeModel {
                id: i.id,
                name: i.name,
                description: i.description,
                created_date: i.created_date.with_timezone(&Utc),
                updated_date: i.updated_date.with_timezone(&Utc),
                created_by_id: i.created_by_id,
                updated_by_id: i.updated_by_id,
                is_actived: i.is_actived,
            })
            .collect::<Vec<FlashcardTypeModel>>();

        return Ok(flashcard_types);
    }

    async fn create(&self, flashcard_type_req: FlashcardTypeModel) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();

        let new_flashcard_type: flashcard_type::ActiveModel = flashcard_type::ActiveModel {
            name: Set(flashcard_type_req.name),
            description: Set(flashcard_type_req.description),
            created_by_id: Set(flashcard_type_req.created_by_id),
            updated_by_id: Set(flashcard_type_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            ..Default::default()
        };

        match FlashcardType::insert(new_flashcard_type).exec(db).await {
            Ok(result) => Ok(result.last_insert_id),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    async fn update(&self, flashcard_type_req: FlashcardTypeModel) -> Result<bool, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = FlashcardType::find_by_id(flashcard_type_req.id)
            .one(db)
            .await;
        let flashcard_type_option = match existing {
            Ok(f) => f,
            Err(_) => None,
        };

        let mut flashcard_type: flashcard_type::ActiveModel = match flashcard_type_option {
            Some(f) => f.into(),
            None => {
                return Err(DomainError::new(
                    ErrorType::NotFound,
                    "Flashcard file not found",
                    None,
                ))
            }
        };

        flashcard_type.updated_by_id = Set(flashcard_type_req.updated_by_id);
        flashcard_type.updated_date = Set(Utc::now().fixed_offset());
        flashcard_type.description = Set(flashcard_type_req.description);
        flashcard_type.name = Set(flashcard_type_req.name);

        match FlashcardType::update(flashcard_type).exec(db).await {
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
            .filter(flashcard_type_relation::Column::FlashcardTypeId.eq(id))
            .exec(db)
            .await
            .map_err(|err| {
                DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
            })?;
        match FlashcardType::delete_by_id(id).exec(db).await {
            Ok(result) => Ok(result.rows_affected),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }
}
