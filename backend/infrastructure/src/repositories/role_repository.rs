use crate::entities::role::{self, Entity as Role};
use chrono::Utc;
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::{page_list_model::PageListModel, role_model::RoleModel},
    repositories::role_repository_trait::RoleRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct RoleRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl RoleRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl RoleRepositoryTrait for RoleRepository {
    async fn create(&self, role_req: RoleModel) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();

        let role = role::ActiveModel {
            name: Set(role_req.name),
            description: Set(role_req.description),
            created_by_id: Set(role_req.created_by_id),
            updated_by_id: Set(role_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            ..Default::default()
        };

        let inserted = Role::insert(role).exec(db).await.map_err(|err| {
            DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
        });

        match inserted {
            Ok(updated) => {
                return Ok(updated.last_insert_id);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    async fn get_by_name(&self, name: &str) -> Result<RoleModel, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = Role::find()
            .filter(Condition::all().add(role::Column::Name.eq(name)))
            .one(db)
            .await
            .map_err(|err| {
                DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
            })?;

        match existing {
            Some(f) => Ok(RoleModel {
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
                "Role not found",
                None,
            )),
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<RoleModel, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = Role::find_by_id(id).one(db).await.map_err(|err| {
            DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
        })?;

        match existing {
            Some(f) => Ok(RoleModel {
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
                "Role not found",
                None,
            )),
        }
    }

    async fn get_paged_list(
        &self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Result<PageListModel<RoleModel>, DomainError> {
        let db = self._db_connection.as_ref();
        let mut query = Role::find();

        if let Some(d) = name {
            query = query.filter(role::Column::Name.eq(d));
        }

        if let Some(n) = description {
            query = query.filter(role::Column::Description.eq(n));
        }

        query = query.order_by(role::Column::UpdatedDate, sea_orm::Order::Desc);

        let paginator = query.paginate(db, page_size);

        let total_count = match paginator.num_items().await {
            Ok(count) => count,
            Err(err) => {
                return Err(DomainError::new(
                    ErrorType::DatabaseError,
                    err.to_string().as_str(),
                    None,
                ))
            }
        };

        let page_list = paginator.fetch_page(page - 1).await;
        match page_list {
            Ok(items) => {
                let list = items
                    .into_iter()
                    .map(|i| RoleModel {
                        id: i.id,
                        name: i.name,
                        description: i.description,
                        created_date: i.created_date.with_timezone(&Utc),
                        updated_date: i.updated_date.with_timezone(&Utc),
                        created_by_id: i.created_by_id,
                        updated_by_id: i.updated_by_id,
                        is_actived: i.is_actived,
                    })
                    .collect::<Vec<RoleModel>>();
                return Ok(PageListModel {
                    items: list,
                    total_count,
                });
            }
            Err(err) => {
                return Err(DomainError::new(
                    ErrorType::DatabaseError,
                    err.to_string().as_str(),
                    None,
                ))
            }
        }
    }

    async fn update(&self, role_req: RoleModel) -> Result<bool, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = Role::find_by_id(role_req.id).one(db).await;
        let role_option = match existing {
            Ok(f) => f,
            Err(_) => None,
        };

        let mut role: role::ActiveModel = match role_option {
            Some(f) => f.into(),
            None => {
                return Err(DomainError::new(
                    ErrorType::NotFound,
                    "Flashcard file not found",
                    None,
                ))
            }
        };

        role.updated_by_id = Set(role_req.updated_by_id);
        role.updated_date = Set(Utc::now().fixed_offset());
        role.description = Set(role_req.description);
        role.name = Set(role_req.name);

        match Role::update(role).exec(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }
}
