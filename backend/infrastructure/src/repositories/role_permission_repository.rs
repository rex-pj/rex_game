use crate::entities::{
    permission, role,
    role_permission::{self, Entity as RolePermission},
};
use chrono::{DateTime, Utc};
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::role_permission_model::RolePermissionModel,
    repositories::role_permission_repository_trait::RolePermissionRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult, JoinType,
    QueryFilter, QuerySelect, RelationTrait, Set,
};
use std::{collections::HashSet, future::Future, pin::Pin, sync::Arc};

#[derive(FromQueryResult)]
struct RolePermissionWithUser {
    pub id: i32,
    pub role_id: i32,
    pub role_name: String,
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
pub struct RolePermissionRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl RolePermissionRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl RolePermissionRepositoryTrait for RolePermissionRepository {
    async fn create(&self, role_permission_req: RolePermissionModel) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();
        let role_permission = role_permission::ActiveModel {
            role_id: Set(role_permission_req.role_id),
            permission_id: Set(role_permission_req.permission_id),
            created_by_id: Set(role_permission_req.created_by_id),
            updated_by_id: Set(role_permission_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            ..Default::default()
        };

        match RolePermission::insert(role_permission).exec(db).await {
            Ok(result) => Ok(result.last_insert_id),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    fn get_list_by_role_id(
        &self,
        role_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionModel>, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = RolePermission::find()
                .filter(role_permission::Column::RoleId.eq(role_id))
                .join(JoinType::InnerJoin, role_permission::Relation::Role.def())
                .join(
                    JoinType::InnerJoin,
                    role_permission::Relation::Permission.def(),
                )
                .select_only()
                .column_as(role::Column::Name, "role_name")
                .column_as(permission::Column::Code, "permission_code")
                .column_as(permission::Column::Name, "permission_name")
                .column_as(permission::Column::Module, "permission_module")
                .columns([
                    role_permission::Column::Id,
                    role_permission::Column::RoleId,
                    role_permission::Column::PermissionId,
                    role_permission::Column::CreatedDate,
                    role_permission::Column::CreatedById,
                    role_permission::Column::UpdatedDate,
                    role_permission::Column::UpdatedById,
                    role_permission::Column::IsActived,
                ])
                .into_model::<RolePermissionWithUser>()
                .all(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;

            let permissions = existing
                .into_iter()
                .map(|i| self::map_entity_to_model(i))
                .collect::<Vec<RolePermissionModel>>();

            Ok(permissions)
        })
    }

    fn get_list_by_role_ids(
        &self,
        role_ids: Vec<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionModel>, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = RolePermission::find()
                .filter(role_permission::Column::RoleId.is_in(role_ids))
                .join(JoinType::InnerJoin, role_permission::Relation::Role.def())
                .join(
                    JoinType::InnerJoin,
                    role_permission::Relation::Permission.def(),
                )
                .select_only()
                .column_as(role::Column::Name, "role_name")
                .column_as(permission::Column::Code, "permission_code")
                .column_as(permission::Column::Name, "permission_name")
                .column_as(permission::Column::Module, "permission_module")
                .columns([
                    role_permission::Column::Id,
                    role_permission::Column::RoleId,
                    role_permission::Column::PermissionId,
                    role_permission::Column::CreatedDate,
                    role_permission::Column::CreatedById,
                    role_permission::Column::UpdatedDate,
                    role_permission::Column::UpdatedById,
                    role_permission::Column::IsActived,
                ])
                .into_model::<RolePermissionWithUser>()
                .all(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;

            let permissions = existing
                .into_iter()
                .map(|i| self::map_entity_to_model(i))
                .collect::<Vec<RolePermissionModel>>();

            Ok(permissions)
        })
    }

    fn get_list(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionModel>, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = RolePermission::find()
                .join(JoinType::InnerJoin, role_permission::Relation::Role.def())
                .join(
                    JoinType::InnerJoin,
                    role_permission::Relation::Permission.def(),
                )
                .select_only()
                .column_as(role::Column::Name, "role_name")
                .column_as(permission::Column::Code, "permission_code")
                .column_as(permission::Column::Name, "permission_name")
                .column_as(permission::Column::Module, "permission_module")
                .columns([
                    role_permission::Column::Id,
                    role_permission::Column::RoleId,
                    role_permission::Column::PermissionId,
                    role_permission::Column::CreatedDate,
                    role_permission::Column::CreatedById,
                    role_permission::Column::UpdatedDate,
                    role_permission::Column::UpdatedById,
                    role_permission::Column::IsActived,
                ])
                .into_model::<RolePermissionWithUser>()
                .all(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;

            let permissions = existing
                .into_iter()
                .map(|i| self::map_entity_to_model(i))
                .collect::<Vec<RolePermissionModel>>();

            Ok(permissions)
        })
    }

    fn are_roles_in_permission(
        &self,
        role_ids: Vec<i32>,
        permission_codes: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        let codes = permission_codes.into_iter().collect::<Vec<String>>();

        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = RolePermission::find()
                .filter(role_permission::Column::RoleId.is_in(role_ids))
                .join(
                    JoinType::InnerJoin,
                    role_permission::Relation::Permission.def(),
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
        role_permission_req: Vec<RolePermissionModel>,
    ) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();

        let role_permissions = role_permission_req
            .into_iter()
            .map(|req| role_permission::ActiveModel {
                role_id: Set(req.role_id),
                permission_id: Set(req.permission_id),
                created_by_id: Set(req.created_by_id),
                updated_by_id: Set(req.updated_by_id),
                created_date: Set(Utc::now().fixed_offset()),
                updated_date: Set(Utc::now().fixed_offset()),
                is_actived: Set(true),
                ..Default::default()
            })
            .collect::<Vec<role_permission::ActiveModel>>();
        match RolePermission::insert_many(role_permissions).exec(db).await {
            Ok(result) => Ok(result.last_insert_id),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    async fn delete_many(
        &self,
        role_id: i32,
        role_permission_req: Vec<RolePermissionModel>,
    ) -> Result<u64, DomainError> {
        let db = self._db_connection.as_ref();

        let delete_permission_ids = role_permission_req
            .into_iter()
            .map(|f| f.permission_id)
            .collect::<Vec<i32>>();
        match RolePermission::delete_many()
            .filter(
                Condition::all()
                    .add(role_permission::Column::PermissionId.is_in(delete_permission_ids))
                    .add(role_permission::Column::RoleId.eq(role_id)),
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

fn map_entity_to_model(entity: RolePermissionWithUser) -> RolePermissionModel {
    RolePermissionModel {
        id: entity.id,
        role_id: entity.role_id,
        permission_id: entity.permission_id,
        role_name: entity.role_name,
        permission_name: entity.permission_name,
        permission_code: entity.permission_code,
        permission_module: entity.permission_module,
        created_date: entity.created_date.with_timezone(&Utc),
        updated_date: entity.updated_date.with_timezone(&Utc),
        created_by_id: entity.created_by_id,
        updated_by_id: entity.updated_by_id,
        is_actived: entity.is_actived,
    }
}
