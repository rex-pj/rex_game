use crate::entities::{
    permission, user,
    user_permission::{self, Entity as UserPermission},
};
use chrono::{DateTime, Utc};
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::user_permission_model::UserPermissionModel,
    repositories::user_permission_repository_trait::UserPermissionRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult, JoinType,
    QueryFilter, QuerySelect, RelationTrait, Set,
};
use std::{collections::HashSet, future::Future, pin::Pin, sync::Arc};

#[derive(FromQueryResult)]
struct UserPermissionWithUser {
    pub id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub permission_id: i32,
    pub permission_name: String,
    pub permission_code: String,
    pub permission_module: String,
    pub created_by_id: i32,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub updated_by_id: i32,
    pub is_actived: bool,
}

#[derive(Clone)]
pub struct UserPermissionRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl UserPermissionRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl UserPermissionRepositoryTrait for UserPermissionRepository {
    async fn create(&self, user_permission_req: UserPermissionModel) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();
        let user_permission = user_permission::ActiveModel {
            user_id: Set(user_permission_req.user_id),
            permission_id: Set(user_permission_req.permission_id),
            created_by_id: Set(user_permission_req.created_by_id),
            updated_by_id: Set(user_permission_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            ..Default::default()
        };

        match UserPermission::insert(user_permission).exec(db).await {
            Ok(result) => Ok(result.last_insert_id),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    fn get_list_by_user_id(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserPermissionModel>, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = UserPermission::find()
                .filter(user_permission::Column::UserId.eq(user_id))
                .join(JoinType::InnerJoin, user_permission::Relation::User1.def())
                .join(
                    JoinType::InnerJoin,
                    user_permission::Relation::Permission.def(),
                )
                .select_only()
                .column_as(user::Column::Name, "user_name")
                .column_as(permission::Column::Code, "permission_code")
                .column_as(permission::Column::Name, "permission_name")
                .column_as(permission::Column::Module, "permission_module")
                .columns([
                    user_permission::Column::Id,
                    user_permission::Column::PermissionId,
                    user_permission::Column::CreatedDate,
                    user_permission::Column::CreatedById,
                    user_permission::Column::UpdatedDate,
                    user_permission::Column::UpdatedById,
                    user_permission::Column::IsActived,
                    user_permission::Column::UserId,
                ])
                .into_model::<UserPermissionWithUser>()
                .all(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;

            let permissions = existing
                .into_iter()
                .map(|i| self::map_entity_to_model(i))
                .collect::<Vec<UserPermissionModel>>();

            Ok(permissions)
        })
    }

    fn get_list(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserPermissionModel>, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = UserPermission::find()
                .join(JoinType::InnerJoin, user_permission::Relation::User1.def())
                .join(
                    JoinType::InnerJoin,
                    user_permission::Relation::Permission.def(),
                )
                .select_only()
                .column_as(user::Column::Name, "user_name")
                .column_as(permission::Column::Code, "permission_code")
                .column_as(permission::Column::Name, "permission_name")
                .column_as(permission::Column::Module, "permission_module")
                .columns([
                    user_permission::Column::Id,
                    user_permission::Column::PermissionId,
                    user_permission::Column::CreatedDate,
                    user_permission::Column::CreatedById,
                    user_permission::Column::UpdatedDate,
                    user_permission::Column::UpdatedById,
                    user_permission::Column::IsActived,
                    user_permission::Column::UserId,
                ])
                .into_model::<UserPermissionWithUser>()
                .all(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;

            let permissions = existing
                .into_iter()
                .map(|i| self::map_entity_to_model(i))
                .collect::<Vec<UserPermissionModel>>();

            Ok(permissions)
        })
    }

    fn is_user_in_permission(
        &self,
        user_id: i32,
        permission_codes: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        let codes = permission_codes.into_iter().collect::<Vec<String>>();

        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = UserPermission::find()
                .filter(user_permission::Column::UserId.eq(user_id))
                .join(
                    JoinType::InnerJoin,
                    user_permission::Relation::Permission.def(),
                )
                .filter(Condition::all().add(permission::Column::Code.is_in(codes)))
                .one(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                });

            match existing {
                Ok(permissions) => Ok(permissions.is_some()),
                Err(err) => return Err(err),
            }
        })
    }

    async fn create_many(
        &self,
        user_permission_req: Vec<UserPermissionModel>,
    ) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();

        let user_permissions = user_permission_req
            .into_iter()
            .map(|req| user_permission::ActiveModel {
                user_id: Set(req.user_id),
                permission_id: Set(req.permission_id),
                created_by_id: Set(req.created_by_id),
                updated_by_id: Set(req.updated_by_id),
                created_date: Set(Utc::now().fixed_offset()),
                updated_date: Set(Utc::now().fixed_offset()),
                is_actived: Set(true),
                ..Default::default()
            })
            .collect::<Vec<user_permission::ActiveModel>>();
        match UserPermission::insert_many(user_permissions).exec(db).await {
            Ok(result) => match result.last_insert_id {
                Some(id) => Ok(id),
                None => Ok(0), // insert_many may return None if empty
            },
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    async fn delete_many(
        &self,
        user_id: i32,
        user_permission_req: Vec<UserPermissionModel>,
    ) -> Result<u64, DomainError> {
        let db = self._db_connection.as_ref();

        let delete_permission_ids = user_permission_req
            .into_iter()
            .map(|f| f.permission_id)
            .collect::<Vec<i32>>();
        match UserPermission::delete_many()
            .filter(
                Condition::all()
                    .add(user_permission::Column::PermissionId.is_in(delete_permission_ids))
                    .add(user_permission::Column::UserId.eq(user_id)),
            )
            .exec(db)
            .await
        {
            Ok(result) => Ok(result.rows_affected),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }
}

fn map_entity_to_model(permission: UserPermissionWithUser) -> UserPermissionModel {
    UserPermissionModel {
        id: permission.id,
        permission_id: permission.permission_id,
        user_id: permission.user_id,
        permission_name: permission.permission_name,
        permission_code: permission.permission_code,
        permission_module: permission.permission_module,
        created_date: permission.created_date.with_timezone(&Utc),
        updated_date: permission.updated_date.with_timezone(&Utc),
        created_by_id: permission.created_by_id,
        updated_by_id: permission.updated_by_id,
        is_actived: permission.is_actived,
        user_name: permission.user_name,
    }
}
