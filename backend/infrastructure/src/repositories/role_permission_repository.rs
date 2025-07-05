use crate::entities::{
    permission,
    role_permission::{self, Entity as RolePermission},
};
use chrono::Utc;
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::role_permission_model::RolePermissionModel,
    repositories::role_permission_repository_trait::RolePermissionRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect,
    RelationTrait, Set,
};
use std::{collections::HashSet, future::Future, pin::Pin, sync::Arc};

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

    fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionModel>, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = RolePermission::find()
                .filter(role_permission::Column::RoleId.eq(role_id))
                .find_with_related(permission::Entity)
                .all(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;

            let permissions = existing
                .into_iter()
                .map(|i| {
                    let permission_name = match i.1.first() {
                        Some(permission) => permission.name.to_owned(),
                        None => String::from(""),
                    };

                    let permission_code = match i.1.first() {
                        Some(permission) => permission.code.to_owned(),
                        None => String::from(""),
                    };

                    let permission_module = match i.1.first() {
                        Some(permission) => permission.module.to_owned(),
                        None => String::from(""),
                    };
                    return RolePermissionModel {
                        id: i.0.id,
                        permission_id: i.0.permission_id,
                        role_id: i.0.role_id,
                        permission_name: permission_name,
                        permission_code: permission_code,
                        permission_module: permission_module,
                        created_date: i.0.created_date.with_timezone(&Utc),
                        updated_date: i.0.updated_date.with_timezone(&Utc),
                        created_by_id: i.0.created_by_id,
                        updated_by_id: i.0.updated_by_id,
                        is_actived: i.0.is_actived,
                    };
                })
                .collect::<Vec<RolePermissionModel>>();

            Ok(permissions)
        })
    }

    fn get_roles_permissions(
        &self,
        role_ids: Vec<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionModel>, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = RolePermission::find()
                .filter(role_permission::Column::RoleId.is_in(role_ids))
                .find_with_related(permission::Entity)
                .all(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;

            let permissions = existing
                .into_iter()
                .map(|i| {
                    let permission_name = match i.1.first() {
                        Some(permission) => permission.name.to_owned(),
                        None => String::from(""),
                    };

                    let permission_code = match i.1.first() {
                        Some(permission) => permission.code.to_owned(),
                        None => String::from(""),
                    };

                    let permission_module = match i.1.first() {
                        Some(permission) => permission.module.to_owned(),
                        None => String::from(""),
                    };
                    return RolePermissionModel {
                        id: i.0.id,
                        permission_id: i.0.permission_id,
                        role_id: i.0.role_id,
                        permission_name: permission_name,
                        permission_code: permission_code,
                        permission_module: permission_module,
                        created_date: i.0.created_date.with_timezone(&Utc),
                        updated_date: i.0.updated_date.with_timezone(&Utc),
                        created_by_id: i.0.created_by_id,
                        updated_by_id: i.0.updated_by_id,
                        is_actived: i.0.is_actived,
                    };
                })
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
}
