use crate::flashcard::domain::{
    models::flashcard_type_model::FlashcardTypeModel,
    repositories::flashcard_type_repository_trait::FlashcardTypeRepositoryTrait,
};
use chrono::Utc;
use rex_game_entities::entities::{
    flashcard_type::{self, Entity as FlashcardType},
    flashcard_type_relation::{self, Entity as FlashcardTypeRelation},
};
use rex_game_shared::domain::models::page_list_model::PageListModel;
use rex_game_shared::InfraError;
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
    ) -> Result<PageListModel<FlashcardTypeModel>, InfraError> {
        let db = self._db_connection.as_ref();
        let mut query = FlashcardType::find();
        if let Some(i) = name {
            query = query.filter(Condition::all().add(flashcard_type::Column::Name.eq(i)))
        }

        query = query.order_by(flashcard_type::Column::UpdatedOn, sea_orm::Order::Desc);

        let total_count = match query.clone().count(db).await {
            Ok(count) => count,
            Err(err) => return Err(InfraError::database(err.to_string().as_str())),
        };
        let page_list = query
            .paginate(db, page_size)
            .fetch_page(page - 1)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()));
        match page_list {
            Ok(items) => {
                let list = items
                    .into_iter()
                    .map(|i| FlashcardTypeModel {
                        id: i.id,
                        name: i.name,
                        description: i.description,
                        created_on: i.created_on.with_timezone(&Utc),
                        updated_on: i.updated_on.with_timezone(&Utc),
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

    async fn get_by_id(&self, id: i32) -> Result<FlashcardTypeModel, InfraError> {
        let db = self._db_connection.as_ref();
        let existing = FlashcardType::find_by_id(id).one(db).await;

        match existing {
            Ok(i) => match i {
                Some(f) => Ok(FlashcardTypeModel {
                    id: f.id,
                    name: f.name,
                    description: f.description,
                    created_on: f.created_on.with_timezone(&Utc),
                    updated_on: f.updated_on.with_timezone(&Utc),
                    created_by_id: f.created_by_id,
                    updated_by_id: f.updated_by_id,
                    is_actived: f.is_actived,
                }),
                None => Err(InfraError::not_found(
                    "Flashcard type not found",
                    id.to_string(),
                )),
            },
            Err(err) => return Err(InfraError::database(err.to_string().as_str())),
        }
    }

    async fn get_by_flashcard_id(
        &self,
        flashcard_id: i32,
    ) -> Result<Vec<FlashcardTypeModel>, InfraError> {
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
                return Err(InfraError::database(err.to_string().as_str()));
            }
        };

        let flashcard_types = existing
            .into_iter()
            .map(|i| FlashcardTypeModel {
                id: i.id,
                name: i.name,
                description: i.description,
                created_on: i.created_on.with_timezone(&Utc),
                updated_on: i.updated_on.with_timezone(&Utc),
                created_by_id: i.created_by_id,
                updated_by_id: i.updated_by_id,
                is_actived: i.is_actived,
            })
            .collect::<Vec<FlashcardTypeModel>>();

        return Ok(flashcard_types);
    }

    async fn create(&self, flashcard_type_req: FlashcardTypeModel) -> Result<i32, InfraError> {
        let db = self._db_connection.as_ref();

        let new_flashcard_type: flashcard_type::ActiveModel = flashcard_type::ActiveModel {
            name: Set(flashcard_type_req.name),
            description: Set(flashcard_type_req.description),
            created_by_id: Set(flashcard_type_req.created_by_id),
            updated_by_id: Set(flashcard_type_req.updated_by_id),
            created_on: Set(Utc::now().fixed_offset()),
            updated_on: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            ..Default::default()
        };

        match FlashcardType::insert(new_flashcard_type).exec(db).await {
            Ok(result) => Ok(result.last_insert_id),
            Err(err) => Err(InfraError::database(err.to_string().as_str())),
        }
    }

    async fn update(&self, flashcard_type_req: FlashcardTypeModel) -> Result<bool, InfraError> {
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
                return Err(InfraError::not_found(
                    "Flashcard file not found",
                    flashcard_type_req.id.to_string(),
                ))
            }
        };

        flashcard_type.updated_by_id = Set(flashcard_type_req.updated_by_id);
        flashcard_type.updated_on = Set(Utc::now().fixed_offset());
        flashcard_type.description = Set(flashcard_type_req.description);
        flashcard_type.name = Set(flashcard_type_req.name);
        flashcard_type.is_actived = Set(flashcard_type_req.is_actived);

        match FlashcardType::update(flashcard_type).exec(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(InfraError::database(err.to_string().as_str())),
        }
    }

    async fn delete_by_id(&self, id: i32) -> Result<u64, InfraError> {
        let db = self._db_connection.as_ref();
        FlashcardTypeRelation::delete_many()
            .filter(flashcard_type_relation::Column::FlashcardTypeId.eq(id))
            .exec(db)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;
        match FlashcardType::delete_by_id(id).exec(db).await {
            Ok(result) => Ok(result.rows_affected),
            Err(err) => Err(InfraError::database(err.to_string().as_str())),
        }
    }
}
