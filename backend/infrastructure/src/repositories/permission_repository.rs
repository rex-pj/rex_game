use crate::entities::permission::{self, Entity as Permission};
use chrono::Utc;
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::{page_list_model::PageListModel, permission_model::PermissionModel},
    repositories::permission_repository_trait::PermissionRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct PermissionRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl PermissionRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl PermissionRepositoryTrait for PermissionRepository {
    async fn create(&self, permission_req: PermissionModel) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();

        let permission = permission::ActiveModel {
            name: Set(permission_req.name),
            description: Set(permission_req.description),
            code: Set(permission_req.code),
            module: Set(permission_req.module),
            created_by_id: Set(permission_req.created_by_id),
            updated_by_id: Set(permission_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            ..Default::default()
        };

        let inserted = Permission::insert(permission)
            .exec(db)
            .await
            .map_err(|err| {
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

    async fn get_by_code(&self, code: &str) -> Result<PermissionModel, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = Permission::find()
            .filter(Condition::all().add(permission::Column::Code.eq(code)))
            .one(db)
            .await
            .map_err(|err| {
                DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
            })?;

        match existing {
            Some(f) => Ok(PermissionModel {
                id: f.id,
                name: f.name,
                code: f.code,
                module: f.module,
                description: f.description,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                created_by_id: f.created_by_id,
                updated_by_id: f.updated_by_id,
                is_actived: f.is_actived,
            }),
            None => Err(DomainError::new(
                ErrorType::NotFound,
                "Permission not found",
                None,
            )),
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<PermissionModel, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = Permission::find_by_id(id).one(db).await.map_err(|err| {
            DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
        })?;

        match existing {
            Some(f) => Ok(PermissionModel {
                id: f.id,
                name: f.name,
                description: f.description,
                code: f.code,
                module: f.module,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                created_by_id: f.created_by_id,
                updated_by_id: f.updated_by_id,
                is_actived: f.is_actived,
            }),
            None => Err(DomainError::new(
                ErrorType::NotFound,
                "Permission not found",
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
    ) -> Result<PageListModel<PermissionModel>, DomainError> {
        let db = self._db_connection.as_ref();
        let mut query = Permission::find();

        if let Some(d) = name {
            query = query.filter(permission::Column::Name.eq(d));
        }

        if let Some(n) = description {
            query = query.filter(permission::Column::Description.eq(n));
        }

        query = query.order_by(permission::Column::UpdatedDate, sea_orm::Order::Desc);

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
                    .map(|i| PermissionModel {
                        id: i.id,
                        name: i.name,
                        description: i.description,
                        code: i.code,
                        module: i.module,
                        created_date: i.created_date.with_timezone(&Utc),
                        updated_date: i.updated_date.with_timezone(&Utc),
                        created_by_id: i.created_by_id,
                        updated_by_id: i.updated_by_id,
                        is_actived: i.is_actived,
                    })
                    .collect::<Vec<PermissionModel>>();
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

    async fn update(&self, permission_req: PermissionModel) -> Result<bool, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = Permission::find_by_id(permission_req.id).one(db).await;
        let permission_option = match existing {
            Ok(f) => f,
            Err(_) => None,
        };

        let mut permission: permission::ActiveModel = match permission_option {
            Some(f) => f.into(),
            None => {
                return Err(DomainError::new(
                    ErrorType::NotFound,
                    "Flashcard file not found",
                    None,
                ))
            }
        };

        permission.updated_by_id = Set(permission_req.updated_by_id);
        permission.description = Set(permission_req.description);
        permission.name = Set(permission_req.name);
        permission.code = Set(permission_req.code);
        permission.module = Set(permission_req.module);
        permission.is_actived = Set(permission_req.is_actived);
        permission.updated_date = Set(Utc::now().fixed_offset());

        match Permission::update(permission).exec(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }
}
